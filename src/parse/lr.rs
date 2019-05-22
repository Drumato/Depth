use super::super::lex::{lexing, token};
use token::{Token, TokenType};
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
    pub fn term(&mut self) {
        if self.cur.ty.string() != TokenType::TkIntlit.string() {
            println!(
                "Error! Int-Literal expected but got {}",
                self.cur.ty.string()
            );
        }
    }
}
