package parse

import (
	"depth/golang/lex"
	"depth/golang/token"
	"fmt"
	"os"
)

var (
	scopeLevel uint8 = 0
	m          *Manager
)

func New(l *lex.Lexer) *Parser {
	p := &Parser{l: l, nexToken: l.NextToken()}
	p.nextToken()
	return p
}

func (p *Parser) nextToken() {
	if p.curToken.Type == token.RBRACE {
		scopeLevel--
	}
	p.curToken = p.nexToken
	p.nexToken = p.l.NextToken()
}

func (p *Parser) expect(t token.TokenType) {
	if p.nexToken.Type != t {
		FoundError(NewError(ParseError, fmt.Sprintf("%s expected, but got %s", t, p.nexToken.Type)))
		os.Exit(1)
	}
	p.nextToken()
}

func (p *Parser) consume(ty token.TokenType) bool {
	if p.curToken.Type != ty {
		FoundError(NewError(ParseError, fmt.Sprintf("%s expected, but got %s", ty, p.curToken.Type)))
		return false
	}
	p.nextToken()
	return true
}

func (p *Parser) term() *Node {
	switch p.curToken.Type {
	case token.INTLIT:
		defer p.nextToken()
		return NewNodeNum(p.curToken.IntVal, scopeLevel)
	case token.FLOATLIT:
		defer p.nextToken()
		return NewNodeFloat(p.curToken.FloatVal, scopeLevel)
	case token.CHARLIT:
		defer p.nextToken()
		return NewNodeChar(p.curToken.Literal, scopeLevel)
	case token.IDENT:
		var ident *Node
		var ok bool
		i := int(scopeLevel)
		for {
			if i < 1 {
				FoundError(NewError(InvalidReferenceError, fmt.Sprintf("cannot find '%s' in this scope", p.curToken.Literal)))
				os.Exit(1)
			}
			if ident, ok = m.EnvTable[i].Variables[p.curToken.Literal]; ok {
				break
			}
			i--
		}
		n := &Node{Name: p.curToken.Literal, Type: ND_IDENT, Level: scopeLevel, IntVal: ident.IntVal, ElementType: m.EnvTable[i].Variables[p.curToken.Literal].ElementType}
		switch n.ElementType.Type {
		case token.I8, token.I16, token.I32, token.I64, token.I128:
			n.IntVal = ident.IntVal
		case token.F32, token.F64:
			n.FloatVal = ident.FloatVal
		case token.CHAR:
			n.CharVal = ident.CharVal
		}
		defer p.nextToken()
		return n
	default:
		FoundError(NewError(ParseError, fmt.Sprintf("number expected, but got %s", p.curToken.Literal)))
		os.Exit(1)
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
		lop = NewNode(NodeType(t.Type), lop, p.term(), scopeLevel)
	}
	return lop
}

func (p *Parser) stmt() *Node {
	n := &Node{}
	switch p.curToken.Type {
	case token.IF:
		n.Type = ND_IF
		p.nextToken()
		n.Condition = p.expr()
		p.consume(token.LBRACE)
		scopeLevel++
		n.Level = scopeLevel
		m.EnvTable[int(n.Level)] = newEnv(int(n.Level))
		for {
			if p.curToken.Type == token.RBRACE {
				break
			}
			st := p.stmt()
			n.Body = append(n.Body, st)
			if st.Type == ND_RETURN {
				for {
					if p.curToken.Type == token.RBRACE {
						break
					}
					p.nextToken()
				}
			}
		}
		if p.nexToken.Type == token.ELSE {
			p.nextToken()
			p.nextToken()
			p.nextToken()
			for {
				if p.curToken.Type == token.RBRACE {
					break
				}
				st := p.stmt()
				n.Alternative = append(n.Alternative, st)
				if st.Type == ND_RETURN {
					for {
						if p.curToken.Type == token.RBRACE {
							break
						}
						p.nextToken()
					}
				}
			}
		}
		p.nextToken()
	case token.RETURN:
		n.Level = scopeLevel
		n.Type = ND_RETURN
		p.nextToken()
		n.Expression = p.expr()
	case token.LET:
		n.Level = scopeLevel
		n.Type = ND_DEFINE
		p.nextToken()
		n.Identifier = p.define()
		n.Expression = p.expr()
		switch n.Identifier.ElementType.Type {
		case token.I8, token.I16, token.I32, token.I64, token.I128:
			m.EnvTable[int(n.Level)].Variables[n.Identifier.Name].IntVal = n.Expression.IntVal
		case token.F32, token.F64:
			m.EnvTable[int(n.Level)].Variables[n.Identifier.Name].FloatVal = n.Expression.FloatVal
		case token.CHAR:
			m.EnvTable[int(n.Level)].Variables[n.Identifier.Name].CharVal = n.Expression.CharVal
		}
	default:
		FoundError(NewError(ParseError, fmt.Sprintf("invalid statement startswith %s", p.curToken.Type)))
		p.nextToken()
		os.Exit(1)
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
		FoundError(NewError(ParseError, fmt.Sprintf("identifier expected, but got %s", p.curToken.Literal)))
		os.Exit(1)
	}
	n.Name = p.curToken.Literal
	n.Type = ND_IDENT
	n.Level = scopeLevel
	p.expect(token.COLON)
	p.nextToken()
	if !isTypename(p.curToken) {
		FoundError(NewError(ParseError, fmt.Sprintf("expected type declaration, but got %s", p.curToken.Literal)))
		os.Exit(1)
	}
	n.ElementType = &Element{Type: p.curToken.Type, Stacksize: stackTable[p.curToken.Literal]}
	m.EnvTable[int(n.Level)].Variables[n.Name] = n
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
		lop = NewNode(NodeType(t.Type), lop, p.mul(), scopeLevel)
	}
	return lop
}

func (p *Parser) expr() *Node {
	lop := p.add()
	for {
		t := p.curToken
		if t.Type != token.LT && t.Type != token.GT && t.Type != token.LTEQ && t.Type != token.GTEQ {
			break
		}
		p.nextToken()
		lop = NewNode(NodeType(t.Type), lop, p.add(), scopeLevel)
	}
	return lop
}

func (p *Parser) function() *Function {
	scopeLevel = 1
	fn := &Function{}
	if p.consume(token.FUNCTION) {
		if p.curToken.Type != token.IDENT {
			FoundError(NewError(ParseError, fmt.Sprintf("identifier expected, but got %s", p.curToken.Type)))
			os.Exit(1)
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

func (p *Parser) Parse(manager *Manager) *RootNode {
	manager.EnvTable[1] = newEnv(1)
	manager.EnvTable[0] = newEnv(0)
	m = manager
	functions := make(map[string]*Function)
	rn := &RootNode{Functions: functions}
	fn := p.function()
	rn.Functions[fn.Name] = fn
	return rn
}
