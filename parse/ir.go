package parse

const (
	IR_IMM      = "Immediate"
	IR_ADD      = "+"
	IR_SUB      = "-"
	IR_MOV      = "Move"
	IR_RETURN   = "Return"
	IR_FREE     = "Free"
	IR_PROLOGUE = "Prologue"
	IR_EPILOGUE = "Epilogue"
)

var (
	irs  []*IR
	nReg int64 = 1
)

type IRType string
type IR struct {
	Type               IRType
	Loperand, Roperand int64
}

type Manager struct {
	FuncTable map[*Function][]*IR
}

func newIR(ty IRType, lop, rop int64) *IR {
	ir := &IR{Type: ty, Loperand: lop, Roperand: rop}
	irs = append(irs, ir)
	return ir
}

func kill(reg int64) {
	newIR(IR_FREE, reg, 0)
}

func expr(n *Node) int64 {
	switch n.Type {
	case ND_INTEGER:
		reg := nReg
		nReg++
		newIR(IR_IMM, reg, n.IntVal)
		return reg
	case ND_PLUS, ND_MINUS:
		lop := expr(n.Loperand)
		rop := expr(n.Roperand)
		newIR(IRType(n.Type), lop, rop)
		kill(rop)
		return lop
	}
	return -42
}

func GenerateIR(rootNode *RootNode) *Manager {
	ft := make(map[*Function][]*IR)
	manager := &Manager{FuncTable: ft}
	var retReg int64
	for _, fn := range rootNode.Functions {
		irs = []*IR{}
		//newIR(IR_PROLOGUE, 0, 0)
		for _, node := range fn.Nodes {
			retReg = expr(node)
		}
		//newIR(IR_EPILOGUE, 0, 0)
		newIR(IR_RETURN, retReg, 0)
		manager.FuncTable[fn] = irs
	}
	return manager
}
