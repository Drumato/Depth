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
	RegMaps   map[int]interface{}
	Variables map[string]*Node
}

type IRType string
type IR struct {
	Type               IRType
	Loperand, Roperand int64
	Level              uint8
	Registerable       bool
	True               int8
}

type Manager struct {
	FuncTable map[*Function][]*IR
	Filename  string
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
