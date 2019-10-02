use super::super::super::super::ce::types::Error;
use super::super::frontmanager::frontmanager::FrontManager;
use super::super::parse::node::{Func, Node};
use super::super::token::token::Token;

#[derive(Clone, Eq, PartialEq)]
pub enum Type {
    INTEGER(IntType),
    CHAR(Option<char>),
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
            Type::CHAR(ochar_val) => match ochar_val {
                Some(char_val) => format!("CHAR<{}>", char_val),
                None => "CHAR".to_string(),
            },
            Type::UNKNOWN => "UNKNOWN".to_string(),
        }
    }
    pub fn size(&self) -> usize {
        match self {
            Type::INTEGER(int_type) => int_type.type_size,
            Type::POINTER(_, size) => *size,
            Type::CHAR(_) => 4,
            Type::UNKNOWN => 42,
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
            Token::CHAR => Type::CHAR(None),
            _ => Type::UNKNOWN,
        }
    }
}

impl FrontManager {
    pub fn semantics(&mut self) {
        let func_num: usize = self.functions.len();
        let mut idx: usize = 0;
        loop {
            if idx == func_num {
                break;
            }
            let f: Func = self.functions[idx].clone();
            self.cur_env = f.env.clone();
            for arg in f.args {
                match arg {
                    Node::DEFARG(arg_name, ty) => {
                        self.stack_offset += Type::from_type(ty.clone()).size();
                        if let Some(ref mut arg) = self.cur_env.table.get_mut(&arg_name) {
                            arg.stack_offset = self.stack_offset;
                        } else {
                            eprintln!("{} can't attaching the stack.", arg_name);
                        }
                    }
                    _ => (),
                }
            }
            for n in f.stmts {
                self.walk(n);
            }
            self.functions[idx].env = self.cur_env.clone();
            idx += 1;
        }
    }
    fn walk(&mut self, n: Node) -> Type {
        match n {
            Node::IDENT(ident_name) => {
                if let Some(var) = self.cur_env.table.get(&ident_name) {
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
            Node::CHARLIT(char_val) => Type::CHAR(Some(char_val)),
            Node::RETURN(bstmt) => {
                let stmt: Node = unsafe { Box::into_raw(bstmt).read() };
                self.walk(stmt);
                Type::UNKNOWN
            }
            Node::ASSIGN(ident_name, _bexpr) => {
                if let Some(ref mut symbol) = self.cur_env.table.get_mut(&ident_name) {
                    if !symbol.is_mutable {
                        Error::TYPE.found(&format!("'{}' is defined as immutable", &ident_name));
                    }
                    symbol.ty.clone()
                } else {
                    Error::TYPE.found(&format!("'{}' is not defined yet", &ident_name));
                    return Type::UNKNOWN;
                }
            }
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
                    Type::CHAR(ochar_val) => {
                        let expr_type: Type = self.walk(expr);
                        self.check_type(Type::CHAR(ochar_val), expr_type);
                        self.stack_offset += 4;
                    }
                    _ => (),
                }
                if let Some(ref mut symbol) = self.cur_env.table.get_mut(&ident_name) {
                    symbol.stack_offset = self.stack_offset;
                }
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
