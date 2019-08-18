use super::super::ir::hi::HIR;
use super::super::parse::node;
use super::super::token::token::Token;
use super::manager::Manager;
impl Manager {
    pub fn gen_expr(&mut self, n: node::Node) -> usize {
        match n {
            node::Node::BINOP(t, blhs, brhs) => {
                let lhs: node::Node = unsafe { Box::into_raw(blhs).read() };
                let rhs: node::Node = unsafe { Box::into_raw(brhs).read() };

                let lr: usize = self.gen_expr(lhs);
                let rr: usize = self.gen_expr(rhs);
                self.gen_binop(t, lr, rr);
                self.regnum -= 1;
                self.regnum
            }
            node::Node::INTEGER(int) => {
                let load_reg: usize = self.regnum;
                self.hirs.push(HIR::LOAD(load_reg, int));
                self.regnum += 1;
                self.regnum
            }
            _ => 42,
        }
    }
    fn gen_binop(&mut self, t: Token, lr: usize, rr: usize) {
        match t {
            Token::PLUS => {
                self.hirs.push(HIR::ADD(lr - 1, rr - 1));
            }
            Token::MINUS => {
                self.hirs.push(HIR::SUB(lr - 1, rr - 1));
            }
            _ => (),
        }
    }
}