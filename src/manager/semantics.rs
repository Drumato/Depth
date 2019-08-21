use super::super::parse::node::{Func, Node};
use super::super::token::token::Token;
use super::manager::{Manager, Variable};

type Pointer = Option<Box<Type>>;
#[derive(Clone)]
pub enum Type {
    INTEGER(IntType), // INTEGER(INTTYPE) in future
    UNKNOWN,          // INTEGER(INTTYPE) in future
}
#[derive(Clone)]
pub struct IntType {
    pub val: Option<i128>,
    pub type_size: usize,
    pub ptr_to: Pointer,
}

impl Type {
    pub fn string(&self) -> String {
        match self {
            Type::INTEGER(int_type) => format!("INTEGER<{}>", int_type.type_size),
            Type::UNKNOWN => "UNKNOWN".to_string(),
        }
    }
    pub fn from_type(t: Token) -> Type {
        match t {
            Token::I8 => Type::INTEGER(IntType {
                val: None,
                type_size: 1,
                ptr_to: None,
            }),
            Token::I16 => Type::INTEGER(IntType {
                val: None,
                type_size: 2,
                ptr_to: None,
            }),
            Token::I32 => Type::INTEGER(IntType {
                val: None,
                type_size: 4,
                ptr_to: None,
            }),
            Token::I64 => Type::INTEGER(IntType {
                val: None,
                type_size: 8,
                ptr_to: None,
            }),
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
            Node::LET(ident_name, type_name, _) => {
                if let Type::INTEGER(ty) = Type::from_type(type_name.clone()) {
                    self.stack_offset += ty.type_size;
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
}
