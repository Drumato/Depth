use super::super::ce::types::Error;
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
            self.hirs.push(HIR::PROLOGUE(self.stack_offset));
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
                self.gen_expr(expr);
                self.regnum -= 1;
                self.hirs.push(HIR::RETURN(self.regnum));
            }
            node::Node::IF(bcond, bstmt, oalter) => {
                let cond: node::Node = unsafe { Box::into_raw(bcond).read() };
                let cmp_reg: usize = self.gen_expr(cond);
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
                let expr_reg: usize = self.gen_expr(expr);
                self.regnum -= 1;
                if let Some(var) = self.var_table.get(&ident_name) {
                    match &var.ty {
                        Type::INTEGER(int_type) => self.hirs.push(HIR::STORE(
                            var.stack_offset,
                            expr_reg,
                            int_type.type_size,
                        )),
                        Type::CHAR(char_type) => self.hirs.push(HIR::STORE(
                            var.stack_offset,
                            expr_reg,
                            char_type.type_size,
                        )),
                        Type::POINTER(_, size) => {
                            self.hirs
                                .push(HIR::STORE(var.stack_offset, expr_reg, *size))
                        }
                        _ => Error::TYPE.found(&"type unknown".to_string()),
                    }
                } else {
                    Error::UNDEFINED
                        .found(&format!("undefined such an identifier '{}'", ident_name));
                }
            }
            _ => (),
        }
    }
    fn gen_expr(&mut self, n: node::Node) -> usize {
        match n {
            node::Node::UNARY(op, binner, _) => match op {
                Token::MINUS => {
                    let inner: node::Node = unsafe { Box::into_raw(binner).read() };
                    let rr: usize = self.gen_expr(inner.clone());
                    self.hirs.push(HIR::NEGATIVE(rr));
                    rr
                }
                Token::AMPERSAND => {
                    let inner: node::Node = unsafe { Box::into_raw(binner).read() };
                    let rr: usize = self.gen_expr(inner.clone());
                    let ident_name: String = self.get_ident_name(inner);
                    if let Some(var) = self.var_table.get(&ident_name) {
                        self.hirs.push(HIR::ADDRESS(rr, var.stack_offset));
                    } else {
                        Error::UNDEFINED
                            .found(&format!("undefined such an identifier '{}'", ident_name));
                    }
                    rr
                }
                Token::STAR => {
                    let inner: node::Node = unsafe { Box::into_raw(binner).read() };
                    let rr: usize = self.gen_expr(inner.clone());
                    let ident_name: String = self.get_ident_name(inner);
                    if let Some(var) = self.var_table.get(&ident_name) {
                        self.hirs.push(HIR::DEREFERENCE(rr, var.stack_offset));
                    } else {
                        Error::UNDEFINED
                            .found(&format!("undefined such an identifier '{}'", ident_name));
                    }
                    rr
                }
                _ => self.regnum,
            },
            node::Node::BINOP(t, blhs, brhs, _) => {
                let lhs: node::Node = unsafe { Box::into_raw(blhs).read() };
                let rhs: node::Node = unsafe { Box::into_raw(brhs).read() };

                let lr: usize = self.gen_expr(lhs);
                let rr: usize = self.gen_expr(rhs);
                self.gen_binop(t, lr, rr);
                self.regnum -= 1;
                lr
            }
            node::Node::NUMBER(ty) => match ty {
                Type::INTEGER(int_type) => {
                    self.hirs.push(HIR::IMM(self.regnum, int_type.val.unwrap()));
                    let return_reg: usize = self.regnum;
                    self.regnum += 1;
                    return_reg
                }
                _ => self.regnum,
            },
            node::Node::IDENT(ident_name) => {
                if let Some(var) = self.var_table.get(&ident_name) {
                    match &var.ty {
                        Type::INTEGER(int_type) => {
                            self.hirs.push(HIR::LOAD(
                                self.regnum,
                                var.stack_offset,
                                int_type.type_size,
                            ));
                            let return_reg: usize = self.regnum;
                            self.regnum += 1;
                            return_reg
                        }
                        Type::CHAR(char_type) => {
                            self.hirs.push(HIR::LOAD(
                                self.regnum,
                                var.stack_offset,
                                char_type.type_size,
                            ));
                            let return_reg: usize = self.regnum;
                            self.regnum += 1;
                            return_reg
                        }
                        Type::POINTER(_, size) => {
                            self.hirs
                                .push(HIR::LOAD(self.regnum, var.stack_offset, *size));
                            let return_reg: usize = self.regnum;
                            self.regnum += 1;
                            return_reg
                        }
                        _ => {
                            Error::TYPE.found(&"type unknown".to_string());
                            self.regnum
                        }
                    }
                } else {
                    Error::UNDEFINED
                        .found(&format!("undefined such an identifier '{}'", ident_name));
                    self.regnum
                }
            }
            _ => 42,
        }
    }
    fn gen_binop(&mut self, t: Token, lr: usize, rr: usize) {
        match t {
            Token::PLUS => {
                self.hirs.push(HIR::ADD(lr, rr));
            }
            Token::MINUS => {
                self.hirs.push(HIR::SUB(lr, rr));
            }
            Token::STAR => {
                self.hirs.push(HIR::MUL(lr, rr));
            }
            Token::SLASH => {
                self.hirs.push(HIR::DIV(lr, rr));
            }
            Token::PERCENT => {
                self.hirs.push(HIR::MOD(lr, rr));
            }
            Token::LSHIFT => {
                self.hirs.push(HIR::LSHIFT(lr, rr));
            }
            Token::RSHIFT => {
                self.hirs.push(HIR::RSHIFT(lr, rr));
            }
            Token::LT => {
                self.hirs.push(HIR::LT(lr, rr));
            }
            Token::GT => {
                self.hirs.push(HIR::GT(lr, rr));
            }
            Token::LTEQ => {
                self.hirs.push(HIR::LTEQ(lr, rr));
            }
            Token::GTEQ => {
                self.hirs.push(HIR::GTEQ(lr, rr));
            }
            Token::EQ => {
                self.hirs.push(HIR::EQ(lr, rr));
            }
            Token::NTEQ => {
                self.hirs.push(HIR::NTEQ(lr, rr));
            }
            _ => (),
        }
    }
    fn get_ident_name(&self, n: node::Node) -> String {
        if let node::Node::IDENT(ident_name) = n {
            return ident_name;
        }
        Error::TYPE.found(&format!("unexpected '{}'", n.string()));
        "".to_string()
    }
}
