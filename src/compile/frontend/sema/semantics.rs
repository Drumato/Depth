use super::super::super::super::ce::types::Error;
use super::super::frontmanager::frontmanager::{Env, FrontManager, Symbol};
use super::super::parse::node::{Func, Node};
use super::super::token::token::Token;
type ArySize = usize;
#[derive(Clone, Eq, PartialEq)]
pub enum Type {
    INTEGER, // future INTEGER(,bitsize)
    POINTER(Box<Type>),
    ARRAY(Box<Type>, ArySize),
    UNKNOWN, //DEFTYPE(name,size)
             // ALIAS(Box<Type>)
}

impl Type {
    pub fn string(&self) -> String {
        match self {
            Self::INTEGER => "INT-TYPE".to_string(),
            Self::POINTER(inner) => format!("POINTER<{}>", inner.string()),
            Self::ARRAY(elem, len) => format!("ARRAY<{},{}>", elem.string(), len),
            Self::UNKNOWN => "UNKNOWN".to_string(),
        }
    }
    pub fn size(&self) -> usize {
        match self {
            Type::INTEGER => 8,
            Type::POINTER(_innter) => 8,
            Type::ARRAY(elem, len) => elem.size() * len,
            Type::UNKNOWN => {
                Error::TYPE.found(&"can't known size at compile time".to_string());
                0
            }
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
                if let Node::DEFARG(name) = arg {
                    if let Some(ref mut s) = self.cur_env.sym_table.get_mut(&name) {
                        self.stack_offset += s.size();
                        s.stack_offset = self.stack_offset;
                    } else {
                        Error::TYPE.found(&format!("{} is not defined", name));
                    }
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
            Node::LET(ident_name, bexpr) => {
                let expr_type: Type = self.walk(*bexpr.clone());
                if let Some(ref mut s) = self.cur_env.sym_table.get_mut(&ident_name) {
                    s.ty = Ok(expr_type.clone());
                    if let Type::ARRAY(_, _) = expr_type {
                    } else {
                        self.stack_offset += s.size();
                    }
                    s.stack_offset = self.stack_offset;
                }
                Type::UNKNOWN
            }
            Node::RETURN(bexpr) => {
                let expr_type: Type = self.walk(*bexpr.clone());
                Type::UNKNOWN
            }
            Node::INDEX(rec, ind) => {
                let array_type: Type = self.walk(*rec.clone());
                if let Type::ARRAY(elem_type, _) = array_type {
                    *elem_type.clone()
                } else {
                    Error::TYPE.found(&format!(
                        "can't indexing {} without array type",
                        rec.string()
                    ));
                    Type::UNKNOWN
                }
            }
            Node::IDENT(name) => {
                if let Some(s) = self.get_symbol(&name) {
                    if let Ok(ty) = s.ty {
                        ty
                    } else {
                        eprintln!("not implemented type-checking identifier");
                        Type::UNKNOWN
                    }
                } else {
                    Type::UNKNOWN
                }
            }
            Node::INTEGER(_val) => Type::INTEGER,
            Node::ARRAYLIT(elems, num) => {
                let mut elem_type: Type = Type::UNKNOWN;
                let length: usize = elems.len();
                for elem in elems.iter() {
                    let cur_type: Type = self.walk(elem.clone());
                    if let Type::UNKNOWN = &elem_type {
                        elem_type = cur_type;
                    } else {
                        if elem_type != cur_type {
                            Error::TYPE.found(&format!(
                                "type difference between {} - {} in arraylit",
                                elem_type.string(),
                                cur_type.string()
                            ));
                        }
                        elem_type = cur_type;
                    }
                }
                if let Some(ref mut array) =
                    self.cur_env.sym_table.get_mut(&format!("Array{}", num))
                {
                    self.stack_offset += elem_type.size() * length;
                    array.stack_offset = self.stack_offset;
                    array.ty = Ok(Type::ARRAY(Box::new(elem_type.clone()), length));
                }
                Type::ARRAY(Box::new(elem_type), length)
            }

            _ => Type::UNKNOWN,
        }
    }
    pub fn get_symbol(&self, name: &String) -> Option<Symbol> {
        let mut env: Env = self.cur_env.clone();
        loop {
            if let None = env.prev {
                if let Some(s) = env.sym_table.get(name) {
                    return Some(s.clone());
                }
                return None;
            }
            if let Some(s) = env.sym_table.get(name) {
                return Some(s.clone());
            }
            env = *env.prev.unwrap().clone();
        }
    }
}
