package parse

import (
	"depth/token"
	"fmt"
	"strconv"

	"github.com/sirupsen/logrus"
)

const (
	ND_INTEGER = "INTEGER"
	ND_CHAR    = "CHAR"
	ND_PLUS    = "+"
	ND_MINUS   = "-"
	ND_MUL     = "*"
	ND_DIV     = "/"
	ND_GT      = ">"
	ND_LT      = "<"
	ND_RETURN  = "RETURN"
	ND_IF      = "IF"
	ND_DEFINE  = "DEFINE"
	ND_IDENT   = "IDENTIFIER"
)

var (
	stackTable = map[string]int64{
		"i8": 8,
		"ch": 32,
	}
)

type NodeType string
type Node struct {
	Loperand    *Node
	Roperand    *Node
	Expression  *Node
	IntVal      int64
	FloatVal    float64
	CharVal     uint32
	Name        string
	Type        NodeType
	Identifier  *Node
	ElementType *Element
	Init        *Node
	Condition   *Node
	Body        []*Node
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

func NewNodeChar(ch string) *Node {
	code, err := strconv.ParseUint(fmt.Sprintf("%d", ch[0]), 10, 32)
	if err != nil {
		logrus.Error("%+v\n", err)
	}
	return &Node{CharVal: uint32(code), Type: ND_CHAR}
}
