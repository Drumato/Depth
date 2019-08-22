use super::super::ce::types::Error;
use super::super::parse::node::{Func, Node};
use super::super::token::token::Token;
use super::manager::{Manager, Variable};

#[derive(Clone, Eq, PartialEq)]
pub enum Type {
    INTEGER(IntType),
    POINTER(Box<Type>, usize), // type_size
    UNKNOWN,
}
#[derive(Clone, Eq, PartialEq)]
pub struct IntType {
    pub val: Option<i128>,
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
            //Node::UNARY(op,binner,otype),
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
        if ltype != rtype {
            Error::TYPE.found(&format!(
                "difference between {} and {}",
                ltype.string(),
                rtype.string()
            ));
        }
    }
}
