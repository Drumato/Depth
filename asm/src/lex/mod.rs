use std::io::BufRead;
use std::str::from_utf8;

pub fn iter_lines(stream: &mut BufRead) {
    let mut buffer = String::new();
    loop {
        match stream.read_line(&mut buffer) {
            Ok(0) => break,
            Ok(_) => {
                lex(&buffer);
                buffer.clear();
                continue;
            }
            Err(e) => {
                println!("Error found:{}", e);
                break;
            }
        }
    }
}

struct Lexer {
    position: usize,
    readposition: usize,
    ch: char,
    input: Vec<char>,
}
impl Lexer {
    fn new(param: (usize, usize, char, Vec<char>)) -> Lexer {
        Lexer {
            position: param.0,
            readposition: param.1,
            ch: param.2,
            input: param.3,
        }
    }
    fn read_char(&mut self) {
        if self.readposition >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input[self.readposition];
        }
        self.position = self.readposition;
        self.readposition += 1;
    }
    fn read_number(&mut self) -> String {
        let position: usize = self.position;
        let mut return_str: Vec<u8> = vec![];
        loop {
            let u: u8 = self.ch as u8;
            return_str.push(u);
            if !self.ch.is_digit(16) && self.ch != 'x' {
                break;
            }
            self.read_char();
        }
        return from_utf8(return_str.as_slice()).unwrap().into();
    }
    fn read_keyword(&mut self) -> String {
        let position: usize = self.position;
        let mut return_str: Vec<u8> = vec![];
        loop {
            let u: u8 = self.ch as u8;
            return_str.push(u);
            if !self.ch.is_alphabetic() {
                break;
            }
            self.read_char();
        }
        return from_utf8(return_str.as_slice()).unwrap().into();
    }
    fn skip_whitespace(&mut self) {
        loop {
            if !self.ch.is_ascii_whitespace() {
                break;
            }
            self.read_char();
        }
    }
    fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        match self.ch {
            _ => Token::new((TokenType::EOF, -42, String::new())),
        }
    }
}

struct Assembly {
    opcode: Mnemonic,
    lop: Operand,
    rop: Operand,
}
struct Mnemonic {
    code: u8,
    name: String,
}
struct Operand {
    reg: String,
    val: u64,
}
struct Token {
    ty: TokenType,
    intval: i64,
    literal: String,
}

impl Token {
    fn new(param: (TokenType, i64, String)) -> Token {
        Token {
            ty: param.0,
            intval: param.1,
            literal: param.2,
        }
    }
}

enum TokenType {
    /*elements*/
    MNEMONIC,
    IMMEDIATE,
    REGISTER,
    WORD,
    DWORD,
    QWORD,
    TKPTR,

    /* symbols */
    PLUS,
    MINUS,
    ASTERISK,
    SLASH,
    DOT,
    COLON,
    COMMA,
    LBRACKET,
    RBRACKET,
    QUOTE,
    DOUBLEQUOTE,
    ATSIGN,

    /* etc */
    EOF,
}

fn lex(input: &String) /*-> Assembly*/
{
    let input_chars: Vec<char> = input.as_str().chars().collect();
    let tokens = tokenize(&input_chars);
}

fn tokenize(input: &Vec<char>) /*-> Vec<Token>*/
{
    let mut tokens: Vec<Token> = vec![];
    let mut lexer = Lexer::new((0, 0, input.to_vec()[0], input.to_vec()));
    lexer.read_char();
    lexer.read_char();
}
