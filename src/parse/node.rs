use super::super::lex::{lexing, token};
use std::collections::HashMap;
use token::{Token, TokenType, TokenVal};
#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    pub ty: NodeType,
    pub id: u64,
}

impl Node {
    pub fn new(op: NodeType, id: u64) -> Self {
        Self {
            ty: op,
            //ty: Box::new(Type::default()),
            id: id,
        }
    }
    pub fn new_binop(ty: TokenType, lchild: Node, rchild: Node, id: u64, ids: Vec<u64>) -> Self {
        Node::new(NodeType::BINOP(ty, vec![lchild], vec![rchild], ids), id)
    }

    pub fn new_num(num: Token, id: u64) -> Self {
        match num.ty {
            TokenType::TkIntlit => Node::new(NodeType::INT(num), id),
            TokenType::TkUintlit => Node::new(NodeType::UINT(num), id),
            _ => Node::new(NodeType::INVALID, 0),
        }
    }
    pub fn new_string(s: Token, id: u64) -> Self {
        Node::new(NodeType::STRING(s), id)
    }
    pub fn new_char(c: Token, id: u64) -> Self {
        Node::new(NodeType::CHAR(c), id)
    }
    pub fn new_call(ident: String, nodes: Vec<Node>, id: u64) -> Self {
        Node::new(NodeType::CALL(ident, nodes), id)
    }
    pub fn new_ident(ident: String, id: u64) -> Self {
        Node::new(NodeType::ID(ident), id)
    }
    pub fn new_rets(ret: TokenType, expr: Node, id: u64) -> Self {
        Node::new(NodeType::RETS(ret, vec![expr]), id)
    }
    pub fn new_lets(let_key: TokenType, ident: String, ty: TokenType, expr: Node, id: u64) -> Self {
        Node::new(NodeType::LETS(let_key, ident, ty, vec![expr]), id)
    }
    pub fn new_func(
        name: String,
        args: HashMap<String, TokenType>,
        ret: TokenType,
        stmts: Vec<Node>,
        id: u64,
        ids: Vec<u64>,
    ) -> Self {
        Node::new(NodeType::FUNC(name, args, ret, stmts, ids), id)
    }
    pub fn new_loops(loop_key: TokenType, stmts: Vec<Node>, id: u64, ids: Vec<u64>) -> Self {
        Node::new(NodeType::LOOP(loop_key, stmts, ids), id)
    }
    pub fn new_fors(
        for_key: TokenType,
        loop_ident: String,
        src_ident: String,
        stmts: Vec<Node>,
        id: u64,
        ids: Vec<u64>,
    ) -> Self {
        Node::new(
            NodeType::FOR(for_key, loop_ident, src_ident, stmts, ids),
            id,
        )
    }
    pub fn new_ifs(
        if_key: TokenType,
        cond: Node,
        stmts: Vec<Node>,
        else_key: TokenType,
        alt: Vec<Node>,
        id: u64,
        ids: Vec<u64>,
    ) -> Self {
        Node::new(
            NodeType::IFS(if_key, vec![cond], stmts, else_key, alt, ids),
            id,
        )
    }
    pub fn new_strary(ss: Vec<Token>, id: u64) -> Self {
        Node::new(NodeType::STRARY(ss), id)
    }
    pub fn new_intary(is: Vec<Token>, id: u64) -> Self {
        Node::new(NodeType::INTARY(is), id)
    }
    pub fn new_charary(cs: Vec<Token>, id: u64) -> Self {
        Node::new(NodeType::CHARARY(cs), id)
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
    ID(String),                                       //identifier
    BINOP(TokenType, Vec<Node>, Vec<Node>, Vec<u64>), //binary-operation
    EX(Vec<Node>),                                    //expression
    CALL(String, Vec<Node>),                          //call-expression
    RETS(TokenType, Vec<Node>),                       //return statement
    LETS(TokenType, String, TokenType, Vec<Node>),    //let statement
    IFS(
        TokenType,
        Vec<Node>,
        Vec<Node>,
        TokenType,
        Vec<Node>,
        Vec<u64>,
    ), //if-else statement
    FUNC(
        String,
        HashMap<String, TokenType>,
        TokenType,
        Vec<Node>,
        Vec<u64>,
    ), //func-name,arguments,statements
    LOOP(TokenType, Vec<Node>, Vec<u64>),             //loop statement
    FOR(TokenType, String, String, Vec<Node>, Vec<u64>), //for statement
    INVALID,                                          //invalid ast-node
}

impl NodeType {
    pub fn dump(&self) -> String {
        match self {
            NodeType::INT(v) => "INTLIT".to_string(),
            NodeType::ID(s) => "IDENT".to_string(),
            NodeType::BINOP(ty, _, _, _) => ty.string().to_string(),
            NodeType::STRING(_) => "STRING".to_string(), //strlit
            NodeType::CHAR(_) => "CHAR".to_string(),     //charlit
            NodeType::EX(n) => n[0].string().to_string(),
            NodeType::RETS(_, _) => "RETURN".to_string(),
            NodeType::FUNC(_, _, _, _, _) => "FUNC".to_string(),
            _ => "Invalid Node".to_string(),
        }
    }
}
