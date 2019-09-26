use super::super::super::manager::manager::Env;
use super::super::sema::semantics::Type;
use super::super::token::token::Token;
extern crate colored;
use colored::*;
type Child = Box<Node>;
#[derive(Clone)]
pub enum Node {
    BINOP(Token, Child, Child, Option<Type>),
    UNARY(Token, Child, Option<Type>),
    NUMBER(Type),
    INDEX(Child, Child),
    CALL(String, Vec<Child>),
    CHARLIT(char),
    ARRAYLIT(Vec<Node>, usize), // expression,number
    IDENT(String),
    RETURN(Child),
    IF(Child, Child, Option<Child>), // condition, block,else
    BLOCK(Vec<Child>),
    LET(String, Token, Child), // ident_name,type,expression
    ASSIGN(String, Child),
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
            Node::ASSIGN(ident_name, expr) => {
                format!("ASSIGN {} \n\t({})", ident_name, expr.string())
            }
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
