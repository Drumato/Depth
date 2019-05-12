package parse

import (
	"depth/golang/lex"
	"depth/golang/token"
)

type Parser struct { //recursive-descent parser
	l *lex.Lexer

	curToken token.Token
	nexToken token.Token
}

type Environment struct {
	Level     int
	RegMaps   map[int]*Node
	Variables map[string]*Node
}

type IRType string
type IR struct {
	Type               IRType
	Loperand, Roperand int64
	Level              uint8
	Registerable       bool
}

type Manager struct {
	FuncTable map[*Function][]*IR
}

type ErrorType string

type WarningType uint16

type Error struct {
	Type    ErrorType
	Message string
}

type NodeType string
type Node struct {
	Loperand    *Node
	Roperand    *Node
	Expression  *Node
	IntVal      int64
	FloatVal    float64
	CharVal     uint32
	Name        string
	Level       uint8
	Type        NodeType
	Identifier  *Node
	ElementType *Element
	Init        *Node
	Condition   *Node
	Body        []*Node
	Alternative []*Node
	Mutable     bool
}

func (n *Node) Val() interface{} {
	switch n.Type {
	case ND_INTEGER:
		return n.IntVal
	case ND_CHAR:
		return n.CharVal
	case ND_FLOAT:
		return n.FloatVal
	case ND_IDENT:
		switch n.ElementType.Type {
		case token.I8, token.I16, token.I32, token.I64:
			return n.IntVal
		case token.F32, token.F64:
			return n.FloatVal
		case token.CHAR:
			return n.CharVal
		}
	}
	return n.IntVal
}

type Element struct {
	Type      token.TokenType
	Stacksize int64
}

type RootNode struct {
	Functions map[string]*Function
}

type Function struct {
	Name string
	//IRs []*IR
	Nodes []*Node //may be remove in future
}
