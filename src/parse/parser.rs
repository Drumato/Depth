use super::super::lex::{lexing, token};
use super::node::{Node, Operator, OperatorType, Term, TermType, TermVal};
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
        if self.cur.ty.string() == ty.string() {
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
        if self.next.ty.string() == ty.string() {
            self.next_token();
        }
        println!(
            "Error! {} expected but got {}",
            ty.string(),
            self.next.ty.string()
        );
    }
    pub fn term(&mut self) -> Box<Node> {
        if self.cur.ty.string() != TokenType::TkIntlit.string() {
            println!(
                "Error! Int-Literal expected but got {}",
                self.cur.ty.string()
            );
        }
        let t: Token = self.cur;
        self.next_token();
        Term::new(
            String::from("intval"),
            TermType::INT,
            TermVal::IntVal(t.val),
        ) as Node
    }
    pub fn adsub(&mut self) -> Box<Node> {
        let mut lchild: Node = self.term();
        if self.cur.ty.compare(TokenType::TkPlus) && self.cur.ty.compare(TokenType::TkMinus) {
            println!("Error! Operator expected but got {}", self.cur.ty.string());
        }
        Operator::new(
            OperatorType::find(self.cur.literal).unwrap(),
            lchild,
            self.term(),
        ) as Node
    }
}

pub fn parse() {}
