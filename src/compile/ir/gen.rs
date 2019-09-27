use super::super::frontend::frontmanager::frontmanager::FrontManager;
use super::super::frontend::parse::node::Node;
use super::super::frontend::sema::semantics::Type;
use super::tac::{Lvalue, Operand, Tac};

impl FrontManager {
    pub fn gen_tacs(&mut self) {
        let functions = self.functions.clone();
        for func in functions.iter() {
            self.add(Tac::LABEL(func.name.clone()));
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
                self.add(Tac::GOTO(format!(".L{}", self.label)));
                if let Some(balter) = oalter {
                    self.add(Tac::LABEL(format!(".L{}", self.label - 1)));
                    self.gen_stmt(*balter.clone());
                }
                self.add(Tac::LABEL(format!(".L{}", self.label)));
            }
            _ => (),
        }
    }
    fn gen_expr(&mut self, n: Node) -> Option<Operand> {
        if let Node::BINOP(op, blop, brop, _) = n {
            let lop: Operand = self.gen_expr(*blop.clone()).unwrap();
            let rop: Operand = self.gen_expr(*brop.clone()).unwrap();
            let virt = self.virt;
            self.add(Tac::EX(Lvalue::REG(virt, 0), op.string_ir(), lop, rop));
            self.virt += 1;
            return Some(Operand::REG(virt, 0));
        } else if let Node::UNARY(op, blop, _) = n {
            let lop: Operand = self.gen_expr(*blop.clone()).unwrap();
            let virt = self.virt;
            self.add(Tac::UNEX(Lvalue::REG(virt, 0), op.string_ir(), lop));
            self.virt += 1;
            return Some(Operand::REG(virt, 0));
        } else if let Node::NUMBER(t) = n {
            if let Type::INTEGER(ty) = t {
                return Some(Operand::INTLIT(ty.val.unwrap()));
            }
            return None;
        } else if let Node::CHARLIT(c) = n {
            return Some(Operand::CHARLIT(c));
        }
        None
    }
    fn add(&mut self, tac: Tac) {
        self.tacs.push(tac);
    }
}
