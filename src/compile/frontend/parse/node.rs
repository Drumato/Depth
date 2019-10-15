use super::super::frontmanager::frontmanager::Env;
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
    ADD(Child, Child),
    SUB(Child, Child),
    MUL(Child, Child),
    DIV(Child, Child),
    MOD(Child, Child),
    EQ(Child, Child),
    NTEQ(Child, Child),
    LT(Child, Child),
    GT(Child, Child),
    LTEQ(Child, Child),
    GTEQ(Child, Child),
    LSHIFT(Child, Child),
    RSHIFT(Child, Child),
    ADDRESS(Child),
    DEREFERENCE(Child),
    MINUS(Child),
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
            Self::ADD(lch, rch) => format!("ADD<{},{}>", lch.string(), rch.string()),
            Self::SUB(lch, rch) => format!("SUB<{},{}>", lch.string(), rch.string()),
            Self::MUL(lch, rch) => format!("MUL<{},{}>", lch.string(), rch.string()),
            Self::DIV(lch, rch) => format!("DIV<{},{}>", lch.string(), rch.string()),
            Self::MOD(lch, rch) => format!("MOD<{},{}>", lch.string(), rch.string()),
            Self::NTEQ(lch, rch) => format!("NTEQ<{},{}>", lch.string(), rch.string()),
            Self::EQ(lch, rch) => format!("EQ<{},{}>", lch.string(), rch.string()),
            Self::LT(lch, rch) => format!("LT<{},{}>", lch.string(), rch.string()),
            Self::GT(lch, rch) => format!("GT<{},{}>", lch.string(), rch.string()),
            Self::LTEQ(lch, rch) => format!("LTEQ<{},{}>", lch.string(), rch.string()),
            Self::GTEQ(lch, rch) => format!("GTEQ<{},{}>", lch.string(), rch.string()),
            Self::LSHIFT(lch, rch) => format!("LSHIFT<{},{}>", lch.string(), rch.string()),
            Self::RSHIFT(lch, rch) => format!("RSHIFT<{},{}>", lch.string(), rch.string()),
            Self::ADDRESS(ch) => format!("ADDRESS<{}>", ch.string()),
            Self::DEREFERENCE(ch) => format!("DEREFERENCE<{}>", ch.string()),
            Self::MINUS(ch) => format!("MINUS<{}>", ch.string()),
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
