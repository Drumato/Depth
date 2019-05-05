package parse

const (
	ND_INTEGER = "INTEGER"
	ND_PLUS    = "+"
	ND_MINUS   = "-"
)

type NodeType string
type Node struct {
	Lhs      *Node
	Rhs      *Node
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
	Nodes []*Node //remove in future
}

func NewNode(ntype NodeType, lhs, rhs *Node) *Node {
	return &Node{Type: ntype, Lhs: lhs, Rhs: rhs}
}

func NewNodeNum(val int64) *Node {
	return &Node{IntVal: val, Type: ND_INTEGER}
}
