#[allow(mutable_borrow_reservation_conflict)]
use super::super::super::ce::types::Error;
use super::super::frontend::parse::node;
use super::super::frontend::token::token::Token;
use super::super::ir::hi::HIR;
use super::manager::{Manager, Symbol};
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
            self.cur_env = f.env;
            self.hirs.push(HIR::SYMBOL(f.name.clone()));
            for arg in f.args.iter() {
                if let node::Node::DEFARG(_, ty) = arg {
                    self.stack_offset -= Type::from_type(ty.clone()).size();
                }
            }
            self.hirs.push(HIR::PROLOGUE(self.stack_offset));
            for (idx, _arg) in f.args.iter().enumerate() {
                self.hirs.push(HIR::PUSHARG(idx));
            }
            self.regnum = 0;
            for n in f.stmts {
                self.gen_stmt(n);
            }
            if f.name == "main" {
                self.hirs.push(HIR::EPILOGUE);
            }
            idx += 1;
        }
    }
    fn gen_stmt(&mut self, n: node::Node) {
        match n {
            node::Node::RETURN(bexpr) => {
                let expr: node::Node = unsafe { Box::into_raw(bexpr).read() };
                let _expr_reg: usize = self.gen_expr(expr);
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
                let var_type: &Type = &self.get_var(&ident_name).unwrap().ty;
                let offset: usize = self.get_var(&ident_name).unwrap().stack_offset;
                match var_type {
                    Type::INTEGER(int_type) => {
                        let size: usize = int_type.type_size;
                        self.hirs.push(HIR::STORE(offset, expr_reg, size))
                    }
                    Type::CHAR(_) => {
                        self.hirs.push(HIR::STORE(offset, expr_reg, 4));
                    }
                    Type::POINTER(_, size) => {
                        let size: usize = *size;
                        self.hirs.push(HIR::STORE(offset, expr_reg, size));
                    }
                    Type::ARRAY(_, _, _) => {}
                    _ => Error::TYPE.found(&"type unknown".to_string()),
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
                    let offset: usize = self.get_var(&ident_name).unwrap().stack_offset;
                    self.hirs.push(HIR::ADDRESS(rr, offset));
                    rr
                }
                Token::STAR => {
                    let inner: node::Node = unsafe { Box::into_raw(binner).read() };
                    let rr: usize = self.gen_expr(inner.clone());
                    let ident_name: String = self.get_ident_name(inner);
                    let offset: usize = self.get_var(&ident_name).unwrap().stack_offset;
                    self.hirs.push(HIR::DEREFERENCE(rr, offset));
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
            node::Node::CALL(func_name, bargs) => {
                let mut regs: Vec<usize> = vec![0; bargs.len()];
                for (idx, barg) in bargs.iter().enumerate() {
                    let arg: node::Node = unsafe { Box::into_raw(barg.clone()).read() };
                    regs[idx] = self.gen_expr(arg);
                }
                if regs.len() > 0 {
                    self.regnum -= regs.len() - 1;
                }
                let expr_reg: usize = self.regnum;
                self.hirs.push(HIR::CALL(func_name, regs, Some(expr_reg)));
                self.regnum += 1;
                expr_reg
            }
            node::Node::INDEX(barray, bexpr) => {
                let array: node::Node = unsafe { Box::into_raw(barray).read() };
                let offset: usize = match array.clone() {
                    node::Node::IDENT(name) => self.get_var(&name).unwrap().stack_offset,
                    node::Node::ARRAYLIT(_, num) => {
                        self.get_var(&format!("Array{}", num)).unwrap().stack_offset
                    }
                    _ => 42,
                };
                self.gen_expr(array);
                let address_reg: usize = self.regnum;
                self.hirs.push(HIR::ADDRESS(address_reg, offset));
                self.regnum += 1;
                let expr: node::Node = unsafe { Box::into_raw(bexpr).read() };
                if let node::Node::NUMBER(Type::INTEGER(int_type)) = expr {
                    self.hirs.push(HIR::INDEXLOAD(
                        self.regnum,
                        address_reg,
                        int_type.val.unwrap(),
                        int_type.type_size,
                    ));
                    self.regnum += 1;
                    return self.regnum - 1;
                }
                let expr_reg: usize = self.gen_expr(expr);
                expr_reg
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
            node::Node::CHARLIT(char_val) => {
                self.hirs.push(HIR::IMMCHAR(self.regnum, char_val));
                let return_reg: usize = self.regnum;
                self.regnum += 1;
                return_reg
            }
            node::Node::ARRAYLIT(elems, num) => {
                let mut total_size: usize = 0;
                let length: usize = elems.len();
                for elem in elems {
                    let elem_size: usize = match &elem {
                        node::Node::BINOP(_, _, _, otype) => otype.clone().unwrap().size(),
                        node::Node::UNARY(_, _, otype) => otype.clone().unwrap().size(),
                        node::Node::NUMBER(ty) => ty.size(),
                        _ => 0,
                    };
                    let expr_reg: usize = self.gen_expr(elem);
                    let offset: usize =
                        self.get_var(&format!("Array{}", num)).unwrap().stack_offset;
                    self.hirs
                        .push(HIR::STORE(offset - total_size, expr_reg, elem_size));
                    total_size += elem_size;
                }
                self.regnum -= length - 1;
                self.regnum
            }
            node::Node::IDENT(ident_name) => {
                let offset: usize = self.get_var(&ident_name).unwrap().stack_offset;
                let var_type: &Type = &self.get_var(&ident_name).unwrap().ty;
                let size: usize = var_type.size();
                match var_type {
                    Type::INTEGER(_) => {
                        self.hirs.push(HIR::LOAD(self.regnum, offset, size));
                        let return_reg: usize = self.regnum;
                        self.regnum += 1;
                        return_reg
                    }
                    Type::CHAR(_) => {
                        self.hirs.push(HIR::LOAD(self.regnum, offset, size));
                        let return_reg: usize = self.regnum;
                        self.regnum += 1;
                        return_reg
                    }
                    Type::POINTER(_, _) => {
                        self.hirs.push(HIR::LOAD(self.regnum, offset, size));
                        let return_reg: usize = self.regnum;
                        self.regnum += 1;
                        return_reg
                    }
                    Type::ARRAY(_, _, _) => {
                        self.hirs.push(HIR::LOAD(self.regnum, offset, 8));
                        let return_reg: usize = self.regnum;
                        self.regnum += 1;
                        return_reg
                    }
                    _ => {
                        Error::TYPE.found(&"type unknown".to_string());
                        self.regnum
                    }
                }
            }
            _ => self.regnum,
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
        match n {
            node::Node::IDENT(ident_name) => ident_name,
            node::Node::UNARY(_, binner, _) => {
                let mut inner: node::Node = unsafe { Box::into_raw(binner).read() };
                while let node::Node::UNARY(_, binners, _) = inner {
                    inner = unsafe { Box::into_raw(binners).read() };
                }
                self.get_ident_name(inner)
            }
            _ => {
                Error::TYPE.found(&format!("unexpected '{}'", n.string()));
                "".to_string()
            }
        }
    }
    fn get_var(&self, ident_name: &String) -> Option<&Symbol> {
        if let Some(var) = self.cur_env.table.get(ident_name) {
            return Some(var);
        }
        Error::UNDEFINED.found(&format!("undefined such an identifier '{}'", ident_name));
        None
    }
}
