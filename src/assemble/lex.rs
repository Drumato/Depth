use super::super::ce::types::Error;
#[derive(Eq, PartialEq, Clone)]
pub enum Token {
    INTEGER(i128),
    COLON,
    COMMA,
    MINUS,
    LBRACKET,
    RBRACKET,
    MOV,
    MOVZX,
    ADD,
    CALL,
    CMP,
    CQO,
    IDIV,
    IMUL,
    JZ,
    JMP,
    LEA,
    NEG,
    PUSH,
    POP,
    RET,
    SAL,
    SAR,
    SETL,
    SETLE,
    SETG,
    SETGE,
    SETE,
    SETNE,
    SUB,
    SYSCALL,
    BLANK,
    LF,
    SYMBOL(String),
    EOF,
}
impl Token {
    fn should_ignore(&self) -> bool {
        match self {
            &Token::BLANK | &Token::LF => true,
            _ => false,
        }
    }
    pub fn string(&self) -> String {
        match self {
            Token::ADD => "add".to_string(),
            Token::CALL => "call".to_string(),
            Token::CMP => "cmp".to_string(),
            Token::CQO => "cqo".to_string(),
            Token::IDIV => "idiv".to_string(),
            Token::IMUL => "imul".to_string(),
            Token::JMP => "jmp".to_string(),
            Token::JZ => "jz".to_string(),
            Token::LEA => "lea".to_string(),
            Token::MOV => "mov".to_string(),
            Token::MOVZX => "movzx".to_string(),
            Token::NEG => "neg".to_string(),
            Token::PUSH => "push".to_string(),
            Token::POP => "pop".to_string(),
            Token::RET => "ret".to_string(),
            Token::SETL => "setl".to_string(),
            Token::SETLE => "setle".to_string(),
            Token::SETG => "setg".to_string(),
            Token::SETGE => "setge".to_string(),
            Token::SETE => "sete".to_string(),
            Token::SETNE => "setne".to_string(),
            Token::SAR => "sar".to_string(),
            Token::SAL => "sal".to_string(),
            Token::SUB => "sub".to_string(),
            Token::SYSCALL => "syscall".to_string(),
            Token::SYMBOL(name) => name.to_string(),
            Token::INTEGER(num) => format!("INTEGER<{}>", num),
            Token::COLON => "COLON".to_string(),
            Token::COMMA => "COMMA".to_string(),
            Token::MINUS => "MINUS".to_string(),
            Token::LBRACKET => "LBRACKET".to_string(),
            Token::RBRACKET => "RBRACKET".to_string(),
            Token::BLANK => "BLANK".to_string(),
            Token::LF => "LF".to_string(),
            Token::EOF => "EOF".to_string(),
        }
    }
}
pub fn lexing(mut input: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    while let Some((t, idx)) = tokenize(&input) {
        input.drain(..idx);
        if t.should_ignore() {
            continue;
        }
        if let &Token::EOF = &t {
            tokens.push(t);
            break;
        }
        tokens.push(t);
    }
    tokens
}

fn tokenize(input: &String) -> Option<(Token, usize)> {
    if input.len() == 0 {
        return None;
    }
    match input.as_bytes()[0] as char {
        c if c.is_alphabetic() || c == '_' || c == '.' => tokenize_keywords(input),
        c if c == '0' => Some((Token::INTEGER(0), 1)),
        c if is_decimal(c) => {
            let length: usize = count_len(input, |c| c.is_ascii_digit());
            Some((
                Token::INTEGER(input[..length].parse::<i128>().unwrap()),
                length,
            ))
        }
        ' ' => Some((Token::BLANK, count_len(input, |c| c == &' '))),
        _ => tokenize_symbols(input),
    }
}
fn tokenize_symbols(input: &String) -> Option<(Token, usize)> {
    match input.as_bytes()[0] as char {
        ':' => Some((Token::COLON, 1)),
        '[' => Some((Token::LBRACKET, 1)),
        ']' => Some((Token::RBRACKET, 1)),
        '-' => Some((Token::MINUS, 1)),
        ',' => Some((Token::COMMA, 1)),
        ' ' => Some((Token::BLANK, count_len(input, |c| c == &' '))),
        '\n' => Some((Token::LF, 1)),
        '\0' => Some((Token::EOF, 1)),
        c => {
            Error::PARSE.found(&format!("unexpected mark '{}'", c));
            None
        }
    }
}
fn tokenize_keywords(input: &String) -> Option<(Token, usize)> {
    let length: usize = count_len(input, |c| {
        c.is_digit(10) || c == &'_' || c.is_alphabetic() || c == &'.'
    });
    let keywords: Vec<&str> = vec![
        "movzx", "ret", "push", "pop", "cqo", "add", "sub", "idiv", "imul", "cmp", "setle",
        "syscall", "call", "setl", "setge", "setg", "sete", "setne", "lea", "neg", "mov", "jmp",
        "sal", "sar", "jz",
    ];
    let types: Vec<Token> = vec![
        Token::MOVZX,
        Token::RET,
        Token::PUSH,
        Token::POP,
        Token::CQO,
        Token::ADD,
        Token::SUB,
        Token::IDIV,
        Token::IMUL,
        Token::CMP,
        Token::SETLE,
        Token::SYSCALL,
        Token::CALL,
        Token::SETL,
        Token::SETGE,
        Token::SETG,
        Token::SETE,
        Token::SETNE,
        Token::LEA,
        Token::NEG,
        Token::MOV,
        Token::JMP,
        Token::SAL,
        Token::SAR,
        Token::JZ,
    ];
    for (idx, k) in keywords.iter().enumerate() {
        if input.len() >= k.to_string().len() && input[..k.to_string().len()] == k.to_string() {
            return Some((types[idx].clone(), length));
        }
    }
    Some((
        Token::SYMBOL(input.chars().take(length).collect::<String>()),
        length,
    ))
}
fn is_decimal(ch: char) -> bool {
    '1' <= ch && ch <= '9'
}
fn count_len(input: &String, f: fn(ch: &char) -> bool) -> usize {
    input.chars().take_while(f).collect::<String>().len()
}
