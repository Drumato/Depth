package token

//TokenType is used to simplify token types
type TokenType string

//Token is used to classify each token appeared in files
type Token struct {
	Type     TokenType
	Literal  string
	IntVal   int64
	FloatVal float64
}

const (
	//ILLEGAL indicates forbidden tokens
	ILLEGAL = "ILLEGAL"
	//EOF indicates the end of files
	EOF = "EOF"
	//BLANK indicates whitespaces
	BLANK = " "

	//ASSIGN indicates assigning variables
	ASSIGN = "="
	//PLUS indicates accumulates operands
	PLUS           = "+"
	INCRE          = "++"
	MINUS          = "-"
	DECRE          = "--"
	BANG           = "!"
	PIPE           = "|"
	AMPERSAND      = "&"
	SLASH          = "/"
	ASTERISK       = "*"
	PERCENT        = "%"
	PLUSASSIGN     = "+="
	MINUSASSIGN    = "-="
	ASTERISKASSIGN = "*="
	SLASHASSIGN    = "/="
	PERCENTASSIGN  = "%="

	LT        = "<"
	GT        = ">"
	EQ        = "=="
	NOT_EQ    = "!="
	LTEQ      = "<="
	GTEQ      = ">="
	LOGOR     = "||"
	LOGAND    = "&&"
	COMMA     = ","
	SEMICOLON = ";"
	COLON     = ":"
	DOT       = "."
	LPAREN    = "("
	RPAREN    = ")"
	LBRACE    = "{"
	RBRACE    = "}"
	LBRACKET  = "["
	RBRACKET  = "]"

	//type
	FUNCTION = "FUNCTION"
	COMMENT  = "COMMENT"
	IDENT    = "IDENT"

	I8   = "I8"
	I16  = "I16"
	I32  = "I32"
	I64  = "I64"
	U8   = "U8"
	U16  = "U16"
	U32  = "U32"
	U64  = "U64"
	F32  = "F32"
	F64  = "F64"
	BOOL = "BOOL"
	CHAR = "CHAR"

	INTLIT    = "INTLIT"
	CHARLIT   = "CHARLIT"
	STRINGLIT = "STRINGLIT"
	FLOATLIT  = "FLOATLIT"

	CONST  = "CONST"
	STRUCT = "STRUCT"

	TRUE    = "TRUE"
	FALSE   = "FALSE"
	IF      = "IF"
	FOR     = "FOR"
	MUTABLE = "MUTABLE"
	LET     = "LET"
	STRING  = "STRING"
	LOOP    = "LOOP"
	RETURN  = "RETURN"
)

var keywords = map[string]TokenType{
	"mut":    MUTABLE,
	"true":   TRUE,
	"false":  FALSE,
	"bool":   BOOL,
	"let":    LET,
	"const":  CONST,
	"ch":     CHAR,
	"str":    STRING,
	"f32":    F32,
	"f64":    F64,
	"for":    FOR,
	"loop":   LOOP,
	"if":     IF,
	"i8":     I8,
	"i16":    I16,
	"i32":    I32,
	"i64":    I64,
	"u8":     U8,
	"u16":    U16,
	"u32":    U32,
	"u64":    U64,
	"return": RETURN,
}

//LookupIdent judge whether Keywords include the token or not
func LookupIdent(ident string) TokenType {
	if tok, ok := keywords[ident]; ok {
		return tok
	}
	return IDENT
}
