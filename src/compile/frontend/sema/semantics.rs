use crate::ce::types::Error;
use crate::compile::frontend;
use frontend::frontmanager::frontmanager::{Env, FrontManager, Symbol};
use frontend::parse::node::{Func, Node};
use frontend::token::token::Token;

use std::collections::BTreeMap;

type ArySize = usize;
type TotalSize = usize;
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
    STRUCT(BTreeMap<String, Symbol>, TotalSize),
}

impl Type {
    pub fn string(&self) -> String {
        match self {
            Self::INTEGER => "INT-TYPE".to_string(),
            Self::POINTER(inner) => format!("POINTER<{}>", inner.string()),
            Self::ARRAY(elem, len) => format!("ARRAY<{},{}>", elem.string(), len),
            Self::ALIAS(alt) => format!("ALIAS<{}>", alt.string()),
            Self::STRUCT(_members, size) => format!("STRUCT<{}>", size),
            Self::UNKNOWN => "UNKNOWN".to_string(),
        }
    }
    pub fn size(&self) -> usize {
        match self {
            Self::INTEGER => 8,
            Self::POINTER(_innter) => 8,
            Self::ARRAY(elem, len) => elem.size() * len,
            Self::ALIAS(alt) => alt.size(),
            Self::STRUCT(_, size) => *size,
            Self::UNKNOWN => {
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
                        Error::UNDEFINED.found(&format!("{} is not defined", name));
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
                    if let Type::ARRAY(_, _) = expr_type {
                        s.ty = Ok(expr_type.clone());
                    } else if let Type::STRUCT(ref mut member_map, ref mut _totalsize) =
                        expr_type.clone()
                    {
                        s.ty = Ok(expr_type.clone());
                        self.stack_offset += s.size();
                        let mut totalsize: usize = 0;
                        for (_member_name, member_s) in member_map.iter_mut() {
                            member_s.stack_offset = self.stack_offset - totalsize;
                            totalsize += member_s.size();
                        }
                    } else {
                        s.ty = Ok(expr_type.clone());
                        self.stack_offset += s.size();
                    }
                    s.stack_offset = self.stack_offset;
                }
                expr_type
            }
            Node::ASSIGN(ident, bexpr) => {
                let expr_type: Type = self.walk(*bexpr.clone());
                if let Some(s) = self.get_symbol(&ident) {
                    if !s.is_mutable {
                        Error::TYPE.found(&format!(
                            "can't assign {} into '{}' it's not mutable",
                            expr_type.string(),
                            ident
                        ));
                    }
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
            Node::MEMBER(ident, member) => {
                let struct_type: Type = self.walk(*ident.clone());
                if let Type::STRUCT(map, _) = struct_type {
                    if let Some(member_s) = map.get(&member) {
                        if let Ok(member_type) = &member_s.ty {
                            return member_type.clone();
                        }
                    }
                }
                Type::UNKNOWN
            }
            Node::ADDRESS(lch) => {
                let ident_node = *lch.clone();
                if let Node::IDENT(_) = &ident_node {
                    Type::POINTER(Box::new(self.walk(ident_node)))
                } else {
                    Error::TYPE.found(&format!("can't address without ident"));
                    Type::UNKNOWN
                }
            }
            Node::DEREFERENCE(lch) => {
                let lch_type: Type = self.walk(*lch.clone());
                if let Type::POINTER(inner) = &lch_type {
                    return *inner.clone();
                }
                Error::TYPE.found(&format!(
                    "can't dereference {} it's not pointer ",
                    lch_type.string(),
                ));
                Type::UNKNOWN
            }
            Node::IDENT(name) => {
                if let Some(s) = self.get_symbol(&name) {
                    if let Ok(ty) = s.ty {
                        if let Type::ALIAS(alt) = ty {
                            *alt
                        } else {
                            ty
                        }
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
            Node::ARRAYLIT(elems, name) => {
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
                if let Some(ref mut array) = self.cur_env.sym_table.get_mut(&name) {
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
            Node::STRUCTLIT(_type_name, members) => {
                let mut total_size: usize = self.stack_offset;
                let mut map: BTreeMap<String, Symbol> = BTreeMap::new();
                for (member_name, member_expr) in members.iter() {
                    let member_type: Type = self.walk(member_expr.clone());
                    total_size += member_type.size();
                    map.insert(
                        member_name.to_string(),
                        Symbol::new(total_size, Ok(member_type), false),
                    );
                }
                Type::STRUCT(map, total_size)
            }
            Node::CALL(func_name, _) => {
                for f in self.functions.iter() {
                    if f.name == func_name {
                        return f.return_type.clone();
                    }
                }
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
