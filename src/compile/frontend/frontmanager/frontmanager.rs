use super::super::super::super::ce::types::Error;
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
                if let Ok(ty) = &symbol.ty {
                    eprintln!(
                        "{}:offset->{} type->{} mutable->{:?}",
                        name.bold().green(),
                        symbol.stack_offset,
                        ty.string().bold().blue(),
                        symbol.is_mutable
                    );
                } else if let Err(type_t) = &symbol.ty {
                    eprintln!(
                        "{}:offset->{} type->{} mutable->{:?}",
                        name.bold().green(),
                        symbol.stack_offset,
                        type_t.string().bold().blue(),
                        symbol.is_mutable
                    );
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct Env {
    pub sym_table: BTreeMap<String, Symbol>,
    pub type_table: BTreeMap<String, Type>,
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

#[derive(Clone, Eq, PartialEq)]
pub struct Symbol {
    pub stack_offset: usize,
    pub ty: Result<Type, Token>,
    pub is_mutable: bool,
}

impl Symbol {
    pub fn new(offset: usize, res_ty: Result<Type, Token>, flg: bool) -> Self {
        Self {
            stack_offset: offset,
            ty: res_ty,
            is_mutable: flg,
        }
    }
    pub fn size(&self) -> usize {
        match &self.ty {
            Ok(ty) => ty.size(),
            Err(ty_t) => match &ty_t {
                Token::I64 => 8,
                Token::POINTER(_) => 8,
                Token::ARRAY(type_t, array_size) => {
                    if let Token::INTEGER(num) = *array_size.clone() {
                        return Self::new(0, Err(*type_t.clone()), false).size() * num as usize;
                    }
                    Error::TYPE.found(&"can't known size at compile time".to_string());
                    0
                }
                _ => {
                    Error::TYPE.found(&"can't known size at compile time".to_string());
                    0
                }
            },
        }
    }
}
