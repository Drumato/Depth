use super::super::frontend::frontmanager::frontmanager::FrontManager;
use super::super::frontend::parse::node::Node;
use super::super::frontend::sema::semantics::Type;
use super::tac::{Operand, Tac};

impl FrontManager {
    pub fn gen_tacs(&mut self) {
        let functions = self.functions.clone();
        for func in functions.iter() {
            self.cur_env = func.env.clone();
            self.add(Tac::FUNCNAME(func.name.clone()));
            self.add(Tac::PROLOGUE(self.stack_offset));
            for (idx, arg) in func.args.iter().enumerate() {
                if let Node::DEFARG(name, _) = arg {
                    let mut stack_offset: usize = 0;
                    if let Some(sym) = self.cur_env.sym_table.get(name) {
                        stack_offset = sym.stack_offset;
                    }
                    self.add(Tac::PUSHARG(idx, stack_offset));
                }
            }
            for st in func.stmts.iter() {
                self.gen_stmt(st.clone());
            }
        }
    }
    fn gen_stmt(&mut self, st: Node) {
        match st {
            Node::RETURN(bch) => {
                let ch: Node = *bch.clone();
                let ret_op: Operand = self.gen_expr(ch).unwrap();
                self.add(Tac::RET(ret_op));
            }
            Node::IF(bcond, block, oalter) => {
                let cond_op: Operand = self.gen_expr(*bcond.clone()).unwrap();
                self.add(Tac::IFF(cond_op, format!(".L{}", self.label)));
                self.label += 1;
                self.gen_stmt(*block.clone());
                if let Some(balter) = oalter {
                    self.add(Tac::GOTO(format!(".L{}", self.label)));
                    self.add(Tac::LABEL(format!(".L{}", self.label - 1)));
                    self.gen_stmt(*balter.clone());
                    self.add(Tac::LABEL(format!(".L{}", self.label)));
                } else {
                    self.add(Tac::LABEL(format!(".L{}", self.label - 1)));
                }
            }
            Node::CONDLOOP(bcond, block) => {
                self.add(Tac::LABEL(format!(".L{}", self.label)));
                self.label += 1;
                let cond_op: Operand = self.gen_expr(*bcond.clone()).unwrap();
                self.add(Tac::IFF(cond_op, format!(".L{}", self.label)));
                self.gen_stmt(*block.clone());
                self.add(Tac::GOTO(format!(".L{}", self.label - 1)));
                self.add(Tac::LABEL(format!(".L{}", self.label)));
            }
            Node::LET(name, _, bexpr) => {
                let expr_op: Operand = self.gen_expr(*bexpr.clone()).unwrap();
                let mut stack_offset = 0;
                if let Some(sym) = self.cur_env.sym_table.get(&name) {
                    stack_offset = sym.stack_offset;
                } else {
                    eprintln!("{} is not defined.", name);
                }
                self.add(Tac::LET(Operand::ID(name, stack_offset, None), expr_op));
            }
            Node::ASSIGN(name, bexpr) => {
                let expr_op: Operand = self.gen_expr(*bexpr.clone()).unwrap();
                let mut stack_offset = 0;
                if let Some(sym) = self.cur_env.sym_table.get(&name) {
                    stack_offset = sym.stack_offset;
                } else {
                    eprintln!("{} is not defined.", name);
                }
                self.add(Tac::LET(Operand::ID(name, stack_offset, None), expr_op));
            }
            Node::BLOCK(stmts) => {
                for st in stmts {
                    self.gen_stmt(*st.clone());
                }
            }
            _ => (),
        }
    }
    fn gen_expr(&mut self, n: Node) -> Option<Operand> {
        match n {
            Node::BINOP(op, blop, brop, _) => {
                let lop: Operand = self.gen_expr(*blop.clone()).unwrap();
                let rop: Operand = self.gen_expr(*brop.clone()).unwrap();
                let virt = self.virt;
                self.add(Tac::EX(
                    Operand::REG(virt, 0, None),
                    op.string_ir(),
                    lop,
                    rop,
                ));
                self.virt += 1;
                Some(Operand::REG(virt, 0, None))
            }
            Node::UNARY(op, blop, _) => {
                let lop: Operand = self.gen_expr(*blop.clone()).unwrap();
                let virt = self.virt;
                self.add(Tac::UNEX(Operand::REG(virt, 0, None), op.string_ir(), lop));
                self.virt += 1;
                Some(Operand::REG(virt, 0, None))
            }
            Node::NUMBER(t) => {
                if let Type::INTEGER(ty) = t {
                    return Some(Operand::INTLIT(ty.val.unwrap()));
                }
                None
            }
            Node::CHARLIT(c) => Some(Operand::CHARLIT(c)),
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
                    eprintln!("{} is not defined.", name);
                }
                Some(Operand::ID(name, stack_offset, None))
            }
            Node::CALL(name, args) => {
                let len: usize = args.len();
                for (idx, barg) in args.iter().enumerate() {
                    let arg_op: Operand = self.gen_expr(*barg.clone()).unwrap();
                    self.add(Tac::PARAM(idx, arg_op));
                }
                Some(Operand::CALL(name, len))
            }
            _ => None,
        }
    }
    fn add(&mut self, tac: Tac) {
        self.tacs.push(tac);
    }
}
