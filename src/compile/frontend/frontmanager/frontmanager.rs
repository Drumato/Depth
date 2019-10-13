use super::super::super::ir::tac::Tac;
use super::super::parse::node::Func;
use super::super::sema::semantics::Type;
use super::super::token::token::Token;
use std::collections::BTreeMap;
extern crate colored;
use colored::*;
pub struct FrontManager {
    pub functions: Vec<Func>,
    pub stack_offset: usize,
    pub cur_env: Env,
    pub tacs: Vec<Tac>,
    pub virt: usize,
    pub label: usize,
}

impl FrontManager {
    pub fn new(funcs: Vec<Func>) -> FrontManager {
        FrontManager {
            functions: funcs,
            stack_offset: 0,
            cur_env: Env::new(),
            tacs: Vec::new(),
            virt: 0,
            label: 0,
        }
    }
    pub fn dump_symbol(&self) {
        eprintln!("{}", "--------symbol_table--------".green().bold());
        for f in self.functions.iter() {
            eprintln!("{}'s symbols:", f.name);
            for (name, symbol) in f.env.sym_table.iter() {
                eprintln!(
                    "{}:offset->{} type->{} mutable->{:?}",
                    name.bold().green(),
                    symbol.stack_offset,
                    symbol.ty.string().bold().blue(),
                    symbol.is_mutable
                );
            }
        }
    }
}

#[derive(Clone)]
pub struct Env {
    pub sym_table: BTreeMap<String, Symbol>,
    pub type_table: BTreeMap<String, DefType>,
    pub prev: Option<Box<Env>>,
}
impl Env {
    pub fn new() -> Env {
        Env {
            sym_table: BTreeMap::new(),
            type_table: BTreeMap::new(),
            prev: None,
        }
    }
}

#[derive(Clone)]
pub struct Symbol {
    pub stack_offset: usize,
    pub ty: Type,
    pub is_mutable: bool,
}

impl Symbol {
    pub fn new(offset: usize, ty: Token, mutable: bool) -> Symbol {
        Symbol {
            stack_offset: offset,
            ty: Type::from_type(ty),
            is_mutable: mutable,
        }
    }
}

#[derive(Clone)]
pub struct DefType {
    pub alias: Option<Type>,
    pub members: BTreeMap<String, Symbol>,
    pub size: usize,
}

impl DefType {
    pub fn new(ty: Option<Type>) -> Self {
        if let Some(t) = ty {
            return Self {
                alias: Some(t.clone()),
                size: t.size(),
                members: BTreeMap::new(),
            };
        }
        Self {
            alias: None,
            size: 0,
            members: BTreeMap::new(),
        }
    }
    pub fn new_struct(member_map: BTreeMap<String, Symbol>) -> Self {
        Self {
            alias: None,
            size: 0,
            members: member_map,
        }
    }
}
