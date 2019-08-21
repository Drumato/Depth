use super::super::ir::hi::HIR;
use super::super::parse::node::Func;
use super::super::token::token::Token;
use super::semantics::Type;
use std::collections::HashMap;
pub struct Manager {
    pub functions: Vec<Func>,
    pub hirs: Vec<HIR>,
    pub regnum: usize,
    pub labelnum: usize,
    pub stack_offset: usize,
    pub var_table: HashMap<String, Variable>,
}

pub struct Variable {
    pub name: String,
    pub stack_offset: usize,
    pub ty: Type,
}

impl Variable {
    pub fn new(n: String, offset: usize, ty: Token) -> Variable {
        Variable {
            name: n,
            stack_offset: offset,
            ty: Type::from_type(ty),
        }
    }
    pub fn string(&self) -> String {
        format!(
            "name->{} offset->{} ty->{}",
            self.name,
            self.stack_offset,
            self.ty.string()
        )
    }
}
