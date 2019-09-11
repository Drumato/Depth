//use super::super::ce::types::Error;
use super::lex::Token;
use std::collections::HashMap;
static mut CUR: usize = 0;
static mut NEXT: usize = 1;
#[derive(Debug, Clone)]
pub enum Inst {
    BINARG(usize),
    UNARG(usize),
    NOARG(usize),
}
pub enum Operand {
    REG(String),
    IMM(i128),
}
impl Operand {
    pub fn string(&self) -> String {
        match self {
            Operand::REG(name) => format!("reg<{}>", name),
            Operand::IMM(value) => format!("imm<{}>", value),
        }
    }
    pub fn reg_number(&self) -> u8 {
        match self {
            Operand::REG(name) => match name.as_str() {
                "rax" | "r8" => 0b000,
                "rcx" | "r9" => 0b001,
                "rdx" | "r10" => 0b010,
                "rbx" | "r11" => 0b011,
                "rsp" | "r12" => 0b100,
                "rbp" | "r13" => 0b101,
                "rsi" | "r14" => 0b110,
                "rdi" | "r15" => 0b111,
                c => {
                    eprintln!("invalid Register<{}>", c);

                    0
                }
            },
            _ => {
                eprintln!("reg_number() must called with register");
                0
            }
        }
    }
}
pub struct Info {
    pub inst_name: String,
    pub lop: Option<Operand>,
    pub rop: Option<Operand>,
}

impl Info {
    fn new(name: String) -> Info {
        Info {
            inst_name: name,
            lop: None,
            rop: None,
        }
    }
}

struct Parser {
    tokens: Vec<Token>,
    info_map: HashMap<usize, Info>,
    insts: Vec<Inst>,
    inst_map: HashMap<String, Vec<Inst>>,
    entry: usize,
}
impl Parser {
    fn parse(&mut self) {
        loop {
            let n: String = if let Token::SYMBOL(name) = self.cur_token() {
                name.to_string()
            } else {
                break;
            };
            self.next_token();
            if let Token::COLON = self.cur_token() {
            } else {
                break;
            }
            self.next_token();
            while let Some(()) = self.parse_inst() {}
            self.inst_map.insert(n, self.insts.clone());
        }
    }
    fn parse_inst(&mut self) -> Option<()> {
        let inst: Token = self.get_token();
        match inst {
            Token::RET => {
                let entry: usize = self.entry;
                self.entry += 1;
                self.insts.push(Inst::NOARG(entry));
                self.info_map.insert(entry, Info::new(inst.string()));
                self.next_token();
                Some(())
            }
            Token::PUSH | Token::POP => {
                self.next_token();
                let entry: usize = self.entry;
                self.entry += 1;
                self.insts.push(Inst::UNARG(entry));
                let mut info: Info = Info::new(inst.string());
                info.lop = self.get_operand();
                self.info_map.insert(entry, info);
                Some(())
            }
            Token::MOV => {
                self.next_token();
                let entry: usize = self.entry;
                self.entry += 1;
                self.insts.push(Inst::BINARG(entry));
                let mut info: Info = Info::new(inst.string());
                info.lop = self.get_operand();
                self.next_token();
                info.rop = self.get_operand();
                self.info_map.insert(entry, info);
                Some(())
            }
            _ => None,
        }
    }
    fn get_operand(&self) -> Option<Operand> {
        let t: &Token = self.cur_token();
        match t {
            Token::SYMBOL(name) => {
                self.next_token();
                Some(Operand::REG(name.to_string()))
            }
            Token::INTEGER(value) => {
                self.next_token();
                Some(Operand::IMM(*value))
            }
            _ => None,
        }
    }
    fn cur_token(&self) -> &Token {
        unsafe {
            if CUR == self.tokens.len() {
                return &Token::EOF;
            }
            &self.tokens[CUR]
        }
    }
    fn get_token(&mut self) -> Token {
        unsafe {
            if CUR == self.tokens.len() {
                return Token::EOF;
            }
            self.tokens[CUR].clone()
        }
    }
    fn next_token(&self) {
        unsafe {
            CUR += 1;
            NEXT += 1;
        }
    }
}
pub fn parsing(tokens: Vec<Token>) -> (HashMap<String, Vec<Inst>>, HashMap<usize, Info>) {
    let mut parser: Parser = Parser {
        tokens: tokens,
        info_map: HashMap::new(),
        inst_map: HashMap::new(),
        insts: Vec::new(),
        entry: 0,
    };
    parser.parse();
    (parser.inst_map, parser.info_map)
}
