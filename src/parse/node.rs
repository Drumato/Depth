use super::super::manager::semantics::Type;
use super::super::token::token::Token;
#[derive(Clone)]
pub enum Node {
    BINOP(Token, Box<Node>, Box<Node>, Option<Type>),
    UNARY(Token, Box<Node>, Option<Type>),
    NUMBER(Type),
    RETURN(Box<Node>),
    IF(Box<Node>, Box<Node>),
    INVALID,
}
impl Node {
    pub fn string(&self) -> String {
        match self {
            Node::BINOP(op, lhs, rhs, _) => {
                format!("{}({}, {})", op.string(), lhs.string(), rhs.string())
            }
            Node::UNARY(op, inner, _) => format!("{}({})", op.string(), inner.string()),
            Node::NUMBER(ty) => match ty {
                Type::INTEGER(val, _, _) => format!("INT-Node<{}>", val),
                _ => format!("UNKNOWN"),
            },
            Node::RETURN(expr) => format!("RETURN({})", expr.string()),
            Node::IF(cond, stmt) => format!("IF({}) ({}) ", cond.string(), stmt.string()),
            Node::INVALID => "INVALID".to_string(),
        }
    }
}

#[derive(Clone)]
pub struct Func {
    pub name: String,
    pub stmts: Vec<Node>,
}
