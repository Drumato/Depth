use super::super::super::super::ce::types::Error;
use super::super::token::token::Token;

type TokenLen = usize;
pub fn lexing(mut input: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::with_capacity(2048);
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

fn tokenize(input: &String) -> Option<(Token, TokenLen)> {
    if input.len() == 0 {
        return None;
    }
    match input.as_bytes()[0] as char {
        c if c.is_alphabetic() => tokenize_keywords(input),

        c if c == '0' => Some((Token::INTEGER(0), 1)),
        c if is_decimal(c) => {
            let length: TokenLen = count_len(input, |c| c.is_ascii_digit());
            Some((
                Token::INTEGER(input[..length].parse::<i128>().unwrap()),
                length,
            ))
        }
        '\'' => {
            let char_val: char = input[1..].as_bytes()[0] as char;
            Some((Token::CHARLIT(char_val), 3))
        }
        ' ' => Some((Token::BLANK, count_len(input, |c| c == &' '))),
        _ => tokenize_symbols(input),
    }
}
fn tokenize_symbols(input: &String) -> Option<(Token, TokenLen)> {
    if input.len() >= 2 {
        let multilength: String = std::str::from_utf8(&input.as_bytes()[0..2]).unwrap().into();
        if let Some(t) = tokenize_multisymbols(&multilength) {
            return Some((t, 2));
        }
    }
    match input.as_bytes()[0] as char {
        '+' => Some((Token::PLUS, 1)),
        '-' => Some((Token::MINUS, 1)),
        '*' => Some((Token::STAR, 1)),
        '/' => Some((Token::SLASH, 1)),
        '%' => Some((Token::PERCENT, 1)),
        '&' => Some((Token::AMPERSAND, 1)),
        '(' => Some((Token::LPAREN, 1)),
        ')' => Some((Token::RPAREN, 1)),
        '{' => Some((Token::LBRACE, 1)),
        '}' => Some((Token::RBRACE, 1)),
        '[' => Some((Token::LBRACKET, 1)),
        ']' => Some((Token::RBRACKET, 1)),
        '<' => Some((Token::LT, 1)),
        '>' => Some((Token::GT, 1)),
        ':' => Some((Token::COLON, 1)),
        ',' => Some((Token::COMMA, 1)),
        '=' => Some((Token::ASSIGN, 1)),
        ' ' => Some((Token::BLANK, count_len(input, |c| c == &' '))),
        '\n' => Some((Token::LF, 1)),
        '\0' => Some((Token::EOF, 1)),
        c => {
            Error::PARSE.found(&format!("unexpected mark '{}'", c));
            None
        }
    }
}
fn tokenize_keywords(input: &String) -> Option<(Token, TokenLen)> {
    let length: TokenLen = count_len(input, |c| c.is_digit(10) || c == &'_' || c.is_alphabetic());
    let keywords: Vec<&str> = vec![
        "return", "if", "else", "func", "let", "i8", "i16", "i32", "i64", "Pointer", "ch", "mut",
    ];
    let types: Vec<Token> = vec![
        Token::RETURN,
        Token::IF,
        Token::ELSE,
        Token::FUNC,
        Token::LET,
        Token::I8,
        Token::I16,
        Token::I32,
        Token::I64,
        Token::POINTER(Box::new(Token::EOF)),
        Token::CHAR,
        Token::MUT,
    ];
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
fn count_len(input: &String, f: fn(ch: &char) -> bool) -> TokenLen {
    input.chars().take_while(f).collect::<String>().len()
}
fn tokenize_multisymbols(input: &String) -> Option<Token> {
    match input.as_str() {
        "<<" => Some(Token::LSHIFT),
        "<=" => Some(Token::LTEQ),
        ">=" => Some(Token::GTEQ),
        "==" => Some(Token::EQ),
        "!=" => Some(Token::NTEQ),
        _ => None,
    }
}
