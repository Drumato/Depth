extern crate drumatech;
use colored::*;
use drumatech::conv;

pub fn lex_phase(input_str: String) -> Vec<AToken> {
    let mut tokens: Vec<AToken> = Vec::new();
    let mut lexer: ALexer = ALexer::new(input_str).unwrap();
    while let Some(t) = lexer.next_token() {
        let ty: ATType = t.ty.clone();
        tokens.push(t);
        if ty == ATType::AEof {
            break;
        }
    }
    tokens
}

#[derive(Debug, Clone, PartialEq)]
pub struct AToken {
    pub ty: ATType,
    pub literal: String,
    pub val: ATVal,
}
impl AToken {
    /* デバッグ用関数 */
    pub fn dump(&self) -> String {
        format!(
            "type:{}  input:{}  val:{}",
            self.ty.string().blue().bold(),
            self.literal.blue().bold(),
            self.val.string().blue().bold()
        )
    }
    /* Constructor */
    pub fn new(param: (ATType, String, ATVal)) -> AToken {
        AToken {
            ty: param.0,
            literal: param.1,
            val: param.2,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum ATVal {
    IntVal(i128),
    InVal,
}

impl ATVal {
    /* デバッグ用メソッド */
    pub fn string(&self) -> String {
        match self {
            ATVal::IntVal(d) => format!("{}", d),
            _ => "".to_string(),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum ATType {
    /* for identifying Type */
    AIllegal,
    AEof,
    AIntlit,

    /* reg and inst */
    AReg,
    AMov,
    APush,
    APop,
    ARet,
    /* marks */
    APlus,
    AMinus,
    ASlash,
    AStar,
    AComma,
    AColon,
    ALbracket,
    ARbracket,
    ALabel,
}
impl ATType {
    fn lookup(s: &str) -> ATType {
        match s {
            "rax" | "rbx" | "rcx" | "rdx" | "rsi" | "rdi" | "rsp" | "rbp" | "r8" | "r9" | "r10"
            | "r11" | "r12" | "r13" | "r14" | "r15" => ATType::AReg,
            "mov" => ATType::AMov,
            "ret" => ATType::ARet,
            "push" => ATType::APush,
            "pop" => ATType::APop,
            _ => ATType::AIllegal,
        }
    }
    pub fn string(&self) -> &str {
        match self {
            ATType::AIllegal => "ILLEGAL",
            ATType::AEof => "EOF",
            ATType::AIntlit => "INT-LITERAL",
            ATType::APlus => "PLUS",
            ATType::AMinus => "MINUS",
            ATType::AStar => "STAR",
            ATType::ASlash => "SLASH",
            ATType::AComma => "COMMA",
            ATType::ALbracket => "LBRACKET",
            ATType::ARbracket => "RBRACKET",
            ATType::AMov => "MOV",
            ATType::APop => "POP",
            ATType::APush => "PUSH",
            ATType::ARet => "RET",
            ATType::AReg => "REGISTER",
            ATType::AColon => "COLON",
            ATType::ALabel => "LABEL",
        }
    }
}
struct ALexer {
    pub input: String, /* 入力文字 */
    pub pos: usize,    /* 現在見ている文字 */
    pub npos: usize,   /* 次見る文字 */
    pub ch: u8,        /* 現在見ている文字 */
}

impl ALexer {
    /* Constructor */
    fn new(input_str: String) -> Option<ALexer> {
        let ch: u8 = input_str.bytes().nth(0)?;
        Some(ALexer {
            input: input_str,
            pos: 0,
            npos: 1,
            ch: ch,
        })
    }
    /* 初期状態から規則に従って解析関数にシフトする(DFA的) */
    fn next_token(&mut self) -> Option<AToken> {
        self.skip_whitespace();
        let s = conv::u8_to_string(&mut self.ch);
        match self.ch as char {
            '\0' => Some(AToken::new((ATType::AEof, s, ATVal::InVal))),
            c if c.is_alphabetic() => Some(self.judge_reginst()),
            c if c.is_ascii_digit() => Some(self.judge_number()), //signed nint
            c if c.is_ascii_punctuation() => Some(self.judge_mark()), //その他記号
            _ => None,
        }
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
    fn read_number(&mut self) -> String {
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
    fn judge_number(&mut self) -> AToken {
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
        AToken::new((ATType::AIntlit, s, ATVal::IntVal(val)))
    }
    pub fn read_reginst(&mut self) -> String {
        let p: usize = self.pos;
        while self.ch.is_ascii_alphabetic() || self.ch.is_ascii_digit() || self.ch == 0x5f {
            self.read_char();
        }
        self.input[p..self.pos].to_string()
    }
    fn judge_reginst(&mut self) -> AToken {
        let s: String = self.read_reginst();
        if self.ch == ':' as u8 {
            self.read_char();
            return AToken::new((ATType::ALabel, s + ":", ATVal::InVal));
        }
        match ATType::lookup(&s) {
            ATType::AReg => AToken::new((ATType::AReg, s, ATVal::InVal)),
            ATType::AMov => AToken::new((ATType::AMov, s, ATVal::InVal)),
            ATType::ARet => AToken::new((ATType::ARet, s, ATVal::InVal)),
            ATType::APush => AToken::new((ATType::APush, s, ATVal::InVal)),
            ATType::APop => AToken::new((ATType::APop, s, ATVal::InVal)),
            _ => AToken::new((ATType::AIllegal, s, ATVal::InVal)),
        }
    }
    fn judge_mark(&mut self) -> AToken {
        let s = conv::u8_to_string(&mut self.ch);
        match self.ch as char {
            '+' => self.judge_plus(),
            '-' => self.judge_minus(),
            '*' => self.judge_star(),
            '/' => self.judge_slash(),
            '[' => self.judge_lbracket(),
            ']' => self.judge_rbracket(),
            ':' => self.judge_colon(),
            ',' => self.judge_comma(),
            _ => AToken::new((ATType::AIllegal, s, ATVal::InVal)),
        }
    }
    fn judge_lbracket(&mut self) -> AToken {
        let s = conv::u8_to_string(&mut self.ch);
        self.read_char();
        AToken::new((ATType::ALbracket, s, ATVal::InVal))
    }
    fn judge_rbracket(&mut self) -> AToken {
        let s = conv::u8_to_string(&mut self.ch);
        self.read_char();
        AToken::new((ATType::ARbracket, s, ATVal::InVal))
    }
    fn judge_comma(&mut self) -> AToken {
        let s = conv::u8_to_string(&mut self.ch);
        self.read_char();
        AToken::new((ATType::AComma, s, ATVal::InVal))
    }
    fn judge_plus(&mut self) -> AToken {
        let mut s = conv::u8_to_string(&mut self.ch);
        self.read_char();
        AToken::new((ATType::APlus, s, ATVal::InVal))
    }
    fn judge_minus(&mut self) -> AToken {
        let mut s = conv::u8_to_string(&mut self.ch);
        self.read_char();
        AToken::new((ATType::AMinus, s, ATVal::InVal))
    }
    fn judge_star(&mut self) -> AToken {
        let mut s = conv::u8_to_string(&mut self.ch);
        self.read_char();
        AToken::new((ATType::AStar, s, ATVal::InVal))
    }
    fn judge_slash(&mut self) -> AToken {
        let mut s = conv::u8_to_string(&mut self.ch);
        self.read_char();
        AToken::new((ATType::ASlash, s, ATVal::InVal))
    }
    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }
    fn judge_colon(&mut self) -> AToken {
        let mut s = conv::u8_to_string(&mut self.ch);
        self.read_char();
        AToken::new((ATType::AColon, s, ATVal::InVal))
    }
}
