use super::super::parse::{error, node};
use super::super::token::{IntType, TokenType};
use std::collections::HashMap;

const REG64: [&str; 8] = ["rax", "rbx", "rcx", "rdx", "rsi", "rdi", "rbp", "rsp"];
const XREG64: [&str; 8] = ["r8", "r9", "r10", "r11", "r12", "r13", "r14", "r15"];
const BP64: u8 = 7;
pub struct Register {
    pub bits: u8, //8,16,32,64
    pub vnum: u8,
    pub name: &'static str,
}
impl Register {
    pub fn new64(vnum: u8) -> Register {
        let reg_name: &'static str = REG64[vnum as usize];
        Register {
            bits: 64,
            vnum: vnum,
            name: reg_name,
        }
    }
}
enum IMMType {
    IMM8(i8),
    IMM16(i16),
    IMM32(i32),
    IMM64(i64),
    IMM128(i128),
    UIMM8(u8),
    UIMM16(u16),
    UIMM32(u32),
    UIMM64(u64),
    UIMM128(u128),
}
struct Immediate {
    bits: u8,
    ty: IMMType,
}
impl Immediate {
    fn new_imm(sem_val: i128) -> Immediate {
        match IntType::judge(sem_val) {
            IntType::I8 => Immediate {
                bits: 8,
                ty: IMMType::IMM8(sem_val as i8),
            },
            IntType::I16 => Immediate {
                bits: 16,
                ty: IMMType::IMM16(sem_val as i16),
            },
            IntType::I32 => Immediate {
                bits: 32,
                ty: IMMType::IMM32(sem_val as i32),
            },
            IntType::I64 => Immediate {
                bits: 64,
                ty: IMMType::IMM64(sem_val as i64),
            },
            _ => Immediate {
                bits: 128,
                ty: IMMType::IMM128(sem_val),
            },
        }
    }
    fn new_uimm(sem_val: u128) -> Immediate {
        match IntType::judgeu(sem_val) {
            _ => Immediate {
                bits: 128,
                ty: IMMType::UIMM128(sem_val),
            },
        }
    }
}
pub struct IRS {
    pub irs: Vec<IR>,
}
pub struct IR {
    pub ty: IRType,
}

impl IR {
    fn new(ty: IRType) -> IR {
        IR { ty: ty }
    }
    fn new_label(name: String) -> IR {
        IR::new(IRType::LABEL(name))
    }
    fn new_push64(reg_num: u8) -> IR {
        IR::new(IRType::PUSH64(Register::new64(reg_num)))
    }
}

pub enum IRType {
    /* label*/
    LABEL(String),

    /* immediate */
    IMM(Register, Immediate),
    UIMM(Register, Immediate),

    MOVIMM(Register),

    /* Stack */
    PUSH64(Register),
    /* prologue-epilogue */
    PROLOGUE,
    EPILOGUE,
}

pub fn generate_ir(nodes: Vec<node::Node>) -> Vec<IR> {
    let mut irs: Vec<IR> = Vec::new();
    for func in nodes {
        if let node::NodeType::FUNC(func_name, args, ret_type, stmts) = func.ty {
            gen_func(func_name, args, ret_type, stmts, &mut irs);
        }
    }
    irs
}

fn gen_func(
    func_name: String,
    args: HashMap<String, TokenType>,
    ret_type: TokenType,
    stmts: Vec<node::Node>,
    irs: &mut Vec<IR>,
) {
    irs.push(IR::new_label(func_name));
    irs.push(IR::new(IRType::PROLOGUE));
    irs.push(IR::new(IRType::EPILOGUE));
}
