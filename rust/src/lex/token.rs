pub struct Token {
    ty: TokenType,
    literal: String,
    //val:,
}

pub fn new_token(param: (TokenType, String)) -> Token {
    Token {
        ty: param.0,
        literal: param.1,
    }
}

impl Token {
    pub fn dump(&self) -> String {
        format!(
            "type:{tokentype}\tinput:{tokenliteral}\n",
            tokentype = self.ty.string(),
            tokenliteral = self.literal
        )
    }
}

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
            TokenType::TkIdent => "IDENTIFER",
            TokenType::TkEof => "EOF",
            TokenType::TkFunction => "FUNCTION",
            TokenType::TkIntlit => "INT-LITERAL",
            TokenType::TkCharlit => "CHAR-LITERAL",
            TokenType::TkStrlit => "STRING-LITERAL",
            TokenType::TkReallit => "REAL-LITERAL",
            TokenType::TkMutable => "mut",
            TokenType::TkF => "f",
            TokenType::TkTrue => "true",
            TokenType::TkFalse => "false",
            TokenType::TkLoop => "loop",
            TokenType::TkFor => "for",
            TokenType::TkLet => "let",
            TokenType::TkConst => "const",
            TokenType::TkIf => "if",
            TokenType::TkElse => "else",     //else
            TokenType::TkReturn => "return", //return
            TokenType::TkStruct => "struct", //struct

            /* types */
            TokenType::TkBool => "bool",  //bool
            TokenType::TkChar => "ch",    //ch
            TokenType::TkString => "str", //str
            TokenType::TkU8 => "u8",      //u8
            TokenType::TkU16 => "u16",    //u16
            TokenType::TkU32 => "u32",    //u32
            TokenType::TkU64 => "u64",    //u64
            TokenType::TkU128 => "u128",
            TokenType::TkI8 => "i8",     //i16
            TokenType::TkI16 => "i16",   //i16
            TokenType::TkI32 => "i32",   //i32
            TokenType::TkI64 => "i64",   //i64
            TokenType::TkI128 => "i128", //i128
            TokenType::TkF32 => "f32",   //f32
            TokenType::TkF64 => "f64",   //f64

            /* marks */
            TokenType::TkAssign => "=",
            TokenType::TkEq => "==",
            TokenType::TkPlus => "+",
            TokenType::TkIncre => "++",
            TokenType::TkPlusassign => "+=",
            TokenType::TkMinus => "-",
            TokenType::TkDecre => "--",
            TokenType::TkMinusassign => "-=",
            TokenType::TkBang => "!",
            TokenType::TkNoteq => "!=",
            TokenType::TkPipe => "|",
            TokenType::TkLogor => "||",
            TokenType::TkAmpersand => "&",
            TokenType::TkLogand => "&&",
            TokenType::TkStar => "*",
            TokenType::TkStarassign => "*=",
            TokenType::TkSlash => "/",
            TokenType::TkSlashassign => "/=",
            TokenType::TkPercent => "%",
            TokenType::TkPercentassign => "%=",
            TokenType::TkArrow => "->",
            TokenType::TkLt => "<",
            TokenType::TkLteq => "<=",
            TokenType::TkGt => ">",
            TokenType::TkGteq => ">=",
            TokenType::TkComma => ",",
            TokenType::TkSemicolon => ";",
            TokenType::TkColon => ":",
            TokenType::TkDot => ".",
            TokenType::TkBackslash => "\\",
            TokenType::TkLparen => "(",
            TokenType::TkRparen => ")",
            TokenType::TkLbrace => "{",
            TokenType::TkRbrace => "}",
            TokenType::TkLbracket => "[",
            TokenType::TkRbracket => "]",
        }
    }
}
