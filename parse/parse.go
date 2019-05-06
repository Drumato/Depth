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
	nexToken token.Token
}

func New(l *lex.Lexer) *Parser {
	p := &Parser{l: l, nexToken: l.NextToken()}
	p.nextToken()
	return p
}

func (p *Parser) nextToken() {
	p.curToken = p.nexToken
	p.nexToken = p.l.NextToken()
}

func (p *Parser) expect(t token.TokenType) {
	if p.nexToken.Type != t {
		logrus.Errorf("%s expected, but got %s", t, p.nexToken.Literal)
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

func (p *Parser) mul() *Node {
	lop := p.number()
	for {
		t := p.curToken
		if t.Type != token.ASTERISK && t.Type != token.SLASH {
			break
		}
		p.nextToken()
		lop = NewNode(NodeType(t.Type), lop, p.number())
	}
	return lop
}

func (p *Parser) expr() *Node {
	lop := p.mul()
	for {
		t := p.curToken
		if t.Type != token.PLUS && t.Type != token.MINUS {
			break
		}
		p.nextToken()
		lop = NewNode(NodeType(t.Type), lop, p.mul())
	}
	if p.curToken.Type != token.RBRACE {
		logrus.Errorf("stray token: %s", p.curToken.Literal)
	}
	return lop
}

func (p *Parser) function() *Function {
	fn := &Function{}
	if p.consume(token.FUNCTION) {
		if p.curToken.Type != token.IDENT {
			logrus.Errorf("identifer expected,but got %s", p.curToken.Literal)
		}
		fn.Name = p.curToken.Literal
		p.expect(token.LPAREN) //yet ignored arguments
		p.expect(token.RPAREN)
		p.expect(token.LBRACE)
		p.nextToken()
		fn.Nodes = append(fn.Nodes, p.expr())
	}
	p.expect(token.EOF)
	return fn
}

func (p *Parser) Parse() *RootNode {
	functions := make(map[string]*Function)
	rn := &RootNode{Functions: functions}
	fn := p.function()
	rn.Functions[fn.Name] = fn
	return rn
}
