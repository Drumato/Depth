use super::super::manager::semantics::Type;
use super::super::token::token::Token;
pub enum Node {
    BINOP(Token, Box<Node>, Box<Node>, Option<Type>),
    UNARY(Token, Box<Node>, Option<Type>),
    NUMBER(Type),
    RETURN(Box<Node>),
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
            },
            Node::RETURN(expr) => format!("RETURN({})", expr.string()),
            Node::INVALID => "INVALID".to_string(),
        }
    }
}

pub struct Func {
    name: String,
    stmts: Vec<Node>,
}
