use super::super::lex::{lexing, token};
use super::node::Node;
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
        }
        println!(
            "Error! {} expected but got {}",
            ty.string(),
            self.next.ty.string()
        );
    }
    pub fn term(&mut self) -> Node {
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
    pub fn adsub(&mut self) -> Node {
        let mut lchild: Node = self.term();
        loop {
            let t: Token = self.cur.clone();
            if t.ty != TokenType::TkPlus && t.ty != TokenType::TkMinus {
                break;
            }
            self.next_token();
            lchild = Node::new_binop(t.ty, lchild, self.term());
        }
        if self.cur.ty != TokenType::TkEof {
            println!("Error! EOF token expected but got {}", self.cur.ty.string());
        }
        lchild
    }
}

pub fn parse(lexer: lexing::Lexer) -> Vec<Node> {
    let mut parser: Parser = Parser::new(lexer);
    let mut nodes: Vec<Node> = Vec::new();
    nodes.push(parser.adsub());
    nodes
}
