use super::super::analysis::{ir, semantic};
use super::super::lex::token::{Token, TokenType, TokenVal};
use super::super::parse::{error, node};
use ir::{CMPType, IMMType, IRType, Immediate, Register, IR};
use std::collections::HashMap;
use std::io::{BufWriter, Write};
use std::str;
//ファイル単位で存在させる(予定の)構造体
pub struct Manager {
    pub nodes: Vec<node::Node>,
    pub irs: Vec<ir::IR>,
    pub env: semantic::Environment,
    pub nreg: usize,
    pub offset: u8,
    pub nlabel: usize,
}

impl Manager {
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
            node::NodeType::LETS(_, ident, _, ex) => self.gen_letstmt(ident, ex),
            node::NodeType::IFS(_, cond, stmts, _, alter) => self.gen_ifstmt(cond, stmts, alter),
            //LOOP(TokenType, Vec<Node>)=>{},
            //FOR(TokenType, String, String, Vec<Node>)=>{},
            _ => {
                error::CompileError::SEMA(format!("unable to generate ir")).found();
            }
        }
    }
    fn gen_letstmt(&mut self, ident_name: &Vec<node::Node>, expr: &Vec<node::Node>) {
        let assign_reg: Register = self.gen_expr(&expr[0]).unwrap();
        if let node::NodeType::ID(ident_name) = &ident_name[0].ty {
            if let semantic::SymbolType::ID(_, _, ref mut stacksize, _) =
                self.env.var_table.get_mut(ident_name).unwrap().ty
            {
                /* consider auto-var all variables now.*/
                self.offset += *stacksize;
                *stacksize = self.offset;
                self.irs.push(IR::new_letreg(assign_reg, self.offset));
            }
        }
        self.kill();
    }
    fn gen_ifstmt(
        &mut self,
        cond: &Vec<node::Node>,
        stmts: &Vec<node::Node>,
        alter: &Vec<node::Node>,
    ) {
        let judge_reg: Register = self.gen_expr(&cond[0]).unwrap();
        let cond_node: &node::Node = &cond[0];
        if let node::NodeType::BINOP(op, _, _) = &cond_node.ty {
            match &op {
                TokenType::TkLt => self
                    .irs
                    .push(IR::new_jmp(format!(".L{}", self.nlabel), CMPType::LT)),
                TokenType::TkLteq => self
                    .irs
                    .push(IR::new_jmp(format!(".L{}", self.nlabel), CMPType::LTEQ)),
                TokenType::TkGt => self
                    .irs
                    .push(IR::new_jmp(format!(".L{}", self.nlabel), CMPType::GT)),
                TokenType::TkGteq => self
                    .irs
                    .push(IR::new_jmp(format!(".L{}", self.nlabel), CMPType::GTEQ)),
                TokenType::TkNoteq => self
                    .irs
                    .push(IR::new_jmp(format!(".L{}", self.nlabel), CMPType::NTEQ)),
                TokenType::TkEq => self
                    .irs
                    .push(IR::new_jmp(format!(".L{}", self.nlabel), CMPType::EQ)),
                _ => (),
            }
        }
        for st in stmts.iter() {
            self.gen_stmt(st.clone());
        }
        if alter.len() > 0 {
            self.irs
                .push(IR::new_jmp(format!(".L{}", self.nlabel + 1), CMPType::NONE));
            self.irs.push(IR::new_label(format!(".L{}", self.nlabel)));
            for st in alter.iter() {
                self.gen_stmt(st.clone());
            }
            self.nlabel += 1;
            self.irs.push(IR::new_label(format!(".L{}", self.nlabel)));
        } else {
            self.irs.push(IR::new_label(format!(".L{}", self.nlabel)));
            self.nlabel += 1;
        }
    }
    fn gen_expr(&mut self, n: &node::Node) -> Option<Register> {
        match &n.ty {
            node::NodeType::ID(ident_name) => Some(self.gen_ident(&ident_name)),
            node::NodeType::INT(tk) => Some(self.gen_imm(&tk)),
            node::NodeType::CHAR(tk) => Some(self.gen_imm(&tk)),
            node::NodeType::STRING(tk) => Some(self.gen_imm(&tk)),
            node::NodeType::BINOP(operator, lop, rop) => {
                Some(self.gen_binop(operator, &lop[0], &rop[0]))
            }
            _ => None,
        }
    }
    pub fn gen_ir(&mut self, matches: &clap::ArgMatches) {
        for func in self.nodes.to_vec() {
            if let node::NodeType::FUNC(func_name, args, ret_type, stmts) = &func.ty {
                self.gen_func(func_name.to_string(), args, ret_type, stmts.to_vec());
            }
        }
    }
    fn new_reg(&mut self) -> Register {
        let reg: Register = Register::new64(self.nreg as u8);
        self.nreg += 1;
        reg
    }
    fn kill(&mut self) {
        self.nreg -= 1;
    }

    fn gen_binop(&mut self, op: &TokenType, lop: &node::Node, rop: &node::Node) -> Register {
        let lreg = self.gen_expr(lop).unwrap();
        let rreg = self.gen_expr(rop).unwrap();
        match op {
            TokenType::TkPlus => {
                self.irs.push(IR::new_addreg(lreg.clone(), rreg));
                self.kill();
                lreg
            }
            TokenType::TkMinus => {
                self.irs.push(IR::new_subreg(lreg.clone(), rreg));
                self.kill();
                lreg
            }
            TokenType::TkStar => {
                self.irs.push(IR::new_mulreg(lreg.clone(), rreg));
                self.kill();
                lreg
            }
            TokenType::TkSlash => {
                self.irs.push(IR::new_divreg(lreg.clone(), rreg));
                self.kill();
                lreg
            }
            TokenType::TkNoteq => {
                self.irs.push(IR::new_cmpreg(lreg.clone(), rreg));
                self.kill();
                lreg
            }
            TokenType::TkLt => {
                self.irs.push(IR::new_cmpreg(lreg.clone(), rreg));
                self.kill();
                lreg
            }
            TokenType::TkLteq => {
                self.irs.push(IR::new_cmpreg(lreg.clone(), rreg));
                self.kill();
                lreg
            }
            TokenType::TkGteq => {
                self.irs.push(IR::new_cmpreg(lreg.clone(), rreg));
                self.kill();
                lreg
            }
            TokenType::TkGt => {
                self.irs.push(IR::new_cmpreg(lreg.clone(), rreg));
                self.kill();
                lreg
            }
            _ => Register::invalid(),
        }
    }
    fn gen_ident(&mut self, ident_name: &String) -> Register {
        let reg: Register = self.new_reg();
        if let semantic::SymbolType::ID(_, _, stacksize, _) =
            self.env.var_table.get(ident_name).unwrap().ty
        {
            self.irs.push(IR::new_ident(reg.clone(), stacksize));
        }
        reg
    }
    fn gen_imm(&mut self, tk: &Token) -> Register {
        let reg: Register = self.new_reg();
        if let TokenType::TkStrlit = tk.ty {
            let reg: Register = Register::new64(reg.vnum as u8);
            self.irs.push(IR::new_imm(
                reg.clone(),
                Immediate::new_str(tk.literal.clone()),
            ));
            return reg;
        }
        match tk.val {
            TokenVal::IntVal(integer) => {
                let reg: Register = Register::new64(reg.vnum as u8);
                self.irs
                    .push(IR::new_imm(reg.clone(), Immediate::new_imm(integer)));
                reg
            }
            TokenVal::UintVal(unsigned) => {
                let reg: Register = Register::new64(reg.vnum as u8);
                self.irs
                    .push(IR::new_uimm(reg.clone(), Immediate::new_uimm(unsigned)));
                reg
            }
            TokenVal::CharVal(ch) => {
                let reg: Register = Register::new64(reg.vnum as u8);
                self.irs
                    .push(IR::new_char(reg.clone(), Immediate::new_char(ch)));
                reg
            }
            //RealVal(f64),
            _ => Register::invalid(),
        }
    }
    pub fn gen_code(&mut self, matches: &clap::ArgMatches) {
        if matches.is_present("intel") {
            println!(".intel_syntax noprefix");
            println!(".globl main");
        }
        for ir in self.irs.iter() {
            match &ir.ty {
                IRType::LETREG(reg1, stacksize) => {
                    println!("    mov QWORD PTR -{}[rbp], {}", stacksize, reg1.name) //bits -> bytes
                }
                IRType::ADDREG(reg1, reg2) => println!("    add {}, {}", reg1.name, reg2.name),
                IRType::SUBREG(reg1, reg2) => println!("    sub {}, {}", reg1.name, reg2.name),
                IRType::MULREG(reg1, reg2) => {
                    println!("    mov rax, {}", reg1.name);
                    println!("    mul {}", reg2.name);
                    println!("    mov {}, rax", reg1.name)
                }
                IRType::DIVREG(reg1, reg2) => {
                    println!("    mov rax, {}", reg1.name);
                    println!("    div {}", reg2.name);
                    println!("    mov {}, rax", reg1.name)
                }
                IRType::IMM(reg, imm) => match &imm.ty {
                    IMMType::IMM8(v) => println!("    mov {}, 0x{:x}", reg.name, v),
                    IMMType::IMM16(v) => println!("    mov {}, 0x{:x}", reg.name, v),
                    IMMType::IMM32(v) => println!("    mov {}, 0x{:x}", reg.name, v),
                    IMMType::IMM64(v) => println!("    mov {}, 0x{:x}", reg.name, v),
                    IMMType::IMM128(v) => println!("    mov {}, {:x}", reg.name, v),
                    _ => (),

                    IMMType::IMMSTR(v) => {
                        let lit: String = str::from_utf8(v.as_bytes()).unwrap().to_string();
                        println!("    movabs {}, {}", reg.name, lit)
                    }
                },
                IRType::UIMM(reg, imm) => match &imm.ty {
                    IMMType::UIMM8(v) => println!("    mov {}, 0x{:x}", reg.name, v),
                    IMMType::UIMM16(v) => println!("    mov {}, 0x{:x}", reg.name, v),
                    IMMType::UIMM32(v) => println!("    mov {}, 0x{:x}", reg.name, v),
                    IMMType::UIMM64(v) => println!("    mov {}, 0x{:x}", reg.name, v),
                    IMMType::UIMM128(v) => println!("    mov {}, 0x{:x}", reg.name, v),
                    _ => (),
                },
                IRType::CHIMM(reg, imm) => match &imm.ty {
                    IMMType::IMMCHAR(v) => {
                        println!("    mov {}, 0x{:x}", reg.name, v.clone() as u32)
                    }
                    _ => (),
                },
                IRType::JMP(label, cmp) => match cmp {
                    CMPType::GT => println!("    jle {}", label),
                    CMPType::GTEQ => println!("    jl {}", label),
                    CMPType::LTEQ => println!("    jge {}", label),
                    CMPType::LT => println!("    jg {}", label),
                    CMPType::EQ => println!("    jne {}", label),
                    CMPType::NTEQ => println!("    je {}", label),
                    CMPType::NONE => println!("    jmp {}", label),
                },
                IRType::RETURNREG(reg1, reg2) => {
                    println!("    mov {}, {}", reg1.name, reg2.name);
                }
                IRType::CMPREG(reg1, reg2) => {
                    println!("    cmp {}, {}", reg1.name, reg2.name);
                }
                IRType::PROLOGUE => {
                    println!("    push rbp");
                    println!("    mov rbp,rsp");
                    if self.offset > 0 {
                        println!("    sub rsp, 0x{:x}", self.offset);
                    }
                }
                IRType::EPILOGUE => {
                    println!("    mov rsp,rbp");
                    println!("    pop rbp");
                    println!("    ret");
                }
                IRType::LABEL(label_name) => println!("{}:", label_name),
                IRType::ID(reg, stacksize) => {
                    println!("    mov {}, QWORD PTR -{}[rbp]", reg.name, stacksize) // bits->bytes
                }
                _ => (),
            }
        }
    }
}
