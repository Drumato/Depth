use super::super::analysis::{ir, semantic};
use super::super::lex::token::{Token, TokenType, TokenVal};
use super::super::parse::{error, node};
use ir::{IRType, Immediate, Register, IR};
use std::collections::HashMap;
//ファイル単位で存在させる(予定の)構造体
pub struct Manager {
    pub nodes: Vec<node::Node>,
    pub irs: Vec<ir::IR>,
    pub env: semantic::Environment,
    pub nreg: usize,
    //pub stacksize: u8,
}

impl Manager {
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
            node::NodeType::LETS(_, ident, _, ex) => {
                let assign_reg: Register = self.gen_expr(&ex[0]).unwrap();
                if let node::NodeType::ID(ident_name) = &ident[0].ty {
                    if let semantic::SymbolType::ID(_, _, stacksize, _) =
                        self.env.var_tables.get(ident_name).unwrap().ty
                    {
                        self.irs.push(IR::new_letreg(assign_reg, stacksize.clone()));
                    }
                }
            }
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
            node::NodeType::ID(ident_name) => Some(self.gen_ident(&ident_name)),
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
            _ => Register::invalid(),
        }
    }
    fn gen_ident(&mut self, ident_name: &String) -> Register {
        let reg: Register = self.new_reg();
        if let semantic::SymbolType::ID(_, _, stacksize, _) =
            self.env.var_tables.get(ident_name).unwrap().ty
        {
            self.irs.push(IR::new_ident(reg.clone(), stacksize));
        }
        reg
    }
    fn gen_imm(&mut self, tk: &Token) -> Register {
        let reg: Register = self.new_reg();
        match tk.val {
            TokenVal::IntVal(integer) => {
                let reg: Register = Register::new64(reg.vnum as u8);
                self.irs
                    .push(IR::new_imm(Immediate::new_imm(integer), reg.clone()));
                reg
            }
            TokenVal::UintVal(unsigned) => {
                let reg: Register = Register::new64(reg.vnum as u8);
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
