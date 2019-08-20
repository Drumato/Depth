use super::super::ce::types::Error;
#[derive(Eq, PartialEq, Clone)]
pub enum Token {
    INTEGER(i128),
    PLUS,
    MINUS,
    STAR,
    SLASH,
    PERCENT,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    LSHIFT,
    RSHIFT,
    LT,
    GT,
    LTEQ,
    GTEQ,
    EQ,
    NTEQ,

    FUNC,
    IDENT(String),
    RETURN,
    IF,
    EOF,
    BLANK,
    LF,
}
impl Token {
    pub fn string(&self) -> String {
        match self {
            Token::INTEGER(int) => format!("INTEGER<{}>", int),
            Token::PLUS => "PLUS".to_string(),
            Token::MINUS => "MINUS".to_string(),
            Token::STAR => "STAR".to_string(),
            Token::SLASH => "SLASH".to_string(),
            Token::PERCENT => "PERCENT".to_string(),
            Token::LPAREN => "LPAREN".to_string(),
            Token::RPAREN => "RPAREN".to_string(),
            Token::LBRACE => "LBRACE".to_string(),
            Token::RBRACE => "RBRACE".to_string(),
            Token::LSHIFT => "LSHIFT".to_string(),
            Token::RSHIFT => "RSHIFT".to_string(),
            Token::LT => "LESSTHAN".to_string(),
            Token::GT => "GREATERTHAN".to_string(),
            Token::LTEQ => "LESSTHANEQUAL".to_string(),
            Token::GTEQ => "GREATERTHANEQUAL".to_string(),
            Token::EQ => "EQUAL".to_string(),
            Token::NTEQ => "NOTEQUAL".to_string(),
            Token::RETURN => "RETURN".to_string(),
            Token::EOF => "EOF".to_string(),
            Token::FUNC => "FUNCTION".to_string(),
            Token::IDENT(name) => format!("IDENTIFIER<{}>", name),
            Token::IF => "IF".to_string(),
            _ => "".to_string(),
        }
    }
    pub fn start_stmt(token: &Token) -> Option<()> {
        match token {
            Token::RETURN | Token::IF | Token::LPAREN | Token::INTEGER(_) => Some(()),
            t => {
                if t == &Token::EOF {
                    return None;
                }
                Error::PARSE.found(&format!(
                    "unexpected {} while parsing statement",
                    token.string()
                ));
                None
            }
        }
    }
    pub fn should_ignore(&self) -> bool {
        match self {
            Token::BLANK | Token::LF => true,
            _ => false,
        }
    }
}
