package parse

const (
	ND_INTEGER = "INTEGER"
	ND_PLUS    = "+"
	ND_MINUS   = "-"
	ND_MUL     = "*"
	ND_DIV     = "/"
)

type NodeType string
type Node struct {
	Loperand *Node
	Roperand *Node
	IntVal   int64
	FloatVal float64
	Name     string
	Type     NodeType
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
