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
    pub fn new_rets(ty: TokenType, expr: Node) -> Self {
        Node::new(NodeType::RETS(ty, Box::new(expr)))
    }
    pub fn new_func(name: String, args: Vec<Node>, stmts: Vec<Node>) -> Self {
        Node::new(NodeType::FUNC(name, Box::new(args), Box::new(stmts)))
    }
}

#[derive(Clone, Debug)]
pub enum NodeType {
    INT(Token),                                   //intlit
    ID(String),                                   //identifier
    BINOP(TokenType, Box<Node>, Box<Node>),       //binary-operation
    EX(Box<Node>),                                //expression
    RETS(TokenType, Box<Node>),                   //return statement
    FUNC(String, Box<Vec<Node>>, Box<Vec<Node>>), //func-name,arguments,statements
    INVALID,                                      //invalid ast-node
}

impl NodeType {
    pub fn dump(&self) -> String {
        match self {
            NodeType::INT(v) => format!("{:?}", v),
            NodeType::ID(s) => format!("{}", s),
            NodeType::BINOP(ty, l, r) => {
                format!("type:{:?}\tlchild:{:?}\trchild:{:?}", ty, l.ty, r.ty)
            }
            NodeType::EX(n) => format!("type:EXPRESSION\tchild:{:?}", n.ty),
            NodeType::RETS(ty, n) => format!("type:{:?}\texpression:{:?}", ty, n.ty),
            NodeType::FUNC(name, args, stmts) => {
                format!("name:{:?}\nargs:{:?}\nstmts:{:?}", name, args, stmts)
            }
            _ => format!("Invalid Node"),
        }
    }
}
