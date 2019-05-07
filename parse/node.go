package parse

import "depth/token"

const (
	ND_INTEGER = "INTEGER"
	ND_PLUS    = "+"
	ND_MINUS   = "-"
	ND_MUL     = "*"
	ND_DIV     = "/"
	ND_RETURN  = "RETURN"
	ND_DEFINE  = "DEFINE"
	ND_IDENT   = "IDENTIFIER"
)

var (
	stackTable = map[string]int64{
		"i8": 8,
	}
)

type NodeType string
type Node struct {
	Loperand    *Node
	Roperand    *Node
	Expression  *Node
	IntVal      int64
	FloatVal    float64
	Name        string
	Type        NodeType
	Identifier  *Node
	ElementType *Element
	Init        *Node
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

func NewNode(ntype NodeType, lop, rop *Node) *Node {
	return &Node{Type: ntype, Loperand: lop, Roperand: rop}
}

func NewNodeNum(val int64) *Node {
	return &Node{IntVal: val, Type: ND_INTEGER}
}
