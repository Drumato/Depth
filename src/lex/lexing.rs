use super::super::token::token::Token;

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
        '(' => Some((Token::LPAREN, 1)),
        ')' => Some((Token::RPAREN, 1)),
        ' ' => Some((Token::BLANK, count_len(input, |c| c == &' '))),
        '\n' => Some((Token::LF, 1)),
        '\0' => Some((Token::EOF, 1)),
        _ => None,
    }
}
fn is_decimal(ch: char) -> bool {
    '1' <= ch && ch <= '9'
}
fn count_len(input: &String, f: fn(ch: &char) -> bool) -> usize {
    input.chars().take_while(f).collect::<String>().len()
}
fn tokenize_multisymbols(input: &String) -> Option<Token> {
    match input.as_str() {
        "<<" => Some(Token::LSHIFT),
        ">>" => Some(Token::RSHIFT),
        _ => None,
    }
}
