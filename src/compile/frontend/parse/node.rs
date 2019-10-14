use super::super::frontmanager::frontmanager::Env;
use super::super::sema::semantics::Type;
use super::super::token::token::Token;
extern crate colored;
use colored::*;
type Name = String;
type Child = Box<Node>;
type Elements = Box<Vec<Node>>;
#[derive(Clone)]
pub enum Node {
    ADD(Child, Child, Option<Type>),
    SUB(Child, Child, Option<Type>),
    MUL(Child, Child, Option<Type>),
    DIV(Child, Child, Option<Type>),
    ADDRESS(Child, Option<Type>),
    DEREFERENCE(Child, Option<Type>),
    INTEGER(i128),
    IDENT(Name),
    INDEX(Child, Child),
    ARRAYLIT(Elements, usize),
    CALL(Name, Elements),
    RETURN(Child),
    LET(Name, Child),
    ASSIGN(Name, Child),
    BLOCK(Elements),
    DEFARG(Name),
    INVALID,
}
impl Node {
    pub fn string(&self) -> String {
        match self {
            Self::ADD(lch, rch, _) => format!("ADD<{},{}>", lch.string(), rch.string()),
            Self::SUB(lch, rch, _) => format!("SUB<{},{}>", lch.string(), rch.string()),
            Self::MUL(lch, rch, _) => format!("MUL<{},{}>", lch.string(), rch.string()),
            Self::DIV(lch, rch, _) => format!("DIV<{},{}>", lch.string(), rch.string()),
            Self::ADDRESS(ch, _) => format!("ADDRESS<{}>", ch.string()),
            Self::DEREFERENCE(ch, _) => format!("DEREFERENCE<{}>", ch.string()),
            Self::INTEGER(val) => format!("INTEGER<{}>", val),
            Self::IDENT(name) => format!("IDENT<{}>", name),
            Self::INDEX(rec, ind) => format!("INDEX<{},{}>", rec.string(), ind.string()),
            Self::RETURN(expr) => format!("RETURN({})", expr.string()),
            Self::LET(ident, expr) => format!("LET<{}>({})", ident, expr.string()),
            Self::ASSIGN(ident, expr) => format!("ASSIGN<{}>({})", ident, expr.string()),
            Self::BLOCK(stmts) => format!("BLOCK<{} stmts>", stmts.len()),
            Self::CALL(ident, _args) => format!("CALL<{}>", ident),
            Self::ARRAYLIT(_elems, num) => format!("ARRAYLIT<{}>", num),
            Self::DEFARG(name) => format!("DEFARG<{}>", name),

            _ => "INVALID".to_string(),
        }
    }
}

#[derive(Clone)]
pub struct Func {
    pub name: String,
    pub stmts: Vec<Node>,
    pub args: Vec<Node>,
    pub env: Env,
}

pub fn dump_ast(funcs: &Vec<Func>) {
    eprintln!("{}", "--------dumpast--------".blue().bold());
    for f in funcs.iter() {
        eprintln!("{}'s stmts:", f.name);
        for st in f.stmts.iter() {
            eprintln!("\t{}", st.string().green().bold());
        }
    }
}
