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
    fn new(param: (usize, usize, Vec<char>)) -> Lexer {
        Lexer {
            position: param.0,
            readposition: param.1,
            ch: '0',
            input: param.2,
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
        let position = self.position;
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
    ty: String,
    intval: i64,
    literal: String,
}
fn lex(input: &String) /*-> Assembly*/
{
    let input_chars: Vec<char> = input.as_str().chars().collect();
    let tokens = tokenize(&input_chars);
}

fn tokenize(input: &Vec<char>) /*-> Vec<Token>*/
{
    let mut tokens: Vec<Token> = vec![];
    let mut lexer = Lexer::new((0, 0, input.to_vec()));
}
