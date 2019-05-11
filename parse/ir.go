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
	IR_LT       = "<"
	IR_GT       = ">"
	IR_LTEQ     = "<="
	IR_GTEQ     = ">="
	IR_CMP      = "COMPARE"
	IR_LABEL    = "LABEL"
	IR_JMP      = "JUMP"
	IR_MOV      = "Move"
	IR_RETURN   = "Return"
	IR_FREE     = "Free"
	IR_ALLOCATE = "Allocate"
	IR_PROLOGUE = "Prologue"
	IR_EPILOGUE = "Epilogue"
	IR_STORE    = "STORE"
	IR_LOAD     = "LOAD"
	IR_NOP      = "Do nothing"
)

var (
	irs       []*IR
	nReg      int64 = 1
	stackSize int64 = 0
	labelNum  int64 = 2
)

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

func newIR(ty IRType, lop, rop int64, registerable bool) *IR {
	ir := &IR{Type: ty, Loperand: lop, Roperand: rop, Registerable: registerable}
	irs = append(irs, ir)
	return ir
}

func label() {
	newIR(IR_LABEL, labelNum, 0, false)
	labelNum++
}

func jump() {
	newIR(IR_JMP, labelNum+1, 0, false)
}

func stmt(n *Node) {
	switch n.Type {
	case ND_RETURN:
		retReg := expr(n.Expression)
		newIR(IR_RETURN, retReg, 0, false)
	case ND_IF:
		expr(n.Condition)
		nReg--
		for _, st := range n.Body {
			stmt(st)
		}
		for _, st := range n.Alternative {
			stmt(st)
		}
	case ND_DEFINE:
		newIR(IR_ALLOCATE, 0, n.Identifier.ElementType.Stacksize, false)
		retReg := expr(n.Expression)
		n.Identifier.ElementType.Stacksize += stackSize
		stackSize += 8
		nReg--
		newIR(IR_STORE, n.Identifier.ElementType.Stacksize, retReg, true)
	default:
		logrus.Errorf("unexpected node:%+v", n)
	}
}

func expr(n *Node) int64 {
	switch n.Type {
	case ND_INTEGER:
		reg := nReg
		nReg++
		newIR(IR_IMM, reg, n.IntVal, true)
		return reg
	case ND_CHAR:
		reg := nReg
		nReg++
		newIR(IR_IMM, reg, int64(n.CharVal), true)
		return reg
	case ND_PLUS, ND_MINUS:
		lop := expr(n.Loperand)
		rop := expr(n.Roperand)
		newIR(IRType(n.Type), lop, rop, true)
		nReg--
		return lop
	case ND_MUL, ND_DIV:
		lop := expr(n.Loperand)
		rop := expr(n.Roperand)
		newIR(IRType(n.Type), lop, rop, true)
		nReg--
		return lop
	case ND_LT, ND_GT, ND_LTEQ, ND_GTEQ:
		lop := expr(n.Loperand)
		rop := expr(n.Roperand)
		newIR(IR_CMP, lop, rop, true)
		newIR(IRType(n.Type), labelNum, 0, false)
		newIR(IR_IMM, lop, 0, true)
		jump()
		label()
		newIR(IR_IMM, lop, 1, true)
		label()
		nReg--
		return lop
	case ND_IDENT:
		reg := nReg
		nReg++
		newIR(IR_LOAD, reg, envTable[int(n.Level)].Variables[n.Name].ElementType.Stacksize, true)
		irs[len(irs)-1].Level = n.Level
		return reg
	}
	return -42
}

func GenerateIR(rootNode *RootNode, c *cli.Context) *Manager {
	ft := make(map[*Function][]*IR)
	manager := &Manager{FuncTable: ft}
	for _, fn := range rootNode.Functions {
		irs = []*IR{}
		newIR(IR_PROLOGUE, 0, 0, false)
		for _, node := range fn.Nodes {
			stmt(node)
		}
		newIR(IR_EPILOGUE, 0, 0, false)
		manager.FuncTable[fn] = irs
	}
	return manager
}
