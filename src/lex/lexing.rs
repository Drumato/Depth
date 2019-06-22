use super::super::parse::error;
use super::token;
use token::{Token, TokenType, TokenVal};
extern crate drumatech;
use drumatech::conv;

/* 字句解析に用いる構造体 */
pub struct Lexer {
    pub input: String, /* 入力文字 */
    pub pos: usize,    /* 現在見ている文字 */
    pub npos: usize,   /* 次見る文字 */
    pub ch: u8,        /* 現在見ている文字 */
}

/* 字句解析を行う関数(トップレベル) */
pub fn lex_phase(input_str: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut lexer: Lexer = Lexer::new(input_str).unwrap();
    while let Some(t) = lexer.next_token() {
        let ty: TokenType = t.ty.clone();
        tokens.push(t);
        if ty == TokenType::TkEof {
            break;
        }
    }
    tokens
}

impl Lexer {
    /* Constructor */
    pub fn new(input_str: String) -> Option<Lexer> {
        let ch: u8 = input_str.bytes().nth(0)?;
        Some(Lexer {
            input: input_str,
            pos: 0,
            npos: 1,
            ch: ch,
        })
    }
    /* 初期状態から規則に従って解析関数にシフトする(DFA的) */
    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();
        let s = conv::u8_to_string(&mut self.ch);
        match self.ch as char {
            '\0' => Some(Token::new((TokenType::TkEof, s, TokenVal::InVal))),
            c if c == '"' => Some(self.judge_string()), //文字列リテラル
            c if (c == 'u' && self.peak_char().is_ascii_digit()) => Some(self.judge_unumber()), //uint
            c if c.is_ascii_digit() => Some(self.judge_number()), //signed nint
            c if c.is_alphabetic() => Some(self.judge_keyword()), //予約語or識別子
            c if c.is_ascii_punctuation() => Some(self.judge_mark()), //その他記号
            _ => None,
        }
    }
    /* 文字列リテラルを受け付けてトークンを返す */
    pub fn judge_string(&mut self) -> Token {
        self.read_char(); //ignore "
        let p: usize = self.pos;
        while self.peak_char() != '"' {
            self.read_char();
        }
        self.read_char();
        let s: String = self.input[p..self.pos].to_string();
        self.read_char();
        Token::new((TokenType::TkStrlit, s, TokenVal::InVal))
    }
    /* 予約後or識別子*/
    fn judge_keyword(&mut self) -> Token {
        let s: String = self.read_ident();
        if token::lookup(&s) {
            return Token::new((token::get_keyword(&s), s, TokenVal::InVal));
        }
        Token::new((TokenType::TkIdent, s, TokenVal::InVal))
    }
    fn judge_mark(&mut self) -> Token {
        let s = conv::u8_to_string(&mut self.ch);
        match self.ch as char {
            '+' => self.judge_plus(),
            '-' => self.judge_minus(),
            '*' => self.judge_star(),
            '/' => self.judge_slash(),
            '%' => self.judge_percent(),
            '=' => self.judge_assign(),
            '&' => self.judge_ampersand(),
            '|' => self.judge_pipe(),
            '!' => self.judge_bang(),
            '<' => self.judge_lt(),
            '>' => self.judge_gt(),
            '.' => self.judge_dot(),
            ',' => self.judge_comma(),
            ':' => self.judge_colon(),
            ';' => self.judge_semicolon(),
            '(' => self.judge_lparen(),
            ')' => self.judge_rparen(),
            '{' => self.judge_lbrace(),
            '}' => self.judge_rbrace(),
            '[' => self.judge_lbracket(),
            ']' => self.judge_rbracket(),
            '\\' => self.judge_backslash(),
            '\'' => self.judge_char(),
            _ => Token::new((TokenType::TkIllegal, s, TokenVal::InVal)),
        }
    }
    /* 識別子である間オフセットを進め､部分文字列を返す */
    pub fn read_ident(&mut self) -> String {
        let p: usize = self.pos;
        while self.ch.is_ascii_alphabetic() || self.ch.is_ascii_digit() || self.ch == 0x5f {
            self.read_char();
        }
        self.input[p..self.pos].to_string()
    }
    /* 数値である間オフセットを進め､部分文字列を返す */
    pub fn read_number(&mut self) -> String {
        let p: usize = self.pos;
        if self.ch as char == '0' {
            //10進数かどうか?
            self.read_char();
            if self.ch as char == 'b' {
                self.read_char();
                //2進数解析
                while (self.ch as char).is_digit(2) {
                    self.read_char();
                }
            } else if self.ch as char == 'o' {
                //8進数解析
                self.read_char();
                while (self.ch as char).is_digit(8) {
                    self.read_char();
                }
            } else if self.ch as char == 'x' {
                //16進数解析
                self.read_char();
                while (self.ch as char).is_digit(16) {
                    self.read_char();
                }
            }
        } else {
            //10進数解析
            while (self.ch as char).is_digit(10) {
                self.read_char();
            }
        }
        self.input[p..self.pos].to_string()
    }
    /* 空白の間進める(これにはタブ文字･改行文字等が含まれる) */
    pub fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }
    fn judge_number(&mut self) -> Token {
        let s: String = self.read_number();
        let ns: &str;
        let base: u32;
        if s.starts_with("0x") {
            /* 16進数 */
            base = 16;
            ns = &s[2..];
        } else if s.starts_with("0o") {
            /* 8進数 */
            base = 8;
            ns = &s[2..];
        } else if s.starts_with("0b") {
            /* 2進数 */
            base = 2;
            ns = &s[2..];
        } else {
            /* 10進数 */
            ns = &s;
            base = 10;
        }
        let val: i128 = i128::from_str_radix(ns, base).unwrap();
        Token::new((TokenType::TkIntlit, s, TokenVal::IntVal(val)))
    }
    fn judge_unumber(&mut self) -> Token {
        self.read_char();
        let s: String = self.read_number();
        if token::lookup(&("u".to_string() + &s)) {
            return Token::new((
                token::get_keyword(&("u".to_string() + &s)),
                s,
                TokenVal::InVal,
            ));
        }
        let ns: &str;
        let base: u32;
        if s.starts_with("0x") {
            base = 16;
            ns = &s[2..];
        } else if s.starts_with("0o") {
            base = 8;
            ns = &s[2..];
        } else if s.starts_with("0b") {
            base = 2;
            ns = &s[2..];
        } else {
            ns = &s;
            base = 10;
        }
        let val: u128 = u128::from_str_radix(ns, base).unwrap();
        Token::new((TokenType::TkUintlit, s, TokenVal::UintVal(val)))
    }
    /* オフセットを1進める */
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
    /* 次の文字をchar型で渡す */
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
    fn judge_char(&mut self) -> Token {
        self.read_char(); //ignore "
        let p: usize = self.pos;
        if (self.input.bytes().nth(self.pos).unwrap() as char) == '\'' {
            error::CompileError::PARSE(format!("empty constant of char '{}'", self.peak_char()))
                .found();
        }
        self.read_char();
        self.read_char();
        let c = self.input.bytes().nth(p).unwrap();
        Token::new((
            TokenType::TkCharlit,
            (c as char).to_string(),
            TokenVal::CharVal(c as char),
        ))
    }
    fn judge_plus(&mut self) -> Token {
        let mut s = conv::u8_to_string(&mut self.ch);
        let p = self.pos;
        if self.peak_char() == '=' {
            self.read_char();
            self.read_char();
            s = self.input[p..self.pos].to_string();
            return Token::new((TokenType::TkPlusassign, s, TokenVal::InVal));
        }
        if self.peak_char() == '+' {
            self.read_char();
            self.read_char();
            s = self.input[p..self.pos].to_string();
            return Token::new((TokenType::TkIncre, s, TokenVal::InVal));
        }
        self.read_char();
        Token::new((TokenType::TkPlus, s, TokenVal::InVal))
    }
    fn judge_assign(&mut self) -> Token {
        let mut s = conv::u8_to_string(&mut self.ch);
        let p = self.pos;
        if self.peak_char() == '=' {
            self.read_char();
            self.read_char();
            s = self.input[p..self.pos].to_string();
            return Token::new((TokenType::TkEq, s, TokenVal::InVal));
        }
        self.read_char();
        Token::new((TokenType::TkAssign, s, TokenVal::InVal))
    }
    fn judge_minus(&mut self) -> Token {
        let mut s = conv::u8_to_string(&mut self.ch);
        let p = self.pos;
        if self.peak_char() == '=' {
            self.read_char();
            self.read_char();
            s = self.input[p..self.pos].to_string();
            return Token::new((TokenType::TkMinusassign, s, TokenVal::InVal));
        }
        if self.peak_char() == '>' {
            self.read_char();
            self.read_char();
            s = self.input[p..self.pos].to_string();
            return Token::new((TokenType::TkArrow, s, TokenVal::InVal));
        }
        if self.peak_char() == '-' {
            self.read_char();
            self.read_char();
            s = self.input[p..self.pos].to_string();
            return Token::new((TokenType::TkDecre, s, TokenVal::InVal));
        }
        self.read_char();
        Token::new((TokenType::TkMinus, s, TokenVal::InVal))
    }
    fn judge_star(&mut self) -> Token {
        let mut s = conv::u8_to_string(&mut self.ch);
        let p = self.pos;
        if self.peak_char() == '=' {
            self.read_char();
            self.read_char();
            s = self.input[p..self.pos].to_string();
            return Token::new((TokenType::TkStarassign, s, TokenVal::InVal));
        }
        self.read_char();
        Token::new((TokenType::TkStar, s, TokenVal::InVal))
    }
    fn judge_slash(&mut self) -> Token {
        let mut s = conv::u8_to_string(&mut self.ch);
        let p = self.pos;
        if self.peak_char() == '=' {
            self.read_char();
            self.read_char();
            s = self.input[p..self.pos].to_string();
            return Token::new((TokenType::TkSlashassign, s, TokenVal::InVal));
        }
        self.read_char();
        Token::new((TokenType::TkSlash, s, TokenVal::InVal))
    }
    fn judge_percent(&mut self) -> Token {
        let mut s = conv::u8_to_string(&mut self.ch);
        let p = self.pos;
        if self.peak_char() == 's' {
            self.read_char();
            self.read_char();
            s = self.input[p..self.pos].to_string();
            return Token::new((TokenType::TkPerStr, s, TokenVal::InVal));
        }
        if self.peak_char() == 'i' {
            self.read_char();
            self.read_char();
            s = self.input[p..self.pos].to_string();
            return Token::new((TokenType::TkPerInt, s, TokenVal::InVal));
        }
        if self.peak_char() == 'c' {
            self.read_char();
            self.read_char();
            s = self.input[p..self.pos].to_string();
            return Token::new((TokenType::TkPerChar, s, TokenVal::InVal));
        }
        if self.peak_char() == '=' {
            self.read_char();
            self.read_char();
            s = self.input[p..self.pos].to_string();
            return Token::new((TokenType::TkPercentassign, s, TokenVal::InVal));
        }
        self.read_char();
        Token::new((TokenType::TkPercent, s, TokenVal::InVal))
    }
    fn judge_ampersand(&mut self) -> Token {
        let mut s = conv::u8_to_string(&mut self.ch);
        let p = self.pos;
        if self.peak_char() == '&' {
            self.read_char();
            self.read_char();
            s = self.input[p..self.pos].to_string();
            return Token::new((TokenType::TkLogand, s, TokenVal::InVal));
        }
        self.read_char();
        Token::new((TokenType::TkAmpersand, s, TokenVal::InVal))
    }
    fn judge_pipe(&mut self) -> Token {
        let mut s = conv::u8_to_string(&mut self.ch);
        let p = self.pos;
        if self.peak_char() == '|' {
            self.read_char();
            self.read_char();
            s = self.input[p..self.pos].to_string();
            return Token::new((TokenType::TkLogor, s, TokenVal::InVal));
        }
        self.read_char();
        Token::new((TokenType::TkPipe, s, TokenVal::InVal))
    }
    fn judge_bang(&mut self) -> Token {
        let mut s = conv::u8_to_string(&mut self.ch);
        let p = self.pos;
        if self.peak_char() == '=' {
            self.read_char();
            self.read_char();
            s = self.input[p..self.pos].to_string();
            return Token::new((TokenType::TkNoteq, s, TokenVal::InVal));
        }
        self.read_char();
        Token::new((TokenType::TkBang, s, TokenVal::InVal))
    }
    fn judge_lt(&mut self) -> Token {
        let mut s = conv::u8_to_string(&mut self.ch);
        let p = self.pos;
        if self.peak_char() == '<' {
            self.read_char();
            self.read_char();
            s = self.input[p..self.pos].to_string();
            return Token::new((TokenType::TkLshift, s, TokenVal::InVal));
        }
        if self.peak_char() == '=' {
            self.read_char();
            self.read_char();
            s = self.input[p..self.pos].to_string();
            return Token::new((TokenType::TkLteq, s, TokenVal::InVal));
        }
        self.read_char();
        Token::new((TokenType::TkLt, s, TokenVal::InVal))
    }
    fn judge_gt(&mut self) -> Token {
        let mut s = conv::u8_to_string(&mut self.ch);
        let p = self.pos;
        if self.peak_char() == '>' {
            self.read_char();
            self.read_char();
            s = self.input[p..self.pos].to_string();
            return Token::new((TokenType::TkRshift, s, TokenVal::InVal));
        }
        if self.peak_char() == '=' {
            self.read_char();
            self.read_char();
            s = self.input[p..self.pos].to_string();
            return Token::new((TokenType::TkGteq, s, TokenVal::InVal));
        }
        self.read_char();
        Token::new((TokenType::TkGt, s, TokenVal::InVal))
    }
    fn judge_dot(&mut self) -> Token {
        let s = conv::u8_to_string(&mut self.ch);
        self.read_char();
        Token::new((TokenType::TkDot, s, TokenVal::InVal))
    }
    fn judge_comma(&mut self) -> Token {
        let s = conv::u8_to_string(&mut self.ch);
        self.read_char();
        Token::new((TokenType::TkComma, s, TokenVal::InVal))
    }
    fn judge_colon(&mut self) -> Token {
        let s = conv::u8_to_string(&mut self.ch);
        self.read_char();
        Token::new((TokenType::TkColon, s, TokenVal::InVal))
    }
    fn judge_semicolon(&mut self) -> Token {
        let s = conv::u8_to_string(&mut self.ch);
        self.read_char();
        Token::new((TokenType::TkSemicolon, s, TokenVal::InVal))
    }
    fn judge_lparen(&mut self) -> Token {
        let s = conv::u8_to_string(&mut self.ch);
        self.read_char();
        Token::new((TokenType::TkLparen, s, TokenVal::InVal))
    }
    fn judge_rparen(&mut self) -> Token {
        let s = conv::u8_to_string(&mut self.ch);
        self.read_char();
        Token::new((TokenType::TkRparen, s, TokenVal::InVal))
    }
    fn judge_lbrace(&mut self) -> Token {
        let s = conv::u8_to_string(&mut self.ch);
        self.read_char();
        Token::new((TokenType::TkLbrace, s, TokenVal::InVal))
    }
    fn judge_rbrace(&mut self) -> Token {
        let s = conv::u8_to_string(&mut self.ch);
        self.read_char();
        Token::new((TokenType::TkRbrace, s, TokenVal::InVal))
    }
    fn judge_lbracket(&mut self) -> Token {
        let s = conv::u8_to_string(&mut self.ch);
        self.read_char();
        Token::new((TokenType::TkLbracket, s, TokenVal::InVal))
    }
    fn judge_rbracket(&mut self) -> Token {
        let s = conv::u8_to_string(&mut self.ch);
        self.read_char();
        Token::new((TokenType::TkRbracket, s, TokenVal::InVal))
    }
    fn judge_backslash(&mut self) -> Token {
        let s = conv::u8_to_string(&mut self.ch);
        self.read_char();
        Token::new((TokenType::TkBackslash, s, TokenVal::InVal))
    }
}
