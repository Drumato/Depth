use super::super::token::token::Token;
pub enum Node {
    BINOP(Token, Box<Node>, Box<Node>),
    UNARY(Token, Box<Node>),
    INTEGER(i128),
    INVALID,
}

impl Node {
    pub fn string(&self) -> String {
        match self {
            Node::BINOP(op, lhs, rhs) => {
                format!("{}({}, {})", op.string(), lhs.string(), rhs.string())
            }
            Node::UNARY(op, inner) => format!("{}({})", op.string(), inner.string()),
            Node::INTEGER(int) => format!("INT-Node<{}>", int),
            Node::INVALID => "INVALID".to_string(),
        }
    }
}

pub struct Func {
    name: String,
    stmts: Vec<Node>,
}
