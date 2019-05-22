use super::super::lex::{lexing, token};
pub struct Parser {
    pub l: lexing::Lexer,
    pub cur: token::Token,
    pub next: token::Token,
}

impl Parser {
    pub fn new(mut lexer: lexing::Lexer) -> Parser {
        let cur: token::Token = lexer.next_token();
        let next: token::Token = lexer.next_token();
        Parser {
            l: lexer,
            cur: cur,
            next: next,
        }
    }
}
