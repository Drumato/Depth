use crate::ce::types::Error;
use crate::compile::frontend;
use crate::compile::ir::tac::{Operand, Tac};
use frontend::frontmanager::frontmanager::FrontManager;
use frontend::parse::node::Node;
use frontend::sema::semantics::Type;

use std::collections::BTreeMap;

impl FrontManager {
    pub fn gen_tacs(&mut self) {
        let functions = self.functions.clone();
        for func in functions.iter() {
            self.cur_env = func.env.clone();
            self.add(Tac::FUNCNAME(func.name.clone()));
            self.add(Tac::PROLOGUE(self.stack_offset));
            for (idx, arg) in func.args.iter().enumerate() {
                let mut stack_offset: usize = 0;
                if let Node::DEFARG(name) = arg {
                    if let Some(sym) = self.get_symbol(name) {
                        stack_offset = sym.stack_offset;
                    }
                }
                self.add(Tac::PUSHARG(idx, stack_offset));
            }
            for st in func.stmts.iter() {
                self.gen_stmt(st);
            }
        }
    }
    fn gen_stmt(&mut self, st: &Node) {
        match st {
            Node::LET(name, bexpr) | Node::ASSIGN(name, bexpr) => {
                let expr_op: Operand = self.gen_expr(*bexpr.clone()).unwrap();
                let mut stack_offset = 0;
                if let Some(sym) = self.get_symbol(name) {
                    stack_offset = sym.stack_offset;
                } else {
                    Error::UNDEFINED.found(&format!("{} is not defined", &name));
                }
                match *bexpr.clone() {
                    Node::STRUCTLIT(_, _) => (),
                    Node::ARRAYLIT(_, _) => (),
                    _ => {
                        self.add(Tac::LET(
                            Operand::ID(name.to_string(), stack_offset, None, None),
                            expr_op,
                        ));
                    }
                }
            }
            Node::IF(bcond, block, alter) => {
                let cond_op: Operand = self.gen_expr(*bcond.clone()).unwrap();
                let label: usize = self.label;
                self.add(Tac::IFF(cond_op, format!(".L{}", label)));
                self.label += 1;
                self.gen_stmt(block);
                if let Some(alt) = alter {
                    self.add(Tac::GOTO(format!(".L{}", self.label)));
                    self.add(Tac::LABEL(format!(".L{}", self.label - 1)));
                    self.label += 1;
                    self.gen_stmt(alt);
                    self.add(Tac::LABEL(format!(".L{}", self.label - 1)));
                } else {
                    self.add(Tac::LABEL(format!(".L{}", label)));
                }
            }
            Node::CONDLOOP(bcond, block) => {
                let loop_label: usize = self.label;
                self.add(Tac::LABEL(format!(".L{}", loop_label)));
                self.label += 1;
                let cond_op: Operand = self.gen_expr(*bcond.clone()).unwrap();
                let break_label: usize = self.label;
                self.add(Tac::IFF(cond_op, format!(".L{}", break_label)));
                self.label += 1;
                self.gen_stmt(block);
                self.add(Tac::GOTO(format!(".L{}", loop_label)));
                self.add(Tac::LABEL(format!(".L{}", break_label)));
            }
            Node::BLOCK(stmts) => {
                for st in stmts.iter() {
                    self.gen_stmt(st);
                }
            }
            Node::RETURN(bch) => {
                let ch: Node = *bch.clone();
                let ret_op: Operand = self.gen_expr(ch).unwrap();
                self.add(Tac::RET(ret_op));
            }
            Node::LABEL(label) => {
                self.add(Tac::LABEL(format!(".L{}", label)));
            }
            Node::GOTO(label) => {
                self.add(Tac::GOTO(format!(".L{}", label)));
            }
            _ => (),
        }
    }
    fn gen_expr(&mut self, n: Node) -> Option<Operand> {
        match n {
            Node::ADD(blop, brop) => self.add_binop(blop, brop, "+"),
            Node::SUB(blop, brop) => self.add_binop(blop, brop, "-"),
            Node::MUL(blop, brop) => self.add_binop(blop, brop, "*"),
            Node::DIV(blop, brop) => self.add_binop(blop, brop, "/"),
            Node::MOD(blop, brop) => self.add_binop(blop, brop, "%"),
            Node::LT(blop, brop) => self.add_binop(blop, brop, "<"),
            Node::GT(blop, brop) => self.add_binop(blop, brop, ">"),
            Node::LSHIFT(blop, brop) => self.add_binop(blop, brop, "<<"),
            Node::RSHIFT(blop, brop) => self.add_binop(blop, brop, ">>"),
            Node::LTEQ(blop, brop) => self.add_binop(blop, brop, "<="),
            Node::GTEQ(blop, brop) => self.add_binop(blop, brop, ">="),
            Node::EQ(blop, brop) => self.add_binop(blop, brop, "=="),
            Node::NTEQ(blop, brop) => self.add_binop(blop, brop, "!="),
            Node::ADDRESS(blop) => self.add_unary(blop, "&"),
            Node::DEREFERENCE(blop) => self.add_unary(blop, "*"),
            Node::MINUS(blop) => self.add_unary(blop, "-"),
            Node::CALL(name, bargs) => {
                let args: Vec<Node> = *bargs.clone();
                let len: usize = args.len();
                for (idx, arg) in args.iter().enumerate() {
                    let arg_op: Operand = self.gen_expr(arg.clone()).unwrap();
                    self.add(Tac::PARAM(idx, arg_op));
                }
                Some(Operand::CALL(name, len))
            }
            Node::STRUCTLIT(st_name, member_map) => {
                let virt = self.virt;
                let mut member_symbols = BTreeMap::new();
                if let Some(sym) = self.cur_env.sym_table.get(&st_name) {
                    if let Ok(s_ty) = &sym.ty {
                        if let Type::STRUCT(map, _) = s_ty {
                            member_symbols = map.clone();
                        }
                    }
                } else {
                    Error::UNDEFINED.found(&format!("{} is not defined", &st_name));
                }
                for (member_name, member_expr) in member_map.iter() {
                    let member_op: Operand = self.gen_expr(member_expr.clone()).unwrap();
                    if let Some(member_s) = member_symbols.get(member_name) {
                        self.add(Tac::LET(
                            Operand::ID(st_name.to_string(), 0, None, Some(member_s.stack_offset)),
                            member_op,
                        ));
                    }
                }
                self.virt += 1;
                Some(Operand::REG(virt, 0, None, None))
            }
            Node::ARRAYLIT(belems, name) => {
                let mut stack_offset = 0;
                if let Some(sym) = self.cur_env.sym_table.get(&name) {
                    stack_offset = sym.stack_offset;
                } else {
                    Error::UNDEFINED.found(&format!("{} is not defined", name));
                }
                for (idx, elem) in belems.iter().enumerate() {
                    let elem_op: Operand = self.gen_expr(elem.clone()).unwrap();
                    self.add(Tac::LET(
                        Operand::ID(
                            name.to_string(),
                            stack_offset,
                            Some(Box::new(Operand::INTLIT(idx as i128))),
                            None,
                        ),
                        elem_op,
                    ));
                }
                Some(Operand::ID(name, stack_offset, None, None))
            }
            Node::INDEX(bbase, bindex) => {
                let base_op: Operand = self.gen_expr(*bbase.clone()).unwrap();
                let index_op: Operand = self.gen_expr(*bindex.clone()).unwrap();
                match base_op {
                    Operand::ID(name, stack_offset, _, _) => Some(Operand::ID(
                        name,
                        stack_offset,
                        Some(Box::new(index_op)),
                        None,
                    )),
                    Operand::REG(_virt, _phys, _, _) => {
                        Some(Operand::REG(self.virt, 0, Some(Box::new(index_op)), None))
                    }
                    _ => None,
                }
            }
            Node::MEMBER(st, member) => {
                let struct_op: Operand = self.gen_expr(*st.clone()).unwrap();
                match struct_op {
                    Operand::ID(name, stack_offset, _, _) => {
                        if let Some(s) = self.get_symbol(&name) {
                            if let Ok(st_ty) = s.ty {
                                if let Type::STRUCT(map, _) = st_ty {
                                    if let Some(member_s) = map.get(&member) {
                                        return Some(Operand::ID(
                                            name,
                                            stack_offset,
                                            None,
                                            Some(member_s.stack_offset),
                                        ));
                                    }
                                }
                            }
                        }
                        None
                    }
                    Operand::REG(_virt, _phys, _, _) => {
                        Some(Operand::REG(self.virt, 0, None, Some(0)))
                    }
                    _ => None,
                }
            }
            Node::IDENT(name) => {
                let mut stack_offset = 0;
                if let Some(sym) = self.cur_env.sym_table.get(&name) {
                    stack_offset = sym.stack_offset;
                } else {
                    Error::UNDEFINED.found(&format!("{} is not defined", &name));
                }
                Some(Operand::ID(name, stack_offset, None, None))
            }
            Node::INTEGER(val) => Some(Operand::INTLIT(val)),

            _ => None,
        }
    }
    fn add(&mut self, tac: Tac) {
        self.tacs.push(tac);
    }
    fn add_unary(&mut self, blop: Box<Node>, op: &str) -> Option<Operand> {
        let lop: Operand = self.gen_expr(*blop.clone()).unwrap();
        let virt = self.virt;
        self.add(Tac::UNEX(
            Operand::REG(virt, 0, None, None),
            String::from(op),
            lop,
        ));
        self.virt += 1;
        Some(Operand::REG(virt, 0, None, None))
    }
    fn add_binop(&mut self, blop: Box<Node>, brop: Box<Node>, op: &str) -> Option<Operand> {
        let lop: Operand = self.gen_expr(*blop.clone()).unwrap();
        let rop: Operand = self.gen_expr(*brop.clone()).unwrap();
        let virt = self.virt;
        self.add(Tac::EX(
            Operand::REG(virt, 0, None, None),
            String::from(op),
            lop,
            rop,
        ));
        self.virt += 1;
        Some(Operand::REG(virt, 0, None, None))
    }
}
