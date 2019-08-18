#[derive(Eq, PartialEq)]
pub enum Token {
    INTEGER(i128),
    PLUS,
    MINUS,
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
            Token::EOF => "EOF".to_string(),
            _ => "".to_string(),
        }
    }
}
