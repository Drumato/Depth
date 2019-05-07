package parse

import (
	"github.com/sirupsen/logrus"
	"github.com/urfave/cli"
)

const (
	IR_IMM      = "Immediate"
	IR_ADD      = "+"
	IR_SUB      = "-"
	IR_MUL      = "*"
	IR_DIV      = "/"
	IR_MOV      = "Move"
	IR_RETURN   = "Return"
	IR_FREE     = "Free"
	IR_PROLOGUE = "Prologue"
	IR_EPILOGUE = "Epilogue"
	IR_NOP      = "Do nothing"
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
	nReg--
}

func stmt(n *Node) {
	switch n.Type {
	case ND_RETURN:
		retReg := expr(n.Expression)
		newIR(IR_RETURN, retReg, 0)
	default:
		logrus.Errorf("unexpected node:%+v", n)
	}

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
	case ND_MUL, ND_DIV:
		lop := expr(n.Loperand)
		rop := expr(n.Roperand)
		newIR(IRType(n.Type), lop, rop)
		kill(rop)
		return lop
	}
	return -42
}

func GenerateIR(rootNode *RootNode, c *cli.Context) *Manager {
	ft := make(map[*Function][]*IR)
	manager := &Manager{FuncTable: ft}
	for _, fn := range rootNode.Functions {
		irs = []*IR{}
		//newIR(IR_PROLOGUE, 0, 0)
		for _, node := range fn.Nodes {
			stmt(node)
		}
		//newIR(IR_EPILOGUE, 0, 0)
		manager.FuncTable[fn] = irs
	}
	return manager
}
