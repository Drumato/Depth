use super::super::parse::{error, node};
use super::super::token::{IntType, Token, TokenType, TokenVal};
use colored::*;
use std::collections::HashMap;

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
    fn invalid() -> Register {
        Register {
            bits: 0,
            vnum: 0,
            name: "invalid",
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
pub struct IRS {
    pub irs: Vec<IR>,
    nreg: usize,
}
impl IRS {
    pub fn new(is: Vec<IR>, nreg: usize) -> IRS {
        IRS {
            irs: is,
            nreg: nreg,
        }
    }
    fn new_reg(&mut self) -> Register {
        let reg: Register = Register::new64(self.nreg as u8);
        self.nreg += 1;
        reg
    }
    fn gen_func(
        &mut self,
        func_name: String,
        args: &HashMap<String, TokenType>,
        ret_type: &TokenType,
        stmts: Vec<node::Node>,
    ) {
        self.irs.push(IR::new_label(func_name));
        self.irs.push(IR::new(IRType::PROLOGUE));
        for st in stmts {
            self.gen_stmt(st);
        }
        self.irs.push(IR::new(IRType::EPILOGUE));
    }
    fn gen_stmt(&mut self, n: node::Node) {
        match &n.ty {
            node::NodeType::RETS(_, ex) => {
                let ret_reg: Register = self.gen_expr(&ex[0]).unwrap();
                self.irs.push(IR::new_ret(ret_reg.vnum));
            }
            //STRUCTS(String, Vec<Node>) => {},
            //LETS(TokenType, Vec<Node>, TokenType, Vec<Node>)=>{},
            //IFS(TokenType, Vec<Node>, Vec<Node>, TokenType, Vec<Node>)=>{},
            //LOOP(TokenType, Vec<Node>)=>{},
            //FOR(TokenType, String, String, Vec<Node>)=>{},
            _ => {
                error::CompileError::SEMA(format!("unable to generate ir")).found();
            }
        }
    }
    fn gen_expr(&mut self, n: &node::Node) -> Option<Register> {
        match &n.ty {
            node::NodeType::INT(tk) => Some(self.gen_imm(&tk)),
            node::NodeType::BINOP(operator, lop, rop) => {
                Some(self.gen_binop(operator, &lop[0], &rop[0]))
            }
            _ => None,
        }
    }

    fn gen_binop(&mut self, op: &TokenType, lop: &node::Node, rop: &node::Node) -> Register {
        let lreg = self.gen_expr(lop).unwrap();
        let rreg = self.gen_expr(rop).unwrap();
        match op {
            TokenType::TkPlus => {
                self.irs.push(IR::new_addreg(lreg.clone(), rreg));
                lreg
            }
            TokenType::TkMinus => lreg,
            _ => Register::invalid(),
        }
    }
    fn gen_imm(&mut self, tk: &Token) -> Register {
        let reg: Register = self.new_reg();
        match tk.val {
            TokenVal::IntVal(integer) => {
                let reg: Register = Register::new64(self.nreg as u8);
                self.irs
                    .push(IR::new_imm(Immediate::new_imm(integer), reg.clone()));
                reg
            }
            TokenVal::UintVal(unsigned) => {
                let reg: Register = Register::new64(self.nreg as u8);
                self.irs
                    .push(IR::new_uimm(Immediate::new_uimm(unsigned), reg.clone()));
                reg
            }
            //RealVal(f64),
            //CharVal(char),
            _ => Register::invalid(),
        }
    }
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
    fn new_ret(reg_num: u8) -> IR {
        IR::new(IRType::RETURNREG(
            Register::new_rax(),
            Register::new64(reg_num),
        ))
    }
    fn new_addreg(reg1: Register, reg2: Register) -> IR {
        IR::new(IRType::ADDREG(reg1, reg2))
    }
    fn new_imm(imm: Immediate, reg: Register) -> IR {
        IR::new(IRType::IMM(reg, imm))
    }
    fn new_uimm(imm: Immediate, reg: Register) -> IR {
        IR::new(IRType::UIMM(reg, imm))
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

    /* prologue-epilogue */
    PROLOGUE,
    EPILOGUE,
    RETURNREG(Register, Register),
    RETURNIMM(Register, Immediate),
}

pub fn generate_ir(nodes: &Vec<node::Node>) -> IRS {
    let mut irs: IRS = IRS::new(Vec::new(), 1);
    for func in nodes {
        if let node::NodeType::FUNC(func_name, args, ret_type, stmts) = &func.ty {
            irs.gen_func(func_name.to_string(), args, ret_type, stmts.to_vec());
        }
    }
    irs
}
