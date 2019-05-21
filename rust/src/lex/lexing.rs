pub struct Lexer {
    pub input: String,
    pub pos: usize,
    pub npos: usize,
    pub ch: u8,
}

use super::token;
use token::{Token, TokenType, TokenVal};
extern crate drumatech;
use drumatech::conv;

impl Lexer {
    pub fn new(input_str: String) -> Option<Lexer> {
        let ch: u8 = input_str.bytes().nth(0)?;
        Some(Lexer {
            input: input_str,
            pos: 0,
            npos: 1,
            ch: ch,
        })
    }
    pub fn read_char(&mut self) {
        if self.npos >= self.input.len() {
            self.ch = 0; //null termination
        } else {
            match self.input.bytes().nth(self.npos) {
                Some(c) => self.ch = c,
                None => panic!("Error found between calling read_char() function"),
            }
        }
        self.pos = self.npos;
        self.npos += 1;
    }
    pub fn peak_char(&self) -> char {
        if self.npos >= self.input.len() {
            return '\0';
        } else {
            match self.input.bytes().nth(self.npos) {
                Some(c) => c as char,
                None => panic!("Error found between calling read_char() function"),
            }
        }
    }
    pub fn peak_byte(self) -> u8 {
        if self.npos >= self.input.len() {
            return 0;
        } else {
            match self.input.bytes().nth(self.npos) {
                Some(c) => c,
                None => panic!("Error found between calling read_char() function"),
            }
        }
    }
    pub fn read_ident(&mut self) -> String {
        let p: usize = self.pos;
        while self.ch.is_ascii_alphabetic() || self.ch == 0x5f {
            self.read_char();
        }
        self.input[p..self.pos].to_string()
    }
    pub fn read_string(&mut self) -> String {
        self.read_char(); //ignore "
        let p: usize = self.pos;
        while self.peak_char() != '"' {
            self.read_char();
        }
        self.read_char(); //ignore "
        self.read_char();
        self.input[p..self.pos].to_string()
    }
    pub fn read_number(&mut self) -> String {
        let p: usize = self.pos;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }
        self.input[p..self.pos].to_string()
    }
    pub fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let s = conv::u8_to_string(&mut self.ch);
        match self.ch as char {
            '\0' => Token::new((TokenType::TkEof, s, TokenVal::InVal)),
            c if c.is_alphabetic() => self.judge_keyword(),
            _ => Token::new((TokenType::TkIllegal, s, TokenVal::InVal)),
        }
    }
    pub fn judge_keyword(&mut self) -> Token {
        let s: String = self.read_ident();
        if token::lookup(&s) {
            self.read_char();
            return Token::new((token::get_keyword(&s), s, TokenVal::InVal));
        }
        self.read_char();
        Token::new((TokenType::TkIdent, s, TokenVal::InVal))
    }
}
