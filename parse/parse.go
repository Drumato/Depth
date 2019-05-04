package parse

import (
	"depth/lex"
	"depth/token"
	"os"

	"github.com/sirupsen/logrus"
)

type Parser struct { //recursive-descent parser
	l        *lex.Lexer
	errors   []Error
	warnings []Warning

	curToken token.Token
}

func New(l *lex.Lexer) *Parser {
	p := &Parser{l: l}
	p.nextToken()
	p.nextToken()
	return p
}

func Parse() {
	p.curToken = p.l.NextToken()
}

func (p *Parser) expect(t token.TokenType) {
	if p.curToken.Type != t {
		logrus.Errorf("%s expected, but got %s", t, p.curToken.Literal)
		os.Exit(1)
	}
	p.nextToken()
}

func (p *Parser) consume(ty token.TokenType) bool {
	if p.curToken.Type != ty {
		logrus.Errorf("expected %s but got %s", ty, p.curToken.Type)
		return false
	}
	p.nextToken()
	return true
}
