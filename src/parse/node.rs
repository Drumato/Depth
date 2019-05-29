use super::super::lex::{lexing, token};
use std::collections::HashMap;
use token::{Token, TokenType, TokenVal};
#[derive(Clone, Debug)]
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
    pub fn new_binop(ty: TokenType, lchild: Node, rchild: Node, id: u64) -> Self {
        Node::new(NodeType::BINOP(ty, Box::new(lchild), Box::new(rchild)), id)
    }

    pub fn new_num(num: Token, id: u64) -> Self {
        match num.ty {
            TokenType::TkIntlit => Node::new(NodeType::INT(num), id),
            TokenType::TkUintlit => Node::new(NodeType::UINT(num), id),
            _ => Node::new(NodeType::INVALID, 0),
        }
    }
    pub fn new_ident(ident: String, id: u64) -> Self {
        Node::new(NodeType::ID(ident), id)
    }
    pub fn new_rets(ret: TokenType, expr: Node, id: u64) -> Self {
        Node::new(NodeType::RETS(ret, Box::new(expr)), id)
    }
    pub fn new_lets(let_key: TokenType, ident: String, ty: TokenType, expr: Node, id: u64) -> Self {
        Node::new(NodeType::LETS(let_key, ident, ty, Box::new(expr)), id)
    }
    pub fn new_func(
        name: String,
        args: HashMap<String, TokenType>,
        ret: TokenType,
        stmts: Vec<Node>,
        id: u64,
    ) -> Self {
        Node::new(NodeType::FUNC(name, args, ret, Box::new(stmts)), id)
    }
    pub fn new_loops(loop_key: TokenType, stmts: Vec<Node>, id: u64) -> Self {
        Node::new(NodeType::LOOP(loop_key, Box::new(stmts)), id)
    }
    pub fn new_fors(
        for_key: TokenType,
        loop_ident: String,
        src_ident: String,
        stmts: Vec<Node>,
        id: u64,
    ) -> Self {
        Node::new(
            NodeType::FOR(for_key, loop_ident, src_ident, Box::new(stmts)),
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
    ) -> Self {
        Node::new(
            NodeType::IFS(
                if_key,
                Box::new(cond),
                Box::new(stmts),
                else_key,
                Box::new(alt),
            ),
            id,
        )
    }
    pub fn new_strary(ss: Vec<Token>, id: u64) -> Self {
        Node::new(NodeType::STRARY(Box::new(ss)), id)
    }
    pub fn new_intary(is: Vec<Token>, id: u64) -> Self {
        Node::new(NodeType::INTARY(Box::new(is)), id)
    }
    pub fn new_charary(cs: Vec<Token>, id: u64) -> Self {
        Node::new(NodeType::CHARARY(Box::new(cs)), id)
    }
    pub fn string(&self) -> String {
        format!("{}\n", self.ty.dump())
    }
}

#[derive(Clone, Debug)]
pub enum NodeType {
    INT(Token),  //intlit
    UINT(Token), //uintlit
    STRARY(Box<Vec<Token>>),
    INTARY(Box<Vec<Token>>),
    CHARARY(Box<Vec<Token>>),
    ID(String),                                    //identifier
    BINOP(TokenType, Box<Node>, Box<Node>),        //binary-operation
    EX(Box<Node>),                                 //expression
    RETS(TokenType, Box<Node>),                    //return statement
    LETS(TokenType, String, TokenType, Box<Node>), //let statement
    IFS(
        TokenType,
        Box<Node>,
        Box<Vec<Node>>,
        TokenType,
        Box<Vec<Node>>,
    ), //if-else statement
    FUNC(
        String,
        HashMap<String, TokenType>,
        TokenType,
        Box<Vec<Node>>,
    ), //func-name,arguments,statements
    LOOP(TokenType, Box<Vec<Node>>),               //loop statement
    FOR(TokenType, String, String, Box<Vec<Node>>), //for statement
    INVALID,                                       //invalid ast-node
}

impl NodeType {
    pub fn dump(&self) -> String {
        match self {
            NodeType::INT(v) => v.dump(),
            NodeType::ID(s) => s.to_string(),
            NodeType::BINOP(ty, l, r) => format!(
                "type:{}\tlchild:{:?}\trchild:{:?}",
                String::from(ty.string()),
                l.ty,
                r.ty
            ),
            NodeType::EX(n) => n.string().to_string(),
            NodeType::RETS(ty, n) => format!("type:{:?}\texpression:{:?}", ty, n.string()),
            NodeType::FUNC(name, args, ret, stmts) => format!(
                "name:{}\nargs:{:?}\nreturn:{}\nstmts:{:?}",
                name,
                args,
                ret.string(),
                stmts
            ),
            _ => "Invalid Node".to_string(),
        }
    }
}
