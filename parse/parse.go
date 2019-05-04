package parse

import (
	"drum/gocc/lexer"
	"go/token"
)

type Parser struct { //recursive-descent parser
	l        *lexer.Lexer
	errors   []Error
	warnings []Warning

	prevToken token.Token
	curToken  token.Token
	peekToken token.Token
}

func Parse() {
}
