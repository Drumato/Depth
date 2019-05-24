use super::super::lex::{lexing, token};
use token::{Token, TokenType, TokenVal};
#[derive(Clone, Debug)]
pub struct Node {
    pub ty: NodeType,
}

impl Node {
    pub fn new(op: NodeType) -> Self {
        Self {
            ty: op,
            //ty: Box::new(Type::default()),
        }
    }
    pub fn new_binop(ty: TokenType, lchild: Node, rchild: Node) -> Self {
        Node::new(NodeType::BINOP(ty, Box::new(lchild), Box::new(rchild)))
    }

    pub fn new_num(num: Token) -> Self {
        Node::new(NodeType::INT(num))
    }
}

#[derive(Clone, Debug)]
pub enum NodeType {
    INT(Token),
    ID(String),
    BINOP(TokenType, Box<Node>, Box<Node>),
}

impl NodeType {
    pub fn dump(&self) -> String {
        match self {
            NodeType::INT(v) => format!("{:?}", v),
            NodeType::ID(s) => format!("{}", s),
            NodeType::BINOP(ty, l, r) => {
                format!("type:{:?}\tlchild:{:?}\trchild:{:?}", ty, l.ty, r.ty)
            }
        }
    }
}
