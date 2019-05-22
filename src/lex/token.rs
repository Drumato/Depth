#[derive(Clone)]
pub struct Token {
    pub ty: TokenType,
    pub literal: String,
    pub val: TokenVal,
}

pub fn lookup(s: &str) -> bool {
    match s {
        "mut" | "f" | "true" | "false" | "loop" | "for" | "let" | "const" | "if" | "else"
        | "return" | "struct" | "bool" | "str" | "u8" | "u16" | "u32" | "u64" | "u128" | "ch"
        | "i8" | "i16" | "i32" | "i64" | "i128" | "f32" | "f64" => true,
        _ => false,
    }
}

pub fn get_keyword(s: &str) -> TokenType {
    match s {
        "mut" => TokenType::TkMutable,
        "f" => TokenType::TkF,
        "true" => TokenType::TkTrue,
        "false" => TokenType::TkFalse,
        "loop" => TokenType::TkLoop,
        "for" => TokenType::TkFor,
        "let" => TokenType::TkLet,
        "const" => TokenType::TkConst,
        "if" => TokenType::TkIf,
        "else" => TokenType::TkElse,
        "return" => TokenType::TkReturn,
        "struct" => TokenType::TkStruct,
        "bool" => TokenType::TkBool,
        "ch" => TokenType::TkChar,
        "str" => TokenType::TkString,
        "u8" => TokenType::TkU8,
        "u16" => TokenType::TkU16,
        "u32" => TokenType::TkU32,
        "u64" => TokenType::TkU64,
        "u128" => TokenType::TkU128,
        "i8" => TokenType::TkI8,
        "i16" => TokenType::TkI16,
        "i32" => TokenType::TkI32,
        "i64" => TokenType::TkI64,
        "i128" => TokenType::TkI128,
        "f32" => TokenType::TkF32,
        "f64" => TokenType::TkF64,
        _ => TokenType::TkIllegal,
    }
}

impl Token {
    pub fn dump(&self) -> String {
        format!(
            "type:{}\tinput:{}\tval:{}",
            self.ty.string(),
            self.literal,
            self.val.string()
        )
    }
    pub fn new(param: (TokenType, String, TokenVal)) -> Token {
        Token {
            ty: param.0,
            literal: param.1,
            val: param.2,
        }
    }
}

#[derive(Clone)]
pub enum TokenVal {
    IntVal(i64),
    RealVal(f64),
    CharVal(char), //change u32 after
    StrVal(String),
    InVal,
}

impl TokenVal {
    pub fn string(&self) -> String {
        match self {
            TokenVal::IntVal(d) => format!("{}", d),
            TokenVal::RealVal(r) => format!("{}", r),
            TokenVal::CharVal(c) => format!("{}", c),
            TokenVal::StrVal(s) => format!("{}", s),
            _ => "".to_string(),
        }
    }
}

#[derive(Clone)]
pub enum TokenType {
    /* for identifying Type */
    TkIllegal,
    TkIdent,
    TkEof,
    TkFunction,
    TkCharlit,
    TkIntlit,
    TkStrlit,
    TkReallit,

    /* keywords */
    TkMutable, //mut
    TkF,       //f
    TkTrue,    //true
    TkFalse,   //false
    TkLoop,    //loop
    TkFor,     //for
    TkLet,     //let
    TkConst,   //const
    TkIf,      //if
    TkElse,    //else
    TkReturn,  //return
    TkStruct,  //struct

    /* types */
    TkBool,   //bool
    TkChar,   //ch
    TkString, //str
    TkU8,     //u8
    TkU16,    //u16
    TkU32,    //u32
    TkU64,    //u64
    TkU128,   //u128
    TkI8,     //i8
    TkI16,    //i16
    TkI32,    //i32
    TkI64,    //i64
    TkI128,   //i128
    TkF32,    //f32
    TkF64,    //f64

    /* marks */
    TkAssign,
    TkPlus,
    TkIncre,
    TkMinus,
    TkDecre,
    TkBang,
    TkPipe,
    TkAmpersand,
    TkSlash,
    TkStar,
    TkPercent,
    TkPlusassign,
    TkMinusassign,
    TkStarassign,
    TkSlashassign,
    TkPercentassign,
    TkArrow,
    TkBackslash,

    TkLt,
    TkGt,
    TkEq,
    TkNoteq,
    TkLteq,
    TkGteq,
    TkLogor,
    TkLogand,
    TkComma,
    TkSemicolon,
    TkColon,
    TkDot,
    TkLparen,
    TkRparen,
    TkLbrace,
    TkRbrace,
    TkLbracket,
    TkRbracket,
}

impl TokenType {
    pub fn string(&self) -> &str {
        match self {
            TokenType::TkIllegal => "ILLEGAL",
            TokenType::TkIdent => "IDENTIFIER",
            TokenType::TkEof => "EOF",
            TokenType::TkFunction => "FUNCTION",
            TokenType::TkIntlit => "INT-LITERAL",
            TokenType::TkCharlit => "CHAR-LITERAL",
            TokenType::TkStrlit => "STRING-LITERAL",
            TokenType::TkReallit => "REAL-LITERAL",
            TokenType::TkMutable => "MUTABLE",
            TokenType::TkF => "F",
            TokenType::TkTrue => "TRUE",
            TokenType::TkFalse => "FALSE",
            TokenType::TkLoop => "LOOP",
            TokenType::TkFor => "FOR",
            TokenType::TkLet => "LET",
            TokenType::TkConst => "CONST",
            TokenType::TkIf => "IF",
            TokenType::TkElse => "ELSE",     //else
            TokenType::TkReturn => "RETURN", //return
            TokenType::TkStruct => "STRUCT", //struct

            /* types */
            TokenType::TkBool => "BOOL",     //bool
            TokenType::TkChar => "CHAR",     //ch
            TokenType::TkString => "STRING", //str
            TokenType::TkU8 => "U8",         //u8
            TokenType::TkU16 => "U16",       //u16
            TokenType::TkU32 => "U32",       //u32
            TokenType::TkU64 => "U64",       //u64
            TokenType::TkU128 => "U128",
            TokenType::TkI8 => "I8",     //i16
            TokenType::TkI16 => "I16",   //i16
            TokenType::TkI32 => "I32",   //i32
            TokenType::TkI64 => "I64",   //i64
            TokenType::TkI128 => "I128", //i128
            TokenType::TkF32 => "F32",   //f32
            TokenType::TkF64 => "F64",   //f64

            /* marks */
            TokenType::TkAssign => "ASSIGN",
            TokenType::TkEq => "EQ",
            TokenType::TkPlus => "PLUS",
            TokenType::TkIncre => "INCRE",
            TokenType::TkPlusassign => "PLUSASSIGN",
            TokenType::TkMinus => "MINUS",
            TokenType::TkDecre => "DECRE",
            TokenType::TkMinusassign => "MINUSASSIGN",
            TokenType::TkStar => "STAR",
            TokenType::TkStarassign => "STARASSIGN",
            TokenType::TkSlash => "SLASH",
            TokenType::TkSlashassign => "SLASHASSIGN",
            TokenType::TkBang => "BANG",
            TokenType::TkNoteq => "NOTEQ",
            TokenType::TkPipe => "PIPE",
            TokenType::TkLogor => "LOGOR",
            TokenType::TkAmpersand => "AMPERSAND",
            TokenType::TkLogand => "LOGAND",
            TokenType::TkPercent => "PERCENT",
            TokenType::TkPercentassign => "PERCENTASSIGN",
            TokenType::TkArrow => "ARROW",
            TokenType::TkLt => "LT",
            TokenType::TkLteq => "LTEQ",
            TokenType::TkGt => "GT",
            TokenType::TkGteq => "GTEQ",
            TokenType::TkComma => "COMMA",
            TokenType::TkSemicolon => "SEMICOLON",
            TokenType::TkColon => "COLON",
            TokenType::TkDot => "DOT",
            TokenType::TkBackslash => "BACKSLASH",
            TokenType::TkLparen => "LPAREN",
            TokenType::TkRparen => "RPAREN",
            TokenType::TkLbrace => "LBRACE",
            TokenType::TkRbrace => "RBRACE",
            TokenType::TkLbracket => "LBRACKET",
            TokenType::TkRbracket => "RBRACKET",
        }
    }
}
