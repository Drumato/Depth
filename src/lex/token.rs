#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub ty: TokenType,
    pub literal: String,
    pub val: TokenVal,
}

pub fn lookup(s: &str) -> bool {
    match s {
        "mut" | "f" | "true" | "false" | "loop" | "for" | "in" | "let" | "const" | "if"
        | "else" | "return" | "struct" | "bool" | "str" | "u8" | "u16" | "u32" | "u64" | "u128"
        | "ch" | "i8" | "i16" | "i32" | "i64" | "i128" | "f32" | "f64" => true,
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
        "in" => TokenType::TkIn,
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

#[derive(Debug, Clone, PartialEq)]
pub enum TokenVal {
    IntVal(i128),
    UintVal(u128),
    RealVal(f64),
    CharVal(char), //change u32 after
    StrVal(String),
    InVal,
}

impl TokenVal {
    pub fn string(&self) -> String {
        match self {
            TokenVal::IntVal(d) => format!("{}", d),
            TokenVal::UintVal(d) => format!("{}", d),
            TokenVal::RealVal(r) => format!("{}", r),
            TokenVal::CharVal(c) => format!("{}", c),
            TokenVal::StrVal(s) => format!("{}", s),
            _ => "".to_string(),
        }
    }
}

pub enum IntType {
    I8,
    I16,
    I32,
    I64,
    I128,
    INVALID,
}

impl IntType {
    pub fn judge(sem_val: i128) -> IntType {
        match sem_val {
            n if (127 >= n && n >= -128) => IntType::I8,
            n if (32767 >= n && n >= -32768) => IntType::I16,
            n if (2147483647 >= n && n >= -2147483648) => IntType::I32,
            n if (9223372036854775807 >= n && n >= -9223372036854775808) => IntType::I64,
            n if (170141183460469231731687303715884105727 >= n
                && -170141183460469231731687303715884105728 >= n) =>
            {
                IntType::I128
            }
            _ => IntType::INVALID,
        }
    }
}

pub enum UintType {
    U8,
    U16,
    U32,
    U64,
    U128,
    INVALID,
}

impl UintType {
    pub fn judge(sem_val: u128) -> UintType {
        match sem_val {
            n if (255 >= n) => UintType::U8,
            n if (65535 >= n) => UintType::U16,
            n if (4294967295 >= n) => UintType::U32,
            n if (18446744073709551615 >= n) => UintType::U64,
            n if (340282366920938463463374607431768211455 >= n) => UintType::U128,
            _ => UintType::INVALID,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    /* for identifying Type */
    TkIllegal,
    TkIdent,
    TkEof,
    TkFunction,
    TkCharlit,
    TkIntlit,
    TkUintlit,
    TkStrlit,
    TkReallit,

    /* keywords */
    TkMutable, //mut
    TkF,       //f
    TkTrue,    //true
    TkFalse,   //false
    TkLoop,    //loop
    TkFor,     //for
    TkIn,      //in
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
    TkLshift,
    TkRshift,
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
    TkExp,
    TkTerm,
    TkAtom,
}

impl TokenType {
    pub fn string(&self) -> &str {
        match self {
            TokenType::TkIllegal => "ILLEGAL",
            TokenType::TkIdent => "IDENTIFIER",
            TokenType::TkEof => "EOF",
            TokenType::TkFunction => "FUNCTION",
            TokenType::TkIntlit => "INT-LITERAL",
            TokenType::TkUintlit => "UINT-LITERAL",
            TokenType::TkCharlit => "CHAR-LITERAL",
            TokenType::TkStrlit => "STRING-LITERAL",
            TokenType::TkReallit => "REAL-LITERAL",
            TokenType::TkMutable => "MUTABLE",
            TokenType::TkF => "F",
            TokenType::TkTrue => "TRUE",
            TokenType::TkFalse => "FALSE",
            TokenType::TkLoop => "LOOP",
            TokenType::TkFor => "FOR",
            TokenType::TkIn => "IN",
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
            TokenType::TkLshift => "LSHIFT",
            TokenType::TkRshift => "LSHIFT",
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
            TokenType::TkExp => "EXP",
            TokenType::TkTerm => "TERM",
            TokenType::TkAtom => "ATOM",
        }
    }
    pub fn is_typename(&self) -> bool {
        match self {
            TokenType::TkBool
            | TokenType::TkChar
            | TokenType::TkString
            | TokenType::TkU8
            | TokenType::TkU16
            | TokenType::TkU32
            | TokenType::TkU64
            | TokenType::TkU128
            | TokenType::TkI8
            | TokenType::TkI16
            | TokenType::TkI32
            | TokenType::TkI64
            | TokenType::TkI128
            | TokenType::TkF32
            | TokenType::TkF64 => true,
            _ => false,
        }
    }
}
