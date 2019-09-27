use super::super::frontend::frontmanager::frontmanager::FrontManager;
use super::super::frontend::parse::node::Node;
use super::super::frontend::sema::semantics::Type;
use super::super::frontend::token::token::Token;
use super::tac::{Lvalue, Operand, Tac};

impl FrontManager {
    pub fn gen_tacs(&mut self) {
        let functions = self.functions.clone();
        for func in functions.iter() {
            self.add(Tac::FUNC(func.name.clone()));
            for st in func.stmts.iter() {
                match st {
                    Node::RETURN(bch) => {
                        let ch: Node = *bch.clone();
                        let ret_op: Operand = self.gen_expr(ch).unwrap();
                        self.add(Tac::RET(ret_op));
                    }
                    _ => (),
                }
            }
        }
    }
    fn gen_expr(&mut self, n: Node) -> Option<Operand> {
        if let Node::BINOP(op, blop, brop, _) = n {
            let lop: Operand = self.gen_expr(*blop.clone()).unwrap();
            let rop: Operand = self.gen_expr(*brop.clone()).unwrap();
            let op: String = match op {
                Token::PLUS => "+".to_string(),
                Token::MINUS => "-".to_string(),
                Token::STAR => "*".to_string(),
                Token::SLASH => "/".to_string(),
                Token::PERCENT => "%".to_string(),
                Token::LSHIFT => "<<".to_string(),
                Token::RSHIFT => ">>".to_string(),
                Token::LT => "<".to_string(),
                Token::GT => ">".to_string(),
                Token::LTEQ => "<=".to_string(),
                Token::GTEQ => ">=".to_string(),
                Token::EQ => "==".to_string(),
                Token::NTEQ => "!=".to_string(),
                _ => "(inv)".to_string(),
            };
            let virt = self.virt;
            self.add(Tac::EX(Lvalue::REG(virt, 0), op, lop, rop));
            self.virt += 1;
            return Some(Operand::REG(virt, 0));
        } else if let Node::UNARY(op, blop, _) = n {
            let lop: Operand = self.gen_expr(*blop.clone()).unwrap();
            let op: String = match op {
                Token::AMPERSAND => "&".to_string(),
                Token::STAR => "*".to_string(),
                Token::MINUS => "-".to_string(),
                _ => "(inv)".to_string(),
            };
            let virt = self.virt;
            self.add(Tac::UNEX(Lvalue::REG(virt, 0), op, lop));
            self.virt += 1;
            return Some(Operand::REG(virt, 0));
        } else if let Node::NUMBER(t) = n {
            if let Type::INTEGER(ty) = t {
                return Some(Operand::INTLIT(ty.val.unwrap()));
            }
            return None;
        }
        None
    }
    fn add(&mut self, tac: Tac) {
        self.tacs.push(tac);
    }
}
