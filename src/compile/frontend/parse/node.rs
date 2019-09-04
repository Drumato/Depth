use super::super::super::manager::manager::Env;
use super::super::super::manager::semantics::Type;
use super::super::token::token::Token;
extern crate colored;
use colored::*;
#[derive(Clone)]
pub enum Node {
    BINOP(Token, Box<Node>, Box<Node>, Option<Type>),
    UNARY(Token, Box<Node>, Option<Type>),
    NUMBER(Type),
    INDEX(Box<Node>, Box<Node>),
    CALL(String, Vec<Box<Node>>),
    CHARLIT(char),
    ARRAYLIT(Vec<Node>, usize), // expression,number
    IDENT(String),
    RETURN(Box<Node>),
    IF(Box<Node>, Box<Node>, Option<Box<Node>>), // condition, block,else
    BLOCK(Vec<Box<Node>>),
    LET(String, Token, Box<Node>), // ident_name,type,expression
    DEFARG(String, Token),
    INVALID,
}
impl Node {
    pub fn string(&self) -> String {
        match self {
            Node::BINOP(op, lhs, rhs, _) => {
                format!("{}({}, {})", op.string(), lhs.string(), rhs.string())
            }
            Node::INDEX(array, expr) => format!("{}[{}]", array.string(), expr.string()),
            Node::UNARY(op, inner, _) => format!("{}({})", op.string(), inner.string()),
            Node::NUMBER(ty) => match ty {
                Type::INTEGER(int_type) => format!("INT-Node<{}>", int_type.val.unwrap()),
                _ => format!("UNKNOWN"),
            },
            Node::RETURN(expr) => format!("RETURN({})", expr.string()),
            Node::IF(cond, stmt, alter) => match alter {
                Some(else_block) => format!(
                    "IF({}) \n\t({}) ELSE \n\t({})",
                    cond.string(),
                    stmt.string(),
                    else_block.string(),
                ),
                None => format!("IF({}) \n\t({}) ", cond.string(), stmt.string()),
            },
            Node::BLOCK(bstmts) => {
                let stmts: String = bstmts
                    .into_iter()
                    .map(|st| "\n\t".to_owned() + &st.string())
                    .collect::<String>();
                format!("BLOCK({})", stmts)
            }
            Node::CALL(ident, bargs) => {
                let args: String = bargs
                    .into_iter()
                    .map(|a| a.string() + ",")
                    .collect::<String>();
                format!("CALL {}({})", ident, args)
            }
            Node::INVALID => "INVALID".to_string(),
            Node::LET(ident_name, ty, expr) => format!(
                "LET {} <- {} \n\t({})",
                ident_name,
                ty.string(),
                expr.string()
            ),
            Node::CHARLIT(char_val) => format!("CHARLIT<{}>", char_val),
            Node::ARRAYLIT(elems, _) => {
                let elems_string: String = elems
                    .into_iter()
                    .map(|b| b.string() + ",")
                    .collect::<String>();
                format!("ARRAY({})", elems_string)
            }
            Node::IDENT(ident_name) => format!("IDENT<{}>", ident_name),
            Node::DEFARG(arg, ty) => format!("DEFARG<{},{}>", arg, ty.string()),
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
    for fu in funcs.iter() {
        eprintln!("{}'s stmts:", fu.name);
        for n in fu.stmts.iter() {
            eprintln!("{}", n.string().green().bold());
        }
    }
}
