use super::super::binary;
use super::parse::{ANType, ANode, ATType, ATVal, AToken};
use binary::bytes::Bin;
enum Prefix {
    W,
    NOTHING,
}
impl Prefix {
    fn bin(&self) -> u8 {
        let mut byte: u8 = 0x0;
        byte = 0x40;
        match self {
            Prefix::NOTHING => 0x00,
            Prefix::W => byte | 0x08,
        }
    }
}
enum Opcode {
    One(u8),
    Else(Vec<u8>),
}
impl Opcode {
    fn bin(&self) -> Vec<u8> {
        match self {
            Opcode::One(b) => vec![*b],
            Opcode::Else(bts) => bts.to_vec(),
        }
    }
}
enum ModRM {
    JUST(u8),
    NOTHING,
}
impl ModRM {
    fn bin(&self) -> u8 {
        match self {
            ModRM::JUST(b) => *b,
            ModRM::NOTHING => 0x00,
        }
    }
    fn number(reg: &String) -> u8 {
        match reg.as_str() {
            "rax" => 0,
            "rcx" => 1,
            "rdx" => 2,
            "rbx" => 3,
            "rsp" => 4,
            "rbp" => 5,
            "rsi" => 6,
            "rdi" => 7,
            _ => 8,
        }
    }
    fn check(lop: &ANode, rop: &ANode) -> ModRM {
        let mut modrm: u8 = 0xc0; //mod->11
        if let ANType::REG(t) = &lop.ty {
            modrm |= ModRM::number(&t.literal) >> 4;
        }
        if let ANType::REG(t) = &rop.ty {
            modrm |= ModRM::number(&t.literal);
        }
        ModRM::JUST(modrm)
    }
}
enum Immediate {
    JUST(u32),
    NOTHING,
}
impl Immediate {
    fn new(op: &ANode) -> Immediate {
        if let ANType::INT(t) = &op.ty {
            if let ATVal::IntVal(i) = t.val {
                return Immediate::JUST(i as u32);
            }
        }
        Immediate::NOTHING
    }
}
pub fn generate(nodes: Vec<ANode>) -> Vec<u8> {
    let mut bin: Bin = Bin::new((vec![], true));
    for n in nodes {
        bin.write(&gen_stmt(n));
    }
    bin.b.get_ref().to_vec()
}
fn gen_stmt(node: ANode) -> Vec<u8> {
    match node.ty {
        ANType::BININST(ty, lop, rop) => gen_bininst(ty, lop, rop),
        //ANType::ONEINST(ty,lop),
        ANType::NINST(ty) => gen_ninst(ty),
        _ => vec![],
    }
}
fn gen_bininst(ty: ATType, lop: Vec<ANode>, rop: Vec<ANode>) -> Vec<u8> {
    let prefix: Prefix = Prefix::W;
    let code: Opcode = gen_opcode(ty, &lop[0]);
    let modrm: ModRM = ModRM::check(&lop[0], &rop[0]);
    let immediate: Immediate = Immediate::new(&rop[0]);
    let mut bytes: Vec<u8> = Vec::new();
    bytes.push(prefix.bin());
    for b in code.bin() {
        bytes.push(b);
    }
    bytes.push(modrm.bin());
    if let Immediate::JUST(b) = immediate {
        for bb in b.to_le_bytes().to_vec() {
            bytes.push(bb);
        }
    }
    bytes
}
fn gen_ninst(ty: ATType) -> Vec<u8> {
    match ty {
        ATType::ARet => vec![0xC3],
        _ => vec![],
    }
}
fn gen_opcode(ty: ATType, lop: &ANode) -> Opcode {
    let mut b: u8 = 0x00;
    match ty {
        ATType::AMov => {
            b |= 0xb8;
            Opcode::One(b) // rax
        }
        _ => Opcode::One(0x00),
    }
}
