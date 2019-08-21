use super::super::ir::hi::HIR;
use super::super::parse::node;
use super::super::token::token::Token;
use super::manager::Manager;
use super::semantics::Type;
impl Manager {
    pub fn gen_irs(&mut self) {
        let func_num: usize = self.functions.len();
        let mut idx: usize = 0;
        loop {
            if idx == func_num {
                break;
            }
            let f: node::Func = self.functions[idx].clone();
            self.hirs.push(HIR::SYMBOL(f.name));
            self.hirs.push(HIR::PROLOGUE);
            for n in f.stmts {
                self.gen_stmt(n);
            }
            self.hirs.push(HIR::EPILOGUE);
            idx += 1;
        }
    }
    fn gen_stmt(&mut self, n: node::Node) {
        match n {
            node::Node::RETURN(bexpr) => {
                let expr: node::Node = unsafe { Box::into_raw(bexpr).read() };
                let return_reg: usize = self.gen_expr(expr) - 1;
                self.hirs.push(HIR::RETURN(return_reg));
            }
            node::Node::IF(bcond, bstmt, oalter) => {
                let cond: node::Node = unsafe { Box::into_raw(bcond).read() };
                let cmp_reg: usize = self.gen_expr(cond) - 1;
                self.hirs.push(HIR::CMP(cmp_reg, self.labelnum));
                let stmt: node::Node = unsafe { Box::into_raw(bstmt).read() };
                self.gen_stmt(stmt);
                match oalter {
                    Some(balter) => {
                        self.hirs.push(HIR::JUMP(self.labelnum + 1));
                        self.hirs.push(HIR::LABEL(self.labelnum));
                        self.labelnum += 1;
                        let alter: node::Node = unsafe { Box::into_raw(balter).read() };
                        self.gen_stmt(alter);
                    }
                    None => (),
                }
                self.hirs.push(HIR::LABEL(self.labelnum));
                self.labelnum += 1;
            }
            node::Node::BLOCK(bstmts) => {
                let stmts: Vec<node::Node> = bstmts
                    .into_iter()
                    .map(|bst| unsafe { Box::into_raw(bst).read() })
                    .collect::<Vec<node::Node>>();
                for st in stmts {
                    self.gen_stmt(st);
                }
            }
            node::Node::LET(ident_name, _, bexpr) => {
                let expr: node::Node = unsafe { Box::into_raw(bexpr).read() };
                self.gen_expr(expr);
                self.regnum -= 1;
                self.hirs.push(HIR::STORE(
                    self.var_table.get(&ident_name).unwrap().stack_offset,
                    self.regnum,
                ));
            }
            _ => (),
        }
    }
    fn gen_expr(&mut self, n: node::Node) -> usize {
        match n {
            node::Node::UNARY(_t, binner, _) => {
                let inner: node::Node = unsafe { Box::into_raw(binner).read() };
                let rr: usize = self.gen_expr(inner);
                self.hirs.push(HIR::NEGATIVE(rr - 1));
                self.regnum
            }
            node::Node::BINOP(t, blhs, brhs, _) => {
                let lhs: node::Node = unsafe { Box::into_raw(blhs).read() };
                let rhs: node::Node = unsafe { Box::into_raw(brhs).read() };

                let lr: usize = self.gen_expr(lhs);
                let rr: usize = self.gen_expr(rhs);
                self.gen_binop(t, lr, rr);
                self.regnum -= 1;
                self.regnum
            }
            node::Node::NUMBER(ty) => match ty {
                Type::INTEGER(int, _, _) => {
                    let load_reg: usize = self.regnum;
                    self.hirs.push(HIR::LOAD(load_reg, int));
                    self.regnum += 1;
                    self.regnum
                }
                _ => self.regnum,
            },
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
            Token::STAR => {
                self.hirs.push(HIR::MUL(lr - 1, rr - 1));
            }
            Token::SLASH => {
                self.hirs.push(HIR::DIV(lr - 1, rr - 1));
            }
            Token::PERCENT => {
                self.hirs.push(HIR::MOD(lr - 1, rr - 1));
            }
            Token::LSHIFT => {
                self.hirs.push(HIR::LSHIFT(lr - 1, rr - 1));
            }
            Token::RSHIFT => {
                self.hirs.push(HIR::RSHIFT(lr - 1, rr - 1));
            }
            Token::LT => {
                self.hirs.push(HIR::LT(lr - 1, rr - 1));
            }
            Token::GT => {
                self.hirs.push(HIR::GT(lr - 1, rr - 1));
            }
            Token::LTEQ => {
                self.hirs.push(HIR::LTEQ(lr - 1, rr - 1));
            }
            Token::GTEQ => {
                self.hirs.push(HIR::GTEQ(lr - 1, rr - 1));
            }
            Token::EQ => {
                self.hirs.push(HIR::EQ(lr - 1, rr - 1));
            }
            Token::NTEQ => {
                self.hirs.push(HIR::NTEQ(lr - 1, rr - 1));
            }
            _ => (),
        }
    }
}
