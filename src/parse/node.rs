use super::super::token::token::Token;
pub enum Node {
    BINOP(Token, Box<Node>, Box<Node>),
    INTEGER(i128),
    INVALID,
}

impl Node {
    pub fn string(&self) -> String {
        match self {
            Node::BINOP(t, lhs, rhs) => {
                format!("{}({}, {})", t.string(), lhs.string(), rhs.string())
            }
            Node::INTEGER(int) => format!("INT-Node<{}>", int),
            Node::INVALID => "INVALID".to_string(),
        }
    }
}

pub struct Func {
    name: String,
    stmts: Vec<Node>,
}
