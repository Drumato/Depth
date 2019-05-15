package parse

import (
	"fmt"
	"strconv"

	"github.com/sirupsen/logrus"
)

const (
	ND_INTEGER = "INTEGER"
	ND_FLOAT   = "FLOAT"
	ND_CHAR    = "CHAR"
	ND_PLUS    = "+"
	ND_MINUS   = "-"
	ND_MUL     = "*"
	ND_DIV     = "/"
	ND_GT      = ">"
	ND_LT      = "<"
	ND_GTEQ    = ">="
	ND_LTEQ    = "<="
	ND_RETURN  = "RETURN"
	ND_IF      = "IF"
	ND_DEFINE  = "DEFINE"
	ND_IDENT   = "IDENTIFIER"
)

var (
	stackTable = map[string]int64{
		"i8":   8,
		"i16":  16,
		"i32":  32,
		"i64":  64,
		"i128": 128,
		"ch":   32,
	}
)

func NewNode(ntype NodeType, lop, rop *Node, scope uint8) *Node {
	return &Node{Type: ntype, Loperand: lop, Roperand: rop, Level: scope}
}

func NewNodeNum(val int64, scope uint8) *Node {
	return &Node{IntVal: val, Type: ND_INTEGER, Level: scope}
}
func NewNodeFloat(val float64, scope uint8) *Node {
	return &Node{FloatVal: val, Type: ND_FLOAT, Level: scope}
}

func NewNodeChar(ch string, scope uint8) *Node {
	code, err := strconv.ParseUint(fmt.Sprintf("%d", ch[0]), 10, 32)
	if err != nil {
		logrus.Errorf("%+v", err)
	}
	return &Node{CharVal: uint32(code), Type: ND_CHAR, Level: scope}
}
