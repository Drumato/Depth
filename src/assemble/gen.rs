use super::super::object::elf::elf64::Rela;
use super::parse::{Info, Inst, Operand};
use std::collections::BTreeMap;
use std::ops::Deref;
type LabelName = String;
type CodeIndex = usize;
type Offset = usize;
struct Generator {
    insts: Vec<Inst>,
    info_map: BTreeMap<usize, Info>,
    jump_map: BTreeMap<LabelName, (CodeIndex, Offset)>,
    codes: Vec<u8>,
    rels_map: BTreeMap<String, Rela>,
    symbol_map: BTreeMap<String, Vec<u8>>,
    offset: u64,
}
impl Generator {
    fn gen(&mut self) {
        let insts: Vec<Inst> = self.insts.to_vec();
        for inst in insts.iter() {
            match inst {
                &Inst::BINARG(num) | &Inst::UNARG(num) | &Inst::NOARG(num) => {
                    self.gen_inst(&num);
                }
                Inst::LABEL(_, name) => {
                    if let Some(tup) = self.jump_map.get_mut(name) {
                        tup.1 = self.codes.len() - tup.1;
                        continue;
                    }
                    self.jump_map
                        .insert(name.to_string(), (0, self.codes.len()));
                }
            }
        }
        for inst in insts.iter() {
            if let Inst::UNARG(num) = inst {
                let info: &Info = self.info_map.get(&num).unwrap();
                match info.inst_name.as_str() {
                    "jmp" => {
                        if let Some(Operand::SYMBOL(name)) = &info.lop {
                            if let Some(tup) = self.jump_map.get(name) {
                                self.codes[tup.0] = tup.1 as u8 - 1;
                            }
                        }
                    }
                    "jz" => {
                        if let Some(Operand::SYMBOL(name)) = &info.lop {
                            if let Some(tup) = self.jump_map.get(name) {
                                self.codes[tup.0] = tup.1 as u8 - 1;
                            }
                        }
                    }
                    _ => (),
                }
            }
        }
    }
    fn gen_inst(&mut self, num: &usize) {
        let info: &Info = self.info_map.get(&num).unwrap();
        match info.inst_name.as_str() {
            "add" => {
                if let Some(Operand::REG(_reg)) = &info.lop {
                    self.codes.push(self.set_rexprefix(&info.lop, &info.rop));
                    let modrm: u8 = self.set_modrm(&info.lop, &info.rop); // mod field of ModR/M
                    if let Some(Operand::ADDRESS(_content, offset)) = &info.rop {
                        self.codes.push(0x03);
                        self.codes.push(modrm); // ModR/M with MR
                        self.codes.push(*offset as u8);
                    } else if let Some(Operand::IMM(value)) = info.rop {
                        self.codes.push(0x81);
                        self.codes.push(modrm); // ModR/M with MR
                        self.gen_immediate(value);
                    } else {
                        self.codes.push(0x01);
                        self.codes.push(modrm);
                    }
                }
            }
            "call" => {
                self.codes.push(0x48);
                self.codes.push(0xc7);
                self.codes.push(0xc0);
                if let Some(Operand::SYMBOL(name)) = &info.lop {
                    if let Some(rela) = self.rels_map.get_mut(name) {
                        rela.r_offset = self.offset + self.codes.len() as u64;
                    }
                    if let None = self.symbol_map.get(name) {
                        self.symbol_map.insert(name.to_string(), Vec::new());
                    }
                }
                self.gen_immediate(0x00);
                self.codes.push(0xff);
                self.codes.push(0xd0);
            }
            "cmp" => {
                self.codes.push(self.set_rexprefix(&info.lop, &info.rop));
                if let Some(Operand::REG(_reg)) = &info.lop {
                    if let Some(Operand::IMM(value)) = info.rop {
                        self.codes.push(0x83);
                        self.codes.push(self.set_modrm(&info.rop, &info.lop) | 0x38); // ModR/M with MR
                        self.codes.push(value as u8)
                    } else if let Some(Operand::ADDRESS(_content, offset)) = &info.rop {
                        self.codes.push(0x3b);
                        self.codes.push(self.set_modrm(&info.rop, &info.lop)); // ModR/M with MR
                        self.codes.push(*offset as u8);
                    } else {
                        self.codes.push(0x3b);
                        self.codes.push(self.set_modrm(&info.rop, &info.lop)); // ModR/M with MR
                    }
                } else if let Some(Operand::ADDRESS(_content, offset)) = &info.lop {
                    if let Some(Operand::IMM(value)) = info.rop {
                        let modrm: u8 = self.set_modrm(&info.lop, &info.rop); // with MR
                        self.codes.push(0x3b); // mp r/m64, imm32
                        self.codes.push(modrm);
                        self.codes.push(*offset as u8);
                        self.gen_immediate(value);
                    }
                }
            }
            "cqo" => {
                self.codes.push(0x48);
                self.codes.push(0x99);
            }
            "idiv" => {
                self.codes.push(0x49);
                self.codes.push(0xf7);
                let mut modrm: u8 = 0xf8;
                if let Some(reg) = &info.lop {
                    modrm |= reg.reg_number();
                }
                self.codes.push(modrm);
            }
            "imul" => {
                if let Some(Operand::REG(_reg)) = &info.lop {
                    self.codes.push(self.set_rexprefix(&info.lop, &info.rop));
                    let modrm: u8 = self.set_modrm(&info.lop, &info.rop); // mod field of ModR/M
                    if let Some(Operand::ADDRESS(_content, offset)) = &info.rop {
                        self.codes.push(0x0f);
                        self.codes.push(0xaf);
                        self.codes.push(modrm);
                        self.codes.push(*offset as u8);
                    } else if let Some(Operand::IMM(value)) = info.rop {
                        self.codes.push(0x69);
                        self.codes.push(modrm);
                        self.gen_immediate(value);
                    } else {
                    }
                }
            }
            "jmp" => {
                self.codes.push(0xeb);
                if let Some(Operand::SYMBOL(name)) = &info.lop {
                    if let Some(tup) = self.jump_map.get_mut(name) {
                        tup.0 = self.codes.len();
                        tup.1 = !(self.codes.len() - tup.1) + 1;
                    } else {
                        self.jump_map
                            .insert(name.to_string(), (self.codes.len(), self.codes.len()));
                    }
                }
                self.codes.push(0x00);
            }
            "jz" => {
                self.codes.push(0x74);
                if let Some(Operand::SYMBOL(name)) = &info.lop {
                    if let Some(tup) = self.jump_map.get_mut(name) {
                        tup.0 = self.codes.len();
                        tup.1 = !(self.codes.len() - tup.1) + 1;
                    } else {
                        self.jump_map
                            .insert(name.to_string(), (self.codes.len(), self.codes.len()));
                    }
                }
                self.codes.push(0x00);
            }
            "lea" => {
                self.codes.push(self.set_rexprefix(&info.lop, &info.rop));
                match &info.lop {
                    Some(Operand::REG(_reg)) => {
                        if let Some(Operand::ADDRESS(_content, offset)) = &info.rop {
                            let modrm: u8 = self.set_modrm(&info.lop, &info.rop); // with MR
                            self.codes.push(0x8d); // lea r64, r/m64
                            self.codes.push(modrm);
                            self.codes.push(*offset as u8);
                        }
                    }
                    _ => (),
                }
            }
            "mov" => {
                self.codes.push(self.set_rexprefix(&info.lop, &info.rop));
                match &info.lop {
                    Some(Operand::REG(_reg)) => {
                        if let Some(Operand::IMM(value)) = info.rop {
                            let modrm: u8 = self.set_modrm(&info.lop, &info.rop); // with RM
                            self.codes.push(0xc7); // mov reg, immediate
                            self.codes.push(modrm);
                            self.gen_immediate(value);
                        } else if let Some(Operand::ADDRESS(_content, offset)) = &info.rop {
                            let modrm: u8 = self.set_modrm(&info.lop, &info.rop); // with MR
                            self.codes.push(0x8b); // mov r64, r/m64
                            self.codes.push(modrm);
                            self.codes.push(*offset as u8);
                        } else if let Some(Operand::ELEMENT(base, idx, scale, offset)) = &info.rop {
                            //44 e5 e8
                            let modrm: u8 = self.set_modrm(&info.lop, &info.rop);
                            self.codes.push(0x8b); // mov r64, r/m64
                            self.codes.push(modrm);
                            self.codes
                                .push(self.set_sib_byte(base.deref(), idx.deref(), *scale));
                            self.codes.push(*offset as u8);
                        } else {
                            let modrm: u8 = self.set_modrm(&info.lop, &info.rop); // with RM
                            self.codes.push(0x89); // mov reg, reg
                            self.codes.push(modrm);
                        }
                    }
                    Some(Operand::ADDRESS(_content, offset)) => {
                        if let Some(Operand::REG(_reg)) = &info.rop {
                            self.codes.push(0x89); // mov r/m64, r64
                            self.codes.push(self.set_modrm(&info.lop, &info.rop)); // with RM
                            self.codes.push(*offset as u8);
                        } else if let Some(Operand::IMM(value)) = info.rop {
                            self.codes.push(0xc7);
                            self.codes.push(self.set_modrm(&info.lop, &info.rop));
                            self.codes.push(*offset as u8);
                            self.gen_immediate(value);
                        }
                    }
                    _ => (),
                }
            }
            "movzx" => {
                self.codes.push(self.set_rexprefix(&info.rop, &info.lop));
                self.codes.push(0x0f);
                self.codes.push(0xb6);
                self.codes.push(self.set_modrm(&info.rop, &info.lop)); // with MR
            }
            "neg" => {
                self.codes.push(self.set_rexprefix(&info.lop, &info.rop));
                self.codes.push(0xf7);
                let modrm: u8 = self.set_modrm(&info.lop, &info.rop);
                self.codes.push(modrm | 0x18);
            }
            "push" => {
                let mut opcode: u8 = 0x50;
                if let Some(reg) = &info.lop {
                    if let Operand::REG(name) = reg {
                        match name.as_str() {
                            "r8" | "r9" | "r10" | "r11" | "r12" | "r13" | "r14" | "r15" => {
                                self.codes.push(0x41);
                            }
                            _ => (),
                        }
                    }
                    opcode |= reg.reg_number();
                }
                self.codes.push(opcode);
            }
            "pop" => {
                let mut opcode: u8 = 0x58;
                if let Some(reg) = &info.lop {
                    if let Operand::REG(name) = reg {
                        match name.as_str() {
                            "r8" | "r9" | "r10" | "r11" | "r12" | "r13" | "r14" | "r15" => {
                                self.codes.push(0x41);
                            }
                            _ => (),
                        }
                    }
                    opcode |= reg.reg_number();
                }
                self.codes.push(opcode);
            }
            "ret" => {
                self.codes.push(0xc3);
            }
            "setl" => {
                self.codes.push(0x0f);
                self.codes.push(0x9c);
                self.codes.push(self.set_modrm(&info.lop, &info.rop));
            }
            "setle" => {
                self.codes.push(0x0f);
                self.codes.push(0x9e);
                self.codes.push(self.set_modrm(&info.lop, &info.rop));
            }
            "setg" => {
                self.codes.push(0x0f);
                self.codes.push(0x9f);
                self.codes.push(self.set_modrm(&info.lop, &info.rop));
            }
            "setge" => {
                self.codes.push(0x0f);
                self.codes.push(0x9d);
                self.codes.push(self.set_modrm(&info.lop, &info.rop));
            }
            "sete" => {
                self.codes.push(0x0f);
                self.codes.push(0x94);
                self.codes.push(self.set_modrm(&info.lop, &info.rop));
            }
            "setne" => {
                self.codes.push(0x0f);
                self.codes.push(0x95);
                self.codes.push(self.set_modrm(&info.lop, &info.rop));
            }
            "sar" => {
                self.codes.push(self.set_rexprefix(&info.lop, &info.rop));
                if let Some(Operand::REG(_reg)) = &info.lop {
                    if let Some(Operand::REG(r2)) = &info.rop {
                        if r2 == "cl" {
                            self.codes.push(0xd3);
                            self.codes.push(self.set_modrm(&info.lop, &info.rop));
                        }
                    } else if let Some(Operand::IMM(value)) = &info.rop {
                        self.codes.push(0xc1);
                        self.codes.push(self.set_modrm(&info.lop, &info.rop) | 0x38);
                        self.codes.push(*value as u8);
                    }
                }
            }
            "sal" => {
                if let Some(Operand::REG(_reg)) = &info.lop {
                    if let Some(Operand::REG(r2)) = &info.rop {
                        if r2 == "cl" {
                            self.codes.push(self.set_rexprefix(&info.lop, &info.rop));
                            self.codes.push(0xd3);
                            self.codes.push(self.set_modrm(&info.lop, &info.rop));
                        }
                    } else if let Some(Operand::IMM(value)) = &info.rop {
                        let modrm: u8 = self.set_modrm(&info.lop, &info.rop);
                        self.codes.push(0xc1);
                        self.codes.push(modrm);
                        self.codes.push(*value as u8);
                    }
                }
            }
            "sub" => {
                self.codes.push(self.set_rexprefix(&info.lop, &info.rop));
                if let Some(Operand::IMM(value)) = info.rop {
                    let mut modrm: u8 = self.set_modrm(&info.lop, &info.rop); // with RM
                    modrm |= 0b00101000;
                    self.codes.push(0x83);
                    self.codes.push(modrm);
                    self.codes.push(value as u8);
                } else {
                    let modrm: u8 = self.set_modrm(&info.lop, &info.rop); // with RM
                    self.codes.push(0x29);
                    self.codes.push(modrm);
                }
            }
            "syscall" => {
                self.codes.push(0x0f);
                self.codes.push(0x05);
            }
            _ => (),
        }
    }
    fn gen_immediate(&mut self, value: i128) {
        match value {
            n if n <= 255 => {
                self.codes.push(value as u8);
                self.codes.push(0x00);
                self.codes.push(0x00);
                self.codes.push(0x00);
            }
            n if n <= 65535 => {
                self.codes.push((value >> 8) as u8);
                self.codes.push(value as u8);
                self.codes.push(0x00);
                self.codes.push(0x00);
            }
            n if n <= 4294967295 => {
                self.codes.push((value >> 16) as u8);
                self.codes.push((value >> 8) as u8);
                self.codes.push(value as u8);
                self.codes.push(0x00);
            }
            _ => {
                self.codes.push((value >> 24) as u8);
                self.codes.push((value >> 16) as u8);
                self.codes.push((value >> 8) as u8);
                self.codes.push(value as u8);
            }
        }
    }
    fn set_rexprefix(&self, lop: &Option<Operand>, rop: &Option<Operand>) -> u8 {
        // 0100 | REX.w	REX.r REX.x REX.b
        let mut rexprefix: u8 = 0x40;
        match lop {
            Some(Operand::REG(name)) => {
                if name.starts_with("r") {
                    rexprefix |= 0x08;
                }
                match name.as_str() {
                    "r8" | "r9" | "r10" | "r11" | "r12" | "r13" | "r14" | "r15" => {
                        rexprefix |= 0x01;
                    }
                    _ => (),
                }
            }
            Some(Operand::ADDRESS(content, _)) => {
                if let Operand::REG(name) = content.deref() {
                    if name.starts_with("r") {
                        rexprefix |= 0x08;
                    }
                    match name.as_str() {
                        "r8" | "r9" | "r10" | "r11" | "r12" | "r13" | "r14" | "r15" => {
                            rexprefix |= 0x04;
                        }
                        _ => (),
                    }
                }
            }
            _ => (),
        }
        match rop {
            Some(Operand::REG(name)) => {
                if name.starts_with("r") {
                    rexprefix |= 0x08;
                }
                match name.as_str() {
                    "r8" | "r9" | "r10" | "r11" | "r12" | "r13" | "r14" | "r15" => {
                        rexprefix |= 0x04;
                    }
                    _ => (),
                }
            }
            Some(Operand::ADDRESS(content, _)) => {
                if let Operand::REG(name) = content.deref() {
                    if name.starts_with("r") {
                        rexprefix |= 0x08;
                    }
                    match name.as_str() {
                        "r8" | "r9" | "r10" | "r11" | "r12" | "r13" | "r14" | "r15" => {
                            rexprefix |= 0x01;
                        }
                        _ => (),
                    }
                }
            }
            Some(Operand::ELEMENT(base, _ind, _scale, _off)) => {
                if let Operand::REG(name) = base.deref() {
                    if name.starts_with("r") {
                        rexprefix |= 0x08;
                    }
                    match name.as_str() {
                        "r8" | "r9" | "r10" | "r11" | "r12" | "r13" | "r14" | "r15" => {
                            rexprefix |= 0x01;
                        }
                        _ => (),
                    }
                }
            }
            _ => (),
        }
        rexprefix
    }

    fn set_sib_byte(&self, base: &Operand, idx: &Operand, _scale: i128) -> u8 {
        let mut sib: u8 = 0xc0;
        if let Operand::REG(name) = base {
            sib |= Operand::number(name);
        }
        if let Operand::REG(name) = idx {
            sib |= Operand::number(name) << 3;
        }
        sib
    }
    fn set_modrm(&self, lop: &Option<Operand>, rop: &Option<Operand>) -> u8 {
        // mod(2 bits) | reg(3 bits) | r/m(3 bits)
        let mut modrm: u8 = 0xc0; // the mod filed of modr/m
        match lop {
            Some(Operand::ADDRESS(content, _offset)) => {
                modrm = 0x40;
                if let Operand::REG(name) = content.deref() {
                    modrm |= Operand::number(name);
                }
                if let Some(Operand::REG(name)) = rop {
                    modrm |= Operand::number(name) << 3;
                }
            }
            Some(Operand::REG(name)) => {
                modrm |= Operand::number(name);
                match rop {
                    Some(Operand::IMM(_)) => (),
                    Some(Operand::REG(name)) => {
                        modrm |= Operand::number(name) << 3;
                    }
                    Some(Operand::ADDRESS(content, _offset)) => {
                        modrm = 0x40;
                        modrm |= Operand::number(name) << 3;
                        if let Operand::REG(name) = content.deref() {
                            modrm |= Operand::number(name);
                        }
                    }
                    Some(Operand::ELEMENT(_base, ind, _scale, _)) => {
                        modrm = 0x40;
                        modrm |= Operand::number(name) << 3;
                        if let Operand::REG(name) = ind.deref() {
                            modrm |= Operand::number(name);
                        }
                    }
                    _ => (),
                }
            }
            Some(Operand::IMM(_)) => (),

            _ => (),
        }
        modrm
    }
}
pub fn generate(
    inst_map: BTreeMap<String, Vec<Inst>>,
    info_map: BTreeMap<usize, Info>,
    rels_map: BTreeMap<String, Rela>,
) -> (BTreeMap<String, Vec<u8>>, BTreeMap<String, Rela>) {
    let mut generator: Generator = Generator {
        insts: Vec::new(),
        info_map: info_map,
        codes: Vec::new(),
        rels_map: rels_map,
        symbol_map: BTreeMap::new(),
        jump_map: BTreeMap::new(),
        offset: 0,
    };
    for (symbol, insts) in inst_map.iter() {
        generator.insts = insts.to_vec();
        generator.gen();
        let md = generator.codes.len() % 4;
        for _ in 0..(4 - md) {
            generator.codes.push(0x00);
        }
        generator.offset += generator.codes.len() as u64;
        generator
            .symbol_map
            .insert(symbol.to_string(), generator.codes.to_vec());
        generator.codes = Vec::new();
    }
    (generator.symbol_map, generator.rels_map)
}
