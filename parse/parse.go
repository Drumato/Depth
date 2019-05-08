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

func (p *Parser) term() *Node {
	switch p.curToken.Type {
	case token.INTLIT:
		defer p.nextToken()
		return NewNodeNum(p.curToken.IntVal)
	case token.CHARLIT:
		defer p.nextToken()
		return NewNodeChar(p.curToken.Literal)
	case token.IDENT:
		defer p.nextToken()
		return &Node{Name: p.curToken.Literal, Type: ND_IDENT, IntVal: variables[p.curToken.Literal].IntVal}
	default:
		logrus.Errorf("number expected, but got %s", p.curToken.Literal)
	}
	return nil
}

func (p *Parser) mul() *Node {
	lop := p.term()
	for {
		t := p.curToken
		if t.Type != token.ASTERISK && t.Type != token.SLASH {
			break
		}
		p.nextToken()
		lop = NewNode(NodeType(t.Type), lop, p.term())
	}
	return lop
}

func (p *Parser) stmt() *Node {
	n := &Node{}
	switch p.curToken.Type {
	case token.RETURN:
		n.Type = ND_RETURN
		p.nextToken()
		n.Expression = p.expr()
	case token.LET:
		n.Type = ND_DEFINE
		p.nextToken()
		n.Identifier = p.define()
		n.Expression = p.expr()
		variables[n.Identifier.Name].IntVal = n.Expression.IntVal
	default:
		logrus.Errorf("invalid statement startswith %s", p.curToken.Literal)
		p.nextToken()
	}
	return n
}

func isTypename(t token.Token) bool {
	switch t.Type {
	case token.I8, token.I16, token.I32, token.I64, token.U8, token.U16, token.U32, token.U64, token.F32, token.F64, token.CHAR, token.BOOL:
		return true
	}
	return false
}

func (p *Parser) define() *Node {
	n := &Node{}
	if p.curToken.Type != token.IDENT {
		logrus.Errorf("identifer expected,but got %s", p.curToken.Literal)
	}
	n.Name = p.curToken.Literal
	n.Type = ND_IDENT
	p.expect(token.COLON)
	p.nextToken()
	if !isTypename(p.curToken) {
		logrus.Errorf("expected type declaration,but got %s", p.curToken.Literal)
	}
	n.ElementType = &Element{Type: p.curToken.Type, Stacksize: stackTable[p.curToken.Literal]}
	variables[n.Name] = n
	p.expect(token.ASSIGN)
	p.nextToken()
	return n
}

func (p *Parser) add() *Node {
	lop := p.mul()
	for {
		t := p.curToken
		if t.Type != token.PLUS && t.Type != token.MINUS {
			break
		}
		p.nextToken()
		lop = NewNode(NodeType(t.Type), lop, p.mul())
	}
	return lop
}

func (p *Parser) expr() *Node {
	lop := p.add()
	for {
		t := p.curToken
		if t.Type != token.LT && t.Type != token.GT {
			break
		}
		p.nextToken()
		lop = NewNode(NodeType(t.Type), lop, p.add())
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
		for {
			if p.curToken.Type == token.RBRACE {
				break
			}
			fn.Nodes = append(fn.Nodes, p.stmt())
		}
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
