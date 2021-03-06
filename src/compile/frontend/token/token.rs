#[derive(Eq, PartialEq, Clone)]
pub enum Token {
    /* symbol */
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
    LT,
    GT,
    LTEQ,
    GTEQ,
    EQ,
    NTEQ,
    SEMICOLON,
    COLON,
    DOUBLECOLON,
    COMMA,
    DOT,
    AMPERSAND,

    /* keyword */
    FUNC,
    TYPE,
    RETURN,
    IF,
    ELSE,
    CONDLOOP,
    LET,
    MUT,
    GOTO,
    STRUCT,
    I64,
    COMPINT,
    POINTER(Box<Token>),
    ARRAY(Box<Token>, Box<Token>),
    INFORMATION(String),

    /* etc */
    INTEGER(i128),
    IDENT(String),
    EOF,
    BLANK,
    LF,
    COMMENT,
    HASH,
}
impl Token {
    pub fn name(&self) -> Option<String> {
        if let Self::IDENT(name) = self {
            return Some(name.to_string());
        }
        None
    }
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
            Token::LT => "LESSTHAN".to_string(),
            Token::GT => "GREATERTHAN".to_string(),
            Token::LTEQ => "LESSTHANEQUAL".to_string(),
            Token::GTEQ => "GREATERTHANEQUAL".to_string(),
            Token::EQ => "EQUAL".to_string(),
            Token::NTEQ => "NOTEQUAL".to_string(),
            Token::SEMICOLON => "SEMICOLON".to_string(),
            Token::COLON => "COLON".to_string(),
            Token::DOUBLECOLON => "DOUBLECOLON".to_string(),
            Token::COMMA => "COMMA".to_string(),
            Token::DOT => "DOT".to_string(),
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
            Token::STRUCT => "STRUCT".to_string(),
            Token::I64 => "i64".to_string(),
            Token::COMPINT => "compint".to_string(),
            Token::POINTER(ptr_to) => format!("POINTER<{}>", ptr_to.string()),
            Token::ARRAY(elem_type, ary_size) => {
                format!("ARRAY<{},{}>", elem_type.string(), ary_size.string())
            }
            Token::INFORMATION(_) => "@info".to_string(),
            _ => "".to_string(),
        }
    }
    pub fn should_ignore(&self) -> bool {
        match self {
            Token::BLANK | Token::LF | Token::COMMENT | Token::HASH => true,
            _ => false,
        }
    }
}
