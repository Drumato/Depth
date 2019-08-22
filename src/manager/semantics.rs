use super::super::ce::types::Error;
use super::super::parse::node::{Func, Node};
use super::super::token::token::Token;
use super::manager::{Manager, Variable};

#[derive(Clone, Eq, PartialEq)]
pub enum Type {
    INTEGER(IntType),
    CHAR(CharType),
    POINTER(Box<Type>, usize), // type_size
    UNKNOWN,
}
#[derive(Clone, Eq, PartialEq)]
pub struct IntType {
    pub val: Option<i128>,
    pub type_size: usize,
}
#[derive(Clone, Eq, PartialEq)]
pub struct CharType {
    pub val: Option<char>,
    pub type_size: usize,
}

impl Type {
    pub fn string(&self) -> String {
        match self {
            Type::INTEGER(int_type) => match int_type.type_size {
                1 => "i8".to_string(),
                2 => "i16".to_string(),
                4 => "i32".to_string(),
                _ => "i64".to_string(),
            },
            Type::POINTER(ptr_to, _) => format!("POINTER<{}>", ptr_to.string()),
            Type::CHAR(char_type) => format!("CHAR<{}>", char_type.val.unwrap()),
            Type::UNKNOWN => "UNKNOWN".to_string(),
        }
    }
    pub fn from_type(t: Token) -> Type {
        match t {
            Token::I8 => Type::INTEGER(IntType {
                val: None,
                type_size: 1,
            }),
            Token::I16 => Type::INTEGER(IntType {
                val: None,
                type_size: 2,
            }),
            Token::I32 => Type::INTEGER(IntType {
                val: None,
                type_size: 4,
            }),
            Token::I64 => Type::INTEGER(IntType {
                val: None,
                type_size: 8,
            }),
            Token::POINTER(bptr_to) => {
                let ptr_to: Token = unsafe { Box::into_raw(bptr_to).read() };
                Type::POINTER(Box::new(Type::from_type(ptr_to)), 8)
            }
            _ => Type::UNKNOWN,
        }
    }
}

impl Manager {
    pub fn semantics(&mut self) {
        let func_num: usize = self.functions.len();
        let mut idx: usize = 0;
        loop {
            if idx == func_num {
                break;
            }
            let f: Func = self.functions[idx].clone();
            for n in f.stmts {
                self.walk(n);
            }
            idx += 1;
        }
    }
    fn walk(&mut self, n: Node) -> Type {
        match n {
            Node::IDENT(ident_name) => {
                if let Some(var) = self.var_table.get(&ident_name) {
                    var.ty.clone()
                } else {
                    Type::UNKNOWN
                }
            }
            Node::BINOP(_, blhs, brhs, _) => {
                let lhs: Node = unsafe { Box::into_raw(blhs).read() };
                let rhs: Node = unsafe { Box::into_raw(brhs).read() };
                let ltype: Type = self.walk(lhs);
                let rtype: Type = self.walk(rhs);
                self.check_type(ltype.clone(), rtype);
                ltype
            }
            Node::UNARY(op, binner, _) => {
                let inner: Node = unsafe { Box::into_raw(binner).read() };
                let inner_type: Type = self.walk(inner);
                match op {
                    Token::MINUS => inner_type,
                    Token::AMPERSAND => Type::POINTER(Box::new(inner_type), 8),
                    Token::STAR => {
                        if let Type::POINTER(_, _) = inner_type.clone() {
                            return inner_type;
                        } else {
                            Error::TYPE
                                .found(&format!("can't dereference {}", inner_type.string()));
                        }
                        Type::UNKNOWN
                    }
                    _ => Type::UNKNOWN,
                }
            }
            Node::NUMBER(ty) => ty,
            //Node::RETURN(bstmt),
            //Node::IF(bcond,bstmt),
            Node::LET(ident_name, type_name, bexpr) => {
                let expr: Node = unsafe { Box::into_raw(bexpr).read() };
                match Type::from_type(type_name.clone()) {
                    Type::INTEGER(int_type) => {
                        let expr_type: Type = self.walk(expr);
                        self.check_type(Type::INTEGER(int_type.clone()), expr_type);
                        self.stack_offset += int_type.type_size;
                    }
                    Type::POINTER(binner, type_size) => {
                        let expr_type: Type = self.walk(expr);
                        self.check_type(Type::POINTER(binner, type_size), expr_type);
                        self.stack_offset += type_size;
                    }
                    Type::CHAR(char_type) => {
                        let expr_type: Type = self.walk(expr);
                        self.check_type(Type::CHAR(char_type.clone()), expr_type);
                        self.stack_offset += char_type.type_size;
                    }
                    _ => (),
                }
                self.var_table.insert(
                    ident_name.clone(),
                    Variable::new(ident_name, self.stack_offset, type_name),
                );
                Type::UNKNOWN
            }
            _ => Type::UNKNOWN,
        }
    }
    fn check_type(&self, ltype: Type, rtype: Type) {
        self.check_builtin_type(ltype, rtype);
    }
    fn check_builtin_type(&self, ltype: Type, rtype: Type) {
        match ltype.clone() {
            Type::INTEGER(_) => {
                if let Type::INTEGER(_) = rtype {
                    ();
                } else {
                    Error::TYPE.found(&format!(
                        "difference type between {} and {}",
                        ltype.string(),
                        rtype.string()
                    ));
                }
            }
            Type::POINTER(lbptr_to, _) => {
                if let Type::POINTER(rbptr_to, _) = rtype {
                    let lptr_to: Type = unsafe { Box::into_raw(lbptr_to).read() };
                    let rptr_to: Type = unsafe { Box::into_raw(rbptr_to).read() };
                    self.check_builtin_type(lptr_to, rptr_to);
                } else {
                    Error::TYPE.found(&format!(
                        "difference type between {} and {}",
                        ltype.string(),
                        rtype.string()
                    ));
                }
            }
            Type::CHAR(_) => {
                if let Type::CHAR(_) = rtype {
                    ();
                } else {
                    Error::TYPE.found(&format!(
                        "difference type between {} and {}",
                        ltype.string(),
                        rtype.string()
                    ));
                }
            }
            _ => (),
        }
    }
}
