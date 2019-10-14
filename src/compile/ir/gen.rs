use super::super::super::ce::types::Error;
use super::super::frontend::frontmanager::frontmanager::FrontManager;
use super::super::frontend::parse::node::Node;
use super::tac::{Operand, Tac};

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
                    Error::TYPE.found(&format!("{} is not defined", &name));
                }
                self.add(Tac::LET(
                    Operand::ID(name.to_string(), stack_offset, None),
                    expr_op,
                ));
            }
            Node::BLOCK(bstmts) => {
                let stmts: Vec<Node> = *bstmts.clone();
                for st in stmts.iter() {
                    self.gen_stmt(st);
                }
            }
            Node::RETURN(bch) => {
                let ch: Node = *bch.clone();
                let ret_op: Operand = self.gen_expr(ch).unwrap();
                self.add(Tac::RET(ret_op));
            }
            _ => (),
        }
    }
    fn gen_expr(&mut self, n: Node) -> Option<Operand> {
        match n {
            Node::ADD(blop, brop, _) => {
                let lop: Operand = self.gen_expr(*blop.clone()).unwrap();
                let rop: Operand = self.gen_expr(*brop.clone()).unwrap();
                let virt = self.virt;
                self.add(Tac::EX(
                    Operand::REG(virt, 0, None),
                    String::from("+"),
                    lop,
                    rop,
                ));
                self.virt += 1;
                Some(Operand::REG(virt, 0, None))
            }
            Node::SUB(blop, brop, _) => {
                let lop: Operand = self.gen_expr(*blop.clone()).unwrap();
                let rop: Operand = self.gen_expr(*brop.clone()).unwrap();
                let virt = self.virt;
                self.add(Tac::EX(
                    Operand::REG(virt, 0, None),
                    String::from("-"),
                    lop,
                    rop,
                ));
                self.virt += 1;
                Some(Operand::REG(virt, 0, None))
            }
            Node::MUL(blop, brop, _) => {
                let lop: Operand = self.gen_expr(*blop.clone()).unwrap();
                let rop: Operand = self.gen_expr(*brop.clone()).unwrap();
                let virt = self.virt;
                self.add(Tac::EX(
                    Operand::REG(virt, 0, None),
                    String::from("*"),
                    lop,
                    rop,
                ));
                self.virt += 1;
                Some(Operand::REG(virt, 0, None))
            }
            Node::DIV(blop, brop, _) => {
                let lop: Operand = self.gen_expr(*blop.clone()).unwrap();
                let rop: Operand = self.gen_expr(*brop.clone()).unwrap();
                let virt = self.virt;
                self.add(Tac::EX(
                    Operand::REG(virt, 0, None),
                    String::from("/"),
                    lop,
                    rop,
                ));
                self.virt += 1;
                Some(Operand::REG(virt, 0, None))
            }
            Node::ADDRESS(blop, _) => {
                let lop: Operand = self.gen_expr(*blop.clone()).unwrap();
                let virt = self.virt;
                self.add(Tac::UNEX(
                    Operand::REG(virt, 0, None),
                    String::from("&"),
                    lop,
                ));
                self.virt += 1;
                Some(Operand::REG(virt, 0, None))
            }

            Node::DEREFERENCE(blop, _) => {
                let lop: Operand = self.gen_expr(*blop.clone()).unwrap();
                let virt = self.virt;
                self.add(Tac::UNEX(
                    Operand::REG(virt, 0, None),
                    String::from("*"),
                    lop,
                ));
                self.virt += 1;
                Some(Operand::REG(virt, 0, None))
            }
            Node::CALL(name, bargs) => {
                let args: Vec<Node> = *bargs.clone();
                let len: usize = args.len();
                for (idx, arg) in args.iter().enumerate() {
                    let arg_op: Operand = self.gen_expr(arg.clone()).unwrap();
                    self.add(Tac::PARAM(idx, arg_op));
                }
                Some(Operand::CALL(name, len))
            }
            Node::ARRAYLIT(belems, num) => {
                let mut stack_offset = 0;
                if let Some(sym) = self.cur_env.sym_table.get(&format!("Array{}", num)) {
                    stack_offset = sym.stack_offset;
                } else {
                    eprintln!("Array{} is not defined.", num);
                }
                for (idx, elem) in belems.iter().enumerate() {
                    let elem_op: Operand = self.gen_expr(elem.clone()).unwrap();
                    self.add(Tac::LET(
                        Operand::ID(
                            format!("Array{}", num),
                            stack_offset,
                            Some(Box::new(Operand::INTLIT(idx as i128))),
                        ),
                        elem_op,
                    ));
                }
                Some(Operand::ID(format!("Array{}", num), stack_offset, None))
            }
            Node::INDEX(bbase, bindex) => {
                let base_op: Operand = self.gen_expr(*bbase.clone()).unwrap();
                let index_op: Operand = self.gen_expr(*bindex.clone()).unwrap();
                match base_op {
                    Operand::ID(name, stack_offset, _) => {
                        Some(Operand::ID(name, stack_offset, Some(Box::new(index_op))))
                    }
                    Operand::REG(_virt, _phys, _) => {
                        Some(Operand::REG(self.virt, 0, Some(Box::new(index_op))))
                    }
                    _ => None,
                }
            }
            Node::IDENT(name) => {
                let mut stack_offset = 0;
                if let Some(sym) = self.cur_env.sym_table.get(&name) {
                    stack_offset = sym.stack_offset;
                } else {
                    Error::TYPE.found(&format!("{} is not defined", &name));
                }
                Some(Operand::ID(name, stack_offset, None))
            }
            Node::INTEGER(val) => Some(Operand::INTLIT(val)),
            _ => None,
        }
    }
    fn add(&mut self, tac: Tac) {
        self.tacs.push(tac);
    }
}
