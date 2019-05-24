use super::super::lex::{lexing, token};
use super::node::{Node, NodeType};
use token::{Token, TokenType, TokenVal};

pub struct Parser {
    pub l: lexing::Lexer,
    pub cur: Token,
    pub next: Token,
}

impl Parser {
    pub fn new(mut lexer: lexing::Lexer) -> Parser {
        let cur: Token = lexer.next_token();
        let next: Token = lexer.next_token();
        Parser {
            l: lexer,
            cur: cur,
            next: next,
        }
    }
    pub fn next_token(&mut self) {
        self.cur = self.next.clone();
        self.next = self.l.next_token();
    }
    pub fn consume(&mut self, ty: TokenType) -> bool {
        if self.cur.ty == ty {
            self.next_token();
            return true;
        }
        println!(
            "Error! {} expected but got {}",
            ty.string(),
            self.cur.ty.string()
        );
        false
    }
    pub fn expect(&mut self, ty: TokenType) {
        if self.next.ty == ty {
            self.next_token();
            return;
        }
        println!(
            "Error! {} expected but got {}",
            ty.string(),
            self.next.ty.string()
        );
    }
    fn term(&mut self) -> Node {
        if self.cur.ty != TokenType::TkIntlit && self.cur.ty != TokenType::TkUintlit {
            println!(
                "Error! Number-Literal expected but got {}",
                self.cur.ty.string()
            );
        }
        let t: Token = self.cur.clone();
        self.next_token();
        Node::new_num(t)
    }
    fn adsub(&mut self) -> Node {
        let mut lchild: Node = self.muldiv();
        loop {
            let t: Token = self.cur.clone();
            if t.ty != TokenType::TkPlus && t.ty != TokenType::TkMinus {
                break;
            }
            self.next_token();
            lchild = Node::new_binop(t.ty, lchild, self.muldiv());
        }
        lchild
    }
    fn muldiv(&mut self) -> Node {
        let mut lchild: Node = self.term();
        loop {
            let t: Token = self.cur.clone();
            if t.ty != TokenType::TkStar && t.ty != TokenType::TkSlash {
                break;
            }
            self.next_token();
            lchild = Node::new_binop(t.ty, lchild, self.term());
        }
        lchild
    }
    fn expr(&mut self) -> Node {
        if self.cur.ty != TokenType::TkIntlit && self.cur.ty != TokenType::TkUintlit {
            println!(
                "Error! Number-Literal expected but got {}",
                self.cur.ty.string()
            );
        }
        self.adsub()
    }
    fn stmt(&mut self) -> Node {
        match self.cur.ty {
            TokenType::TkReturn => self.parse_return(),
            _ => Node::new(NodeType::INVALID),
        }
    }

    fn parse_return(&mut self) -> Node {
        let ret_keyword: TokenType = self.cur.ty.clone();
        self.next_token();
        Node::new_rets(ret_keyword, self.expr())
    }
    fn func(&mut self) -> Node {
        if !self.consume(TokenType::TkF) {
            println!("invalid f {}", self.cur.literal);
            return Node::new(NodeType::INVALID);
        }
        let func_name: String = self.cur.literal.clone();
        self.expect(TokenType::TkLparen);
        let mut arguments: Vec<Node> = Vec::new();
        self.expect(TokenType::TkRparen);
        self.expect(TokenType::TkLbrace);
        self.next_token();
        let mut statements: Vec<Node> = Vec::new();
        while self.cur.ty != TokenType::TkRbrace {
            let n: Node = self.stmt();
            statements.push(n);
        }
        self.consume(TokenType::TkRbrace);
        self.consume(TokenType::TkEof);
        Node::new_func(func_name, arguments, statements)
    }
}

pub fn parse(lexer: lexing::Lexer) -> Vec<Node> {
    let mut parser: Parser = Parser::new(lexer);
    let mut nodes: Vec<Node> = Vec::new();
    nodes.push(parser.func());
    nodes
}
