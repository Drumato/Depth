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

impl Manager {
    pub fn new(funcs: Vec<Func>) -> Manager {
        Manager {
            functions: funcs,
            hirs: Vec::new(),
            regnum: 0,
            labelnum: 0,
            stack_offset: 0,
            cur_env: Env::new(),
        }
    }
    pub fn dump_symbol(&self) {
        eprintln!("{}", "--------symbol_table--------".green().bold());
        for f in self.functions.iter() {
            eprintln!("{}'s symbols:", f.name);
            for (name, symbol) in f.env.table.iter() {
                eprintln!(
                    "{}:offset->{} type->{}",
                    name.bold().green(),
                    symbol.stack_offset,
                    symbol.ty.string().bold().blue()
                );
            }
        }
    }
    pub fn dump_hir(&self) {
        eprintln!("{}", "--------dumphir--------".blue().bold());
        for ir in self.hirs.iter() {
            eprintln!("{}", ir.string().green().bold());
        }
    }
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
