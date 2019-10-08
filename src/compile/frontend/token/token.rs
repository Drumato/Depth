#[derive(Eq, PartialEq, Clone)]
pub enum Token {
    INTEGER(i128),
    PLUS,
    MINUS,
    STAR,
    SLASH,
    PERCENT,
    ASSIGN,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    LBRACKET,
    RBRACKET,
    LSHIFT,
    RSHIFT,
    LT,
    GT,
    LTEQ,
    GTEQ,
    EQ,
    NTEQ,
    SEMICOLON,
    COLON,
    COMMA,
    AMPERSAND,

    FUNC,
    TYPE,
    IDENT(String),
    RETURN,
    IF,
    ELSE,
    CONDLOOP,
    LET,
    MUT,
    I64,
    POINTER(Box<Token>),
    ARRAY(Box<Token>, Box<Token>),
    CHAR,
    CHARLIT(char),
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
            Token::ASSIGN => "ASSIGN".to_string(),
            Token::LPAREN => "LPAREN".to_string(),
            Token::RPAREN => "RPAREN".to_string(),
            Token::LBRACE => "LBRACE".to_string(),
            Token::RBRACE => "RBRACE".to_string(),
            Token::LBRACKET => "LBRACKET".to_string(),
            Token::RBRACKET => "RBRACKET".to_string(),
            Token::LSHIFT => "LSHIFT".to_string(),
            Token::RSHIFT => "RSHIFT".to_string(),
            Token::LT => "LESSTHAN".to_string(),
            Token::GT => "GREATERTHAN".to_string(),
            Token::LTEQ => "LESSTHANEQUAL".to_string(),
            Token::GTEQ => "GREATERTHANEQUAL".to_string(),
            Token::EQ => "EQUAL".to_string(),
            Token::NTEQ => "NOTEQUAL".to_string(),
            Token::SEMICOLON => "SEMICOLON".to_string(),
            Token::COLON => "COLON".to_string(),
            Token::COMMA => "COMMA".to_string(),
            Token::AMPERSAND => "AMPERSAND".to_string(),
            Token::RETURN => "RETURN".to_string(),
            Token::EOF => "EOF".to_string(),
            Token::FUNC => "FUNCTION".to_string(),
            Token::TYPE => "TYPE".to_string(),
            Token::IDENT(name) => format!("IDENTIFIER<{}>", name),
            Token::IF => "IF".to_string(),
            Token::ELSE => "ELSE".to_string(),
            Token::CONDLOOP => "CONDLOOP".to_string(),
            Token::LET => "LET".to_string(),
            Token::MUT => "MUTABLE".to_string(),
            Token::I64 => "i64".to_string(),
            Token::POINTER(ptr_to) => format!("POINTER<{}>", ptr_to.string()),
            Token::ARRAY(elem_type, ary_size) => {
                format!("ARRAY<{},{}>", elem_type.string(), ary_size.string())
            }
            Token::CHAR => "CHAR".to_string(),
            Token::CHARLIT(char_val) => format!("CHARLIT<{}>", char_val),
            _ => "".to_string(),
        }
    }
    pub fn string_ir(&self) -> String {
        match self {
            Token::PLUS => "+".to_string(),
            Token::MINUS => "-".to_string(),
            Token::STAR => "*".to_string(),
            Token::SLASH => "/".to_string(),
            Token::PERCENT => "%".to_string(),
            Token::LSHIFT => "<<".to_string(),
            Token::RSHIFT => ">>".to_string(),
            Token::LT => "<".to_string(),
            Token::GT => ">".to_string(),
            Token::LTEQ => "<=".to_string(),
            Token::GTEQ => ">=".to_string(),
            Token::EQ => "==".to_string(),
            Token::NTEQ => "!=".to_string(),
            Token::AMPERSAND => "&".to_string(),
            _ => "(inv)".to_string(),
        }
    }
    pub fn start_stmt(token: &Token) -> Option<()> {
        match token {
            Token::LET
            | Token::LBRACE
            | Token::CONDLOOP
            | Token::RETURN
            | Token::IF
            | Token::IDENT(_)
            | Token::LPAREN
            | Token::INTEGER(_) => Some(()),
            t => {
                if t == &Token::EOF {
                    return None;
                }
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
