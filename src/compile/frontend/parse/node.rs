use super::super::frontmanager::frontmanager::Env;
use super::super::sema::semantics::Type;
use super::super::token::token::Token;
extern crate colored;
use colored::*;
type Name = String;
type Child = Box<Node>;
type Expr = Box<Node>;
type Ary = Box<Node>;
type Idx = Box<Node>;
type Condition = Box<Node>;
type Blk = Box<Node>;
type Alter = Option<Box<Node>>;
type Elements = Box<Vec<Node>>;
#[derive(Clone)]
pub enum Node {
    ADD(Child, Child, Option<Type>),
    SUB(Child, Child, Option<Type>),
    MUL(Child, Child, Option<Type>),
    DIV(Child, Child, Option<Type>),
    MOD(Child, Child, Option<Type>),
    EQ(Child, Child, Option<Type>),
    NTEQ(Child, Child, Option<Type>),
    LT(Child, Child, Option<Type>),
    GT(Child, Child, Option<Type>),
    LTEQ(Child, Child, Option<Type>),
    GTEQ(Child, Child, Option<Type>),
    LSHIFT(Child, Child, Option<Type>),
    RSHIFT(Child, Child, Option<Type>),
    ADDRESS(Child, Option<Type>),
    DEREFERENCE(Child, Option<Type>),
    MINUS(Child, Option<Type>),
    INTEGER(i128),
    IDENT(Name),
    INDEX(Ary, Idx),
    ARRAYLIT(Elements, usize),
    CALL(Name, Elements),
    RETURN(Expr),
    LET(Name, Expr),
    ASSIGN(Name, Expr),
    CONDLOOP(Condition, Blk),
    IF(Condition, Blk, Alter),
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
            Self::MOD(lch, rch, _) => format!("MOD<{},{}>", lch.string(), rch.string()),
            Self::NTEQ(lch, rch, _) => format!("NTEQ<{},{}>", lch.string(), rch.string()),
            Self::EQ(lch, rch, _) => format!("EQ<{},{}>", lch.string(), rch.string()),
            Self::LT(lch, rch, _) => format!("LT<{},{}>", lch.string(), rch.string()),
            Self::GT(lch, rch, _) => format!("GT<{},{}>", lch.string(), rch.string()),
            Self::LTEQ(lch, rch, _) => format!("LTEQ<{},{}>", lch.string(), rch.string()),
            Self::GTEQ(lch, rch, _) => format!("GTEQ<{},{}>", lch.string(), rch.string()),
            Self::LSHIFT(lch, rch, _) => format!("LSHIFT<{},{}>", lch.string(), rch.string()),
            Self::RSHIFT(lch, rch, _) => format!("RSHIFT<{},{}>", lch.string(), rch.string()),
            Self::ADDRESS(ch, _) => format!("ADDRESS<{}>", ch.string()),
            Self::DEREFERENCE(ch, _) => format!("DEREFERENCE<{}>", ch.string()),
            Self::MINUS(ch, _) => format!("MINUS<{}>", ch.string()),
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
            Self::CONDLOOP(cond, stmts) => {
                format!("CONDLOOP<{},{}>", cond.string(), stmts.string())
            }
            Self::IF(cond, stmts, alter) => match alter {
                Some(alt) => format!(
                    "IF<{},{}> ELSE<{}>",
                    cond.string(),
                    stmts.string(),
                    alt.string()
                ),
                None => format!("IF<{},{}>", cond.string(), stmts.string()),
            },

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
