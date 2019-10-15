use super::super::super::super::ce::types::Error;
use super::super::frontmanager::frontmanager::{Env, FrontManager, Symbol};
use super::super::parse::node::{Func, Node};
use super::super::token::token::Token;
type ArySize = usize;
type PointerTo = Box<Type>;
type Elem = Box<Type>;
type Alias = Box<Type>;
#[derive(Clone, Eq, PartialEq)]
pub enum Type {
    INTEGER, // future INTEGER(,bitsize)
    POINTER(PointerTo),
    ARRAY(Elem, ArySize),
    UNKNOWN, //DEFTYPE(name,size)
    ALIAS(Alias),
}

impl Type {
    pub fn string(&self) -> String {
        match self {
            Self::INTEGER => "INT-TYPE".to_string(),
            Self::POINTER(inner) => format!("POINTER<{}>", inner.string()),
            Self::ARRAY(elem, len) => format!("ARRAY<{},{}>", elem.string(), len),
            Self::ALIAS(alt) => format!("ALIAS<{}>", alt.string()),
            Self::UNKNOWN => "UNKNOWN".to_string(),
        }
    }
    pub fn size(&self) -> usize {
        match self {
            Type::INTEGER => 8,
            Type::POINTER(_innter) => 8,
            Type::ARRAY(elem, len) => elem.size() * len,
            Type::ALIAS(alt) => alt.size(),
            Type::UNKNOWN => {
                Error::TYPE.found(&"can't known size at compile time".to_string());
                0
            }
        }
    }
    pub fn from_token(type_t: Token) -> Self {
        match type_t {
            Token::I64 => Type::INTEGER,
            Token::POINTER(inner) => {
                let inner_type: Type = Self::from_token(*inner.clone());
                Self::POINTER(Box::new(inner_type))
            }
            Token::ARRAY(elem, size) => {
                let elem_type: Type = Self::from_token(*elem.clone());
                if let Token::INTEGER(ary_size) = *size.clone() {
                    return Self::ARRAY(Box::new(elem_type), ary_size as usize);
                }
                Error::TYPE.found(&"array size is known at compile time.".to_string());
                Self::UNKNOWN
            }
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
                if let Node::DEFARG(name) = arg {
                    if let Some(ref mut s) = self.cur_env.sym_table.get_mut(&name) {
                        let res_ty = s.ty.clone();
                        if let Err(type_t) = res_ty {
                            s.ty = Ok(Type::from_token(type_t));
                        }
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
                expr_type
            }
            Node::RETURN(bexpr) => {
                let expr_type: Type = self.walk(*bexpr.clone());
                expr_type
            }
            Node::INDEX(rec, ind) => {
                let array_type: Type = self.walk(*rec.clone());
                let index_type: Type = self.walk(*ind.clone());
                if let Type::INTEGER = &index_type {
                } else {
                    Error::TYPE.found(&format!(
                        "must be integer-type in index but got {}",
                        index_type.string()
                    ));
                }

                if let Type::ARRAY(elem_type, _) = array_type {
                    *elem_type.clone()
                } else {
                    Error::TYPE.found(&format!("can't indexing {} it's not array ", rec.string()));
                    Type::UNKNOWN
                }
            }
            Node::ADDRESS(lch) => {
                let lch_type: Type = self.walk(*lch.clone());
                Type::POINTER(Box::new(lch_type))
            }
            Node::DEREFERENCE(lch) => {
                let lch_type: Type = self.walk(*lch.clone());
                if let Type::POINTER(inner) = &lch_type {
                    return *inner.clone();
                }
                Error::TYPE.found(&format!(
                    "can't dereferecne {} it's not pointer ",
                    lch_type.string(),
                ));
                Type::UNKNOWN
            }
            Node::IDENT(name) => {
                if let Some(s) = self.get_symbol(&name) {
                    if let Ok(ty) = s.ty {
                        ty
                    } else if let Err(type_t) = s.ty {
                        Type::from_token(type_t)
                    } else {
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
            Node::ADD(lch, rch)
            | Node::SUB(lch, rch)
            | Node::MUL(lch, rch)
            | Node::DIV(lch, rch)
            | Node::MOD(lch, rch)
            | Node::EQ(lch, rch)
            | Node::NTEQ(lch, rch)
            | Node::LT(lch, rch)
            | Node::GT(lch, rch)
            | Node::LTEQ(lch, rch)
            | Node::GTEQ(lch, rch)
            | Node::LSHIFT(lch, rch)
            | Node::RSHIFT(lch, rch) => {
                let lch_type: Type = self.walk(*lch.clone());
                let rch_type: Type = self.walk(*rch.clone());
                if lch_type != lch_type {
                    Error::TYPE.found(&format!(
                        "type difference between {} - {} ",
                        lch_type.string(),
                        rch_type.string()
                    ));
                    return Type::UNKNOWN;
                }
                lch_type
            }
            Node::MINUS(lch) => {
                let lch_type: Type = self.walk(*lch.clone());
                if let Type::INTEGER = &lch_type {
                    return Type::INTEGER;
                }
                Error::TYPE.found(&format!(
                    "can't negative {} it's not integer ",
                    lch_type.string(),
                ));
                Type::UNKNOWN
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
