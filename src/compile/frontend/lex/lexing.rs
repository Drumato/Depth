use super::super::super::super::ce::types::Error;
use super::super::token::token::Token;
use std::collections::HashMap;

type TokenLen = usize;

pub fn lexing(mut input: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::with_capacity(2048);

    /* build all keywords they used in depth. */
    let keywords: HashMap<&str, (Token, usize)> = build_keywords();

    /* append this_token to tokens while given tokens are valid. */
    while let Some((t, idx)) = tokenize(&input, &keywords) {
        /* next point. */
        input.drain(..idx);

        if t.should_ignore() {
            continue;
        }

        /* if this_token is End-Of-File then we should exit from tokenize. */
        if let &Token::EOF = &t {
            tokens.push(t);
            break;
        }
        tokens.push(t);
    }

    tokens
}

fn tokenize(input: &String, keywords: &HashMap<&str, (Token, usize)>) -> Option<(Token, TokenLen)> {
    /* return None if can not tokenize */
    if input.len() == 0 {
        return None;
    }
    match input.as_bytes()[0] as char {
        /* keyword and identifier */
        c if c.is_alphabetic() => tokenize_keywords(input, keywords),

        c if c == '0' => Some((Token::INTEGER(0), 1)),

        /* integer-literal */
        c if is_decimal(c) => {
            let length: TokenLen = count_len(input, |c| c.is_ascii_digit());
            Some((
                Token::INTEGER(input[..length].parse::<i128>().unwrap()),
                length,
            ))
        }

        /* ignore comment or Token::SLASH */
        '/' => {
            if input.as_bytes()[1] as char == '/' {
                let length: TokenLen = count_len(input, |c| c != &'\n') + 1;
                return Some((Token::COMMENT, length));
            }
            tokenize_symbols(input)
        }
        /* ignore white-space */
        ' ' => Some((Token::BLANK, count_len(input, |c| c == &' '))),
        /* symbol */
        _ => tokenize_symbols(input),
    }
}
fn tokenize_symbols(input: &String) -> Option<(Token, TokenLen)> {
    if input.len() >= 2 {
        /* check the symbol has multilength at read-offset */
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
        ';' => Some((Token::SEMICOLON, 1)),
        ',' => Some((Token::COMMA, 1)),
        '=' => Some((Token::ASSIGN, 1)),
        '.' => Some((Token::DOT, 1)),
        ' ' => Some((Token::BLANK, count_len(input, |c| c == &' '))),
        '\n' => Some((Token::LF, 1)),
        '\0' => Some((Token::EOF, 1)),
        c => {
            Error::PARSE.found(&format!("unexpected mark '{}'", c));
            None
        }
    }
}
fn tokenize_keywords(
    input: &String,
    keywords: &HashMap<&str, (Token, usize)>,
) -> Option<(Token, TokenLen)> {
    let length: TokenLen = count_len(input, |c| c.is_digit(10) || c == &'_' || c.is_alphabetic());
    if let Some(t) = keywords.get(&input[0..length]) {
        return Some((t.0.clone(), t.1));
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

fn build_keywords() -> HashMap<&'static str, (Token, usize)> {
    let mut keywords: HashMap<&str, (Token, usize)> = HashMap::with_capacity(12);
    keywords.insert("return", (Token::RETURN, 6));
    keywords.insert("if", (Token::IF, 2));
    keywords.insert("else", (Token::ELSE, 4));
    keywords.insert("func", (Token::FUNC, 4));
    keywords.insert("let", (Token::LET, 3));
    keywords.insert("i64", (Token::I64, 3));
    keywords.insert("Pointer", (Token::POINTER(Box::new(Token::EOF)), 7));
    keywords.insert("mut", (Token::MUT, 3));
    keywords.insert("goto", (Token::GOTO, 4));
    keywords.insert(
        "Array",
        (Token::ARRAY(Box::new(Token::EOF), Box::new(Token::EOF)), 5),
    );
    keywords.insert("type", (Token::TYPE, 4));
    keywords.insert("struct", (Token::STRUCT, 6));
    keywords.insert("condloop", (Token::CONDLOOP, 8));
    keywords.insert("compint", (Token::COMPINT, 7));
    keywords
}
