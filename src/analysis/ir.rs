use super::super::parse::{error, node};
use super::super::token::IntType;

pub struct Register {
    pub bits: u8, //8,16,32,64
    pub vnum: u8,
    pub name: &'static str,
}
impl Register {
    pub fn new64(vnum: u8) -> Register {
        let register64: [&str; 7] = ["rbx", "r10", "r11", "r12", "r13", "r14", "r15"];
        let reg_name: &'static str = register64[vnum as usize];
        Register {
            bits: 64,
            vnum: vnum,
            name: reg_name,
        }
    }
}
pub struct IR {
    pub ty: IRType,
}

impl IR {
    pub fn new(ty: IRType) -> IR {
        IR { ty: ty }
    }
    pub fn new_intimm(reg_num: u8, sem_val: i128) -> IR {
        match IntType::judge(sem_val) {
            IntType::I8 => IR::new(IRType::IMM8(vec![Register::new64(reg_num)], sem_val as i8)),
            IntType::I16 => IR::new(IRType::IMM16(
                vec![Register::new64(reg_num)],
                sem_val as i16,
            )),
            IntType::I32 => IR::new(IRType::IMM32(
                vec![Register::new64(reg_num)],
                sem_val as i32,
            )),
            IntType::I64 => IR::new(IRType::IMM64(
                vec![Register::new64(reg_num)],
                sem_val as i64,
            )),
            _ => IR::new(IRType::IMM128(
                vec![Register::new64(reg_num)],
                sem_val as i128,
            )),
        }
    }
}

pub enum IRType {
    IMM8(Vec<Register>, i8),
    IMM16(Vec<Register>, i16),
    IMM32(Vec<Register>, i32),
    IMM64(Vec<Register>, i64),
    IMM128(Vec<Register>, i128),
    UIMM8(Vec<Register>, u8),
    UIMM16(Vec<Register>, u16),
    UIMM32(Vec<Register>, u32),
    UIMM64(Vec<Register>, u64),
    UIMM128(Vec<Register>, u128),
}

pub fn generate_ir(n: Vec<node::Node>) -> Vec<IR> {
    let mut irs: Vec<IR> = Vec::new();
    irs.push(IR::new_intimm(1, 30));
    irs
}
