use super::super::lex::token;
use std::collections::HashMap;
use token::{Token, TokenType};
#[derive(Clone, Debug, PartialEq)]
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
        Node::new(NodeType::BINOP(ty, vec![lchild], vec![rchild]))
    }

    pub fn new_num(num: Token) -> Self {
        match num.ty {
            TokenType::TkIntlit => Node::new(NodeType::INT(num)),
            TokenType::TkUintlit => Node::new(NodeType::UINT(num)),
            _ => Node::new(NodeType::INVALID),
        }
    }
    pub fn new_string(s: Token) -> Self {
        Node::new(NodeType::STRING(s))
    }
    pub fn new_char(c: Token) -> Self {
        Node::new(NodeType::CHAR(c))
    }
    pub fn new_call(ident: String, nodes: Vec<Node>) -> Self {
        Node::new(NodeType::CALL(ident, nodes))
    }
    pub fn new_ident(ident: String) -> Self {
        Node::new(NodeType::ID(ident))
    }
    pub fn new_rets(ret: TokenType, expr: Node) -> Self {
        Node::new(NodeType::RETS(ret, vec![expr]))
    }
    pub fn new_lets(let_key: TokenType, ident: Vec<Node>, ty: TokenType, expr: Node) -> Self {
        Node::new(NodeType::LETS(let_key, ident, ty, vec![expr]))
    }
    pub fn new_func(
        name: String,
        args: HashMap<String, TokenType>,
        ret: TokenType,
        stmts: Vec<Node>,
    ) -> Self {
        Node::new(NodeType::FUNC(name, args, ret, stmts))
    }
    pub fn new_loops(loop_key: TokenType, stmts: Vec<Node>) -> Self {
        Node::new(NodeType::LOOP(loop_key, stmts))
    }
    pub fn new_fors(
        for_key: TokenType,
        loop_ident: String,
        src_ident: String,
        stmts: Vec<Node>,
    ) -> Self {
        Node::new(NodeType::FOR(for_key, loop_ident, src_ident, stmts))
    }
    pub fn new_ifs(
        if_key: TokenType,
        cond: Node,
        stmts: Vec<Node>,
        else_key: TokenType,
        alt: Vec<Node>,
    ) -> Self {
        Node::new(NodeType::IFS(if_key, vec![cond], stmts, else_key, alt))
    }
    pub fn new_strary(ss: Vec<Token>) -> Self {
        Node::new(NodeType::STRARY(ss))
    }
    pub fn new_intary(is: Vec<Token>) -> Self {
        Node::new(NodeType::INTARY(is))
    }
    pub fn new_charary(cs: Vec<Token>) -> Self {
        Node::new(NodeType::CHARARY(cs))
    }
    pub fn string(&self) -> String {
        format!("{}", self.ty.dump())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum NodeType {
    INT(Token),    //intlit
    UINT(Token),   //uintlit
    STRING(Token), //strlit
    CHAR(Token),   //charlit
    STRARY(Vec<Token>),
    INTARY(Vec<Token>),
    CHARARY(Vec<Token>),
    ID(String),                                                     //identifier
    BINOP(TokenType, Vec<Node>, Vec<Node>),                         //binary-operation
    CALL(String, Vec<Node>),                                        //call-expression
    RETS(TokenType, Vec<Node>),                                     //return statement
    LETS(TokenType, Vec<Node>, TokenType, Vec<Node>),               //let statement
    IFS(TokenType, Vec<Node>, Vec<Node>, TokenType, Vec<Node>),     //if-else statement
    FUNC(String, HashMap<String, TokenType>, TokenType, Vec<Node>), //func-name,arguments,statements
    LOOP(TokenType, Vec<Node>),                                     //loop statement
    FOR(TokenType, String, String, Vec<Node>),                      //for statement
    INVALID,                                                        //invalid ast-node
}

impl NodeType {
    pub fn dump(&self) -> String {
        match self {
            NodeType::INT(_) => "INTLIT".to_string(),
            NodeType::ID(_) => "IDENT".to_string(),
            NodeType::BINOP(ty, _, _) => ty.string().to_string(),
            NodeType::STRING(_) => "STRING".to_string(), //strlit
            NodeType::CHAR(_) => "CHAR".to_string(),     //charlit
            NodeType::RETS(_, _) => "RETURN".to_string(),
            NodeType::FUNC(_, _, _, _) => "FUNC".to_string(),
            _ => "Invalid Node".to_string(),
        }
    }
}
