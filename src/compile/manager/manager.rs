use super::super::frontend::parse::node::Func;
use super::super::frontend::token::token::Token;
use super::super::ir::hi::HIR;
use super::semantics::Type;
use std::collections::HashMap;
extern crate colored;
use colored::*;
pub struct Manager {
    pub functions: Vec<Func>,
    pub hirs: Vec<HIR>,
    pub regnum: usize,
    pub labelnum: usize,
    pub stack_offset: usize,
    pub cur_env: Env,
}

#[derive(Clone)]
pub struct Env {
    pub table: HashMap<String, Symbol>,
    pub prev: Option<Box<Env>>,
}
impl Env {
    pub fn new() -> Env {
        Env {
            table: HashMap::new(),
            prev: None,
        }
    }
}

#[derive(Clone)]
pub struct Symbol {
    pub stack_offset: usize,
    pub ty: Type,
}

impl Symbol {
    pub fn new(offset: usize, ty: Token) -> Symbol {
        Symbol {
            stack_offset: offset,
            ty: Type::from_type(ty),
        }
    }
}
