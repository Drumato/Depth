use crate::assemble::lex::Token;
use crate::ce::types::Error;
use crate::object::elf::elf64::Rela;

use std::collections::BTreeMap;

static mut CUR: usize = 0;
static mut NEXT: usize = 1;
#[derive(Debug, Clone)]
pub enum Inst {
    BINARG(usize),
    UNARG(usize),
    NOARG(usize),
    LABEL(usize, String),
}
pub enum Operand {
    REG(String),
    SYMBOL(String),
    IMM(i128),
    ADDRESS(Box<Operand>, i128),
    ELEMENT(Box<Operand>, Box<Operand>, i128, i128),
}
impl Operand {
    pub fn string(&self) -> String {
        match self {
            Operand::REG(name) => format!("reg<{}>", name),
            Operand::SYMBOL(name) => format!("symbol<{}>", name),
            Operand::IMM(value) => format!("imm<{}>", value),
            Operand::ADDRESS(content, offset) => {
                format!("address -{}[{}]", offset, content.string())
            }
            Operand::ELEMENT(base, idx, size, _id) => {
                format!("element[{} + {} * {}]", base.string(), idx.string(), size,)
            }
        }
    }
    pub fn number(name: &str) -> u8 {
        match name {
            "al" | "ax" | "eax" | "rax" | "r8" => 0b000,
            "cl" | "cx" | "ecx" | "rcx" | "r9" => 0b001,
            "rdx" | "r10" => 0b010,
            "rbx" | "r11" => 0b011,
            "rsp" | "r12" => 0b100,
            "rbp" | "r13" => 0b101,
            "rsi" | "r14" => 0b110,
            "rdi" | "r15" => 0b111,
            c => {
                Error::ASSEMBLE.found(&format!("invalid Register<{}>", c));
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
    info_map: BTreeMap<usize, Info>,
    insts: Vec<Inst>,
    inst_map: BTreeMap<String, Vec<Inst>>,
    entry: usize,
    rels: BTreeMap<String, Rela>,
}
impl Parser {
    fn parse(&mut self) {
        loop {
            let t: &Token = self.cur_token();
            let n: String;
            if let Token::SYMBOL(name) = t {
                n = name.to_string();
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
            self.insts = Vec::new();
        }
    }
    fn parse_inst(&mut self) -> Option<()> {
        let inst: Token = self.get_token();
        match inst {
            Token::RET | Token::CQO | Token::SYSCALL => {
                let entry: usize = self.entry;
                self.entry += 1;
                self.insts.push(Inst::NOARG(entry));
                self.info_map.insert(entry, Info::new(inst.string()));
                self.next_token();
                Some(())
            }
            Token::PUSH
            | Token::POP
            | Token::IDIV
            | Token::SETL
            | Token::SETLE
            | Token::SETG
            | Token::SETGE
            | Token::SETE
            | Token::SETNE
            | Token::CALL
            | Token::NEG
            | Token::JMP
            | Token::JZ => {
                self.next_token();
                let entry: usize = self.entry;
                self.entry += 1;
                self.insts.push(Inst::UNARG(entry));
                let mut info: Info = Info::new(inst.string());
                info.lop = self.get_operand();
                if let Some(Operand::SYMBOL(name)) = &info.lop {
                    if !name.starts_with(".") {
                        self.rels.insert(name.to_string(), Rela::new());
                    }
                }
                self.info_map.insert(entry, info);
                Some(())
            }
            Token::MOV
            | Token::MOVZX
            | Token::ADD
            | Token::SUB
            | Token::CMP
            | Token::LEA
            | Token::IMUL
            | Token::SAR
            | Token::SAL => {
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
            Token::SYMBOL(name) => {
                if name.starts_with(".") {
                    let entry: usize = self.entry;
                    self.insts.push(Inst::LABEL(entry, name.to_string()));
                    self.next_token();
                    self.next_token();
                    Some(())
                } else {
                    None
                }
            }
            _ => None,
        }
    }
    fn get_operand(&self) -> Option<Operand> {
        let t: &Token = self.cur_token();
        match t {
            Token::SYMBOL(name) => match name.as_str() {
                "rax" | "rbx" | "rcx" | "rdx" | "rsi" | "rdi" | "rsp" | "rbp" | "r8" | "r9"
                | "r10" | "r11" | "r12" | "r13" | "r14" | "r15" | "al" => {
                    self.next_token();
                    Some(Operand::REG(name.to_string()))
                }
                _ => {
                    self.next_token();
                    Some(Operand::SYMBOL(name.to_string()))
                }
            },
            Token::INTEGER(value) => {
                self.next_token();
                Some(Operand::IMM(*value))
            }
            Token::MINUS => {
                self.next_token();
                let integer: Option<Operand> = self.get_operand();
                let mut address: Option<Operand> = self.get_operand();
                if let Some(Operand::ADDRESS(ref mut _content, ref mut offset)) = address {
                    if let Some(Operand::IMM(value)) = integer {
                        *offset = -value;
                    }
                } else if let Some(Operand::ELEMENT(
                    ref mut _base,
                    ref mut _index,
                    ref mut _scale,
                    ref mut offset,
                )) = address
                {
                    if let Some(Operand::IMM(value)) = integer {
                        *offset = -value;
                    }
                }
                address
            }
            Token::LBRACKET => {
                self.next_token();
                let content: Option<Operand> = self.get_operand();
                let t: &Token = self.cur_token();
                if let &Token::PLUS = t {
                    self.next_token();
                    let idx: Operand = self.get_operand().unwrap();
                    self.next_token();
                    let size: Operand = self.get_operand().unwrap();
                    if let Operand::IMM(value) = size {
                        self.next_token();
                        return Some(Operand::ELEMENT(
                            Box::new(content.unwrap()),
                            Box::new(idx),
                            value,
                            0,
                        ));
                    } else {
                        return None;
                    }
                } else {
                    self.next_token();
                    Some(Operand::ADDRESS(Box::new(content.unwrap()), 0))
                }
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
pub fn parsing(
    tokens: Vec<Token>,
) -> (
    BTreeMap<String, Vec<Inst>>,
    BTreeMap<usize, Info>,
    BTreeMap<String, Rela>,
) {
    unsafe {
        CUR = 0;
        NEXT = 1;
    }
    let mut parser: Parser = Parser {
        tokens: tokens,
        info_map: BTreeMap::new(),
        inst_map: BTreeMap::new(),
        insts: Vec::new(),
        entry: 0,
        rels: BTreeMap::new(),
    };
    parser.parse();
    (parser.inst_map, parser.info_map, parser.rels)
}
