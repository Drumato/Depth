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
	return p
}

func (p *Parser) nextToken() {
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

func (p *Parser) number() *Node {
	if p.curToken.Type == token.INTLIT {
		defer p.nextToken()
		return NewNodeNum(p.curToken.IntVal)
	}
	logrus.Errorf("number expected, but got %s", p.curToken.Literal)
	return nil
}

func (p *Parser) expr() *Node {
	lhs := p.number()
	for {
		t := p.curToken
		if t.Type != token.PLUS && t.Type != token.MINUS {
			break
		}
		p.nextToken()
		lhs = NewNode(NodeType(t.Type), lhs, p.number())
	}
	if p.curToken.Type != token.EOF {
		logrus.Errorf("stray token: %s", p.curToken.Literal)
	}
	return lhs
}

func (p *Parser) Parse() *Node {
	return p.expr()
}
