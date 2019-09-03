use super::super::ce::types::Error;
#[derive(Eq, PartialEq, Clone)]
pub enum Token {
    INTEGER(i128),
    COMMA,
    IDENT(String),
    EOF,
    BLANK,
    LF,
}
impl Token {
    fn should_ignore(&self) -> bool {
        match self {
            Token::BLANK | Token::LF | Token::COMMA => true,
            _ => false,
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
        c if c.is_alphabetic() => tokenize_keywords(input),
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
    let length: usize = count_len(input, |c| c.is_digit(10) || c == &'_' || c.is_alphabetic());
    let keywords: Vec<&str> = vec![];
    let types: Vec<Token> = vec![];
    for (idx, k) in keywords.iter().enumerate() {
        if input.starts_with(k) {
            return Some((types[idx].clone(), length));
        }
    }
    Some((
        Token::IDENT(input.chars().take(length).collect::<String>()),
        length,
    ))
}
fn is_decimal(ch: char) -> bool {
    '1' <= ch && ch <= '9'
}
fn count_len(input: &String, f: fn(ch: &char) -> bool) -> usize {
    input.chars().take_while(f).collect::<String>().len()
}
