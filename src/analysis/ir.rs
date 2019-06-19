use super::super::lex::token::IntType;
use colored::*;

const REG64: [&str; 8] = ["rax", "rbx", "rcx", "rdx", "rsi", "rdi", "rbp", "rsp"];
const XREG64: [&str; 8] = ["r8", "r9", "r10", "r11", "r12", "r13", "r14", "r15"];
const BP64: u8 = 7;
const AX64: u8 = 0;
#[derive(Clone)]
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
    pub fn new_rbp() -> Register {
        Register {
            bits: 64,
            vnum: BP64,
            name: REG64[BP64 as usize],
        }
    }
    pub fn new_rax() -> Register {
        Register {
            bits: 64,
            vnum: AX64,
            name: REG64[AX64 as usize],
        }
    }
    pub fn invalid() -> Register {
        Register {
            bits: 0,
            vnum: 0,
            name: "invalid",
        }
    }
}
pub enum IMMType {
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
pub struct Immediate {
    bits: u8,
    ty: IMMType,
}
impl Immediate {
    pub fn new_imm(sem_val: i128) -> Immediate {
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
    pub fn new_uimm(sem_val: u128) -> Immediate {
        match IntType::judgeu(sem_val) {
            IntType::U8 => Immediate {
                bits: 8,
                ty: IMMType::UIMM8(sem_val as u8),
            },
            IntType::U16 => Immediate {
                bits: 16,
                ty: IMMType::UIMM16(sem_val as u16),
            },
            IntType::U32 => Immediate {
                bits: 32,
                ty: IMMType::UIMM32(sem_val as u32),
            },
            IntType::U64 => Immediate {
                bits: 64,
                ty: IMMType::UIMM64(sem_val as u64),
            },
            _ => Immediate {
                bits: 128,
                ty: IMMType::UIMM128(sem_val),
            },
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
    pub fn new_label(name: String) -> IR {
        IR::new(IRType::LABEL(name))
    }
    pub fn new_push64(reg_num: u8) -> IR {
        IR::new(IRType::PUSH64(Register::new64(reg_num)))
    }
    pub fn new_ret(reg_num: u8) -> IR {
        IR::new(IRType::RETURNREG(
            Register::new_rax(),
            Register::new64(reg_num),
        ))
    }
    pub fn new_addreg(reg1: Register, reg2: Register) -> IR {
        IR::new(IRType::ADDREG(reg1, reg2))
    }
    pub fn new_subreg(reg1: Register, reg2: Register) -> IR {
        IR::new(IRType::SUBREG(reg1, reg2))
    }
    pub fn new_mulreg(reg1: Register, reg2: Register) -> IR {
        IR::new(IRType::MULREG(reg1, reg2))
    }
    pub fn new_divreg(reg1: Register, reg2: Register) -> IR {
        IR::new(IRType::DIVREG(reg1, reg2))
    }
    pub fn new_imm(imm: Immediate, reg: Register) -> IR {
        IR::new(IRType::IMM(reg, imm))
    }
    pub fn new_uimm(imm: Immediate, reg: Register) -> IR {
        IR::new(IRType::UIMM(reg, imm))
    }
    pub fn new_letreg(ident: Register, stacksize: u8) -> IR {
        IR::new(IRType::LETREG(ident, stacksize))
    }
    pub fn dump(&self) {
        match &self.ty {
            IRType::LABEL(label_name) => println!("label {}:", label_name.blue().bold()),

            IRType::IMM(reg, imm) => println!("immediate reg:{} imm", reg.name.blue().bold()),
            IRType::UIMM(reg, imm) => println!("u-immediate reg:{} imm", reg.name.blue().bold()),

            IRType::PUSH64(reg) => println!("push-reg:{}", reg.name.blue().bold()),

            IRType::ADDREG(reg1, reg2) => println!(
                "add reg-reg:{} + {}",
                reg1.name.blue().bold(),
                reg2.name.blue().bold()
            ),
            IRType::ADDIMM(reg, imm) => println!("add reg-imm:{} + imm", reg.name.blue().bold()),
            IRType::SUBREG(reg1, reg2) => println!(
                "sub reg-reg:{} - {}",
                reg1.name.blue().bold(),
                reg2.name.blue().bold()
            ),
            IRType::SUBIMM(reg, imm) => println!("sub reg-imm:{} - imm", reg.name.blue().bold()),
            IRType::MULREG(reg1, reg2) => println!(
                "mul reg-reg:{} * {}",
                reg1.name.blue().bold(),
                reg2.name.blue().bold()
            ),
            IRType::MULIMM(reg, imm) => println!("mul reg-imm:{} + imm", reg.name.blue().bold()),
            IRType::DIVREG(reg1, reg2) => println!(
                "div reg-reg:{} / {}",
                reg1.name.blue().bold(),
                reg2.name.blue().bold()
            ),
            IRType::DIVIMM(reg, imm) => println!("div reg-imm:{} - imm", reg.name.blue().bold()),

            IRType::LETREG(reg1, stacksize) => println!(
                "let ident:{} stacksize:{}",
                reg1.name.blue().bold(),
                stacksize
            ),
            IRType::LETIMM(reg1, stacksize) => println!(
                "let ident:{} src:imm stacksize:{}",
                reg1.name.blue().bold(),
                stacksize
            ),
            //PROLOGUE => println!(""),
            //EPILOGUE,
            IRType::RETURNREG(reg1, reg2) => println!(
                "return reg-reg:{} <- {}",
                reg1.name.blue().bold(),
                reg2.name.blue().bold()
            ),
            IRType::RETURNIMM(reg, imm) => {
                println!("return reg-immL{} <- imm", reg.name.blue().bold())
            }
            _ => (),
        }
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

    /* accumulate */
    ADDREG(Register, Register),
    ADDIMM(Register, Immediate),
    SUBREG(Register, Register),
    SUBIMM(Register, Immediate),
    MULREG(Register, Register),
    MULIMM(Register, Immediate),
    DIVREG(Register, Register),
    DIVIMM(Register, Immediate),

    /* statement */
    LETREG(Register, u8), // u8 => stacksize
    LETIMM(Register, u8), // u8 => stacksize

    /* prologue-epilogue */
    PROLOGUE,
    EPILOGUE,
    RETURNREG(Register, Register),
    RETURNIMM(Register, Immediate),
}
