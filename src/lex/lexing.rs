use super::super::token::token::Token;

pub fn lexing(mut input: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    while let Some((t, idx)) = tokenize(&input) {
        input.drain(..idx);
        if should_ignore(&t) {
            continue;
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
    match input.as_bytes()[0] as char {
        '+' => Some((Token::PLUS, 1)),
        '-' => Some((Token::MINUS, 1)),
        ' ' => Some((Token::BLANK, count_len(input, |c| c == &' '))),
        '\n' => Some((Token::LF, 1)),
        _ => None,
    }
}
fn is_decimal(ch: char) -> bool {
    '1' <= ch && ch <= '9'
}
fn should_ignore(t: &Token) -> bool {
    match t {
        Token::BLANK | Token::LF => true,
        _ => false,
    }
}
fn count_len(input: &String, f: fn(ch: &char) -> bool) -> usize {
    input.chars().take_while(f).collect::<String>().len()
}
