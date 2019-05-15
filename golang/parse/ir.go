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
	IR_IF       = "IF"
)

var (
	irs       []*IR
	nReg      int64 = 1
	stackSize int64 = 0
	labelNum  int64 = 2
)

func newIR(ty IRType, lop, rop int64, registerable bool, scope uint8) *IR {
	ir := &IR{Type: ty, Loperand: lop, Roperand: rop, Registerable: registerable, Level: scope}
	irs = append(irs, ir)
	return ir
}

func label() {
	newIR(IR_LABEL, labelNum, 0, false, scopeLevel)
	labelNum++
}

func jump() {
	newIR(IR_JMP, labelNum+1, 0, false, scopeLevel)
}

func stmt(n *Node) *IR {
	switch n.Type {
	case ND_RETURN:
		retReg := expr(n.Expression)
		idx := len(irs) - 1
		rn := newIR(IR_RETURN, retReg, 0, false, n.Level)
		if isCompare(irs[idx].Type) {
			retReg = int64(irs[idx].True)
			irs[idx].Type = IR_NOP
			rn.True = irs[idx].True
		}
		return rn
	case ND_IF:
		i := newIR(IR_IF, 0, 0, false, n.Level)
		expr(n.Condition)
		idx := len(irs) - 1
		for idx > 0 {
			if isCompare(irs[idx].Type) {
				if compare(idx) {
					i.True = 2
				} else {
					i.True = 1
				}
				break
			}
			idx--
		}
		idx = len(irs) - 1
		for irs[idx].Type != IR_IF {
			irs[idx].Type = IR_NOP
			idx--
		}
		scopeLevel++
		nReg--
		if i.True == 2 {
			for _, st := range n.Body {
				stmt(st)
			}
		} else if i.True == 1 {
			for _, st := range n.Alternative {
				stmt(st)
			}
		}
		scopeLevel--
		return i
	case ND_DEFINE:
		newIR(IR_ALLOCATE, 0, n.Identifier.ElementType.Stacksize, false, n.Level)
		retReg := expr(n.Expression)
		n.Identifier.ElementType.Stacksize += stackSize
		stackSize += 8
		nReg--
		return newIR(IR_STORE, n.Identifier.ElementType.Stacksize, retReg, true, n.Level)
	default:
		logrus.Errorf("unexpected node:%+v", n)
	}
	return nil
}

func expr(n *Node) int64 {
	switch n.Type {
	case ND_INTEGER:
		reg := nReg
		m.EnvTable[int(n.Level)].RegMaps[int(reg)] = n.IntVal
		nReg++
		newIR(IR_IMM, reg, n.IntVal, true, n.Level)
		return reg
	case ND_CHAR:
		reg := nReg
		nReg++
		m.EnvTable[int(n.Level)].RegMaps[int(reg)] = n.CharVal
		newIR(IR_IMM, reg, int64(n.CharVal), true, n.Level)
		return reg
	case ND_FLOAT:
		reg := nReg
		nReg++
		m.EnvTable[int(n.Level)].RegMaps[int(reg)] = n.FloatVal
		newIR(IR_IMM, reg, int64(n.FloatVal), true, n.Level)
		return reg
	case ND_PLUS, ND_MINUS:
		lop := expr(n.Loperand)
		rop := expr(n.Roperand)
		newIR(IRType(n.Type), lop, rop, true, n.Level)
		nReg--
		return lop
	case ND_MUL, ND_DIV:
		lop := expr(n.Loperand)
		rop := expr(n.Roperand)
		newIR(IRType(n.Type), lop, rop, true, n.Level)
		nReg--
		return lop
	case ND_LT, ND_GT, ND_LTEQ, ND_GTEQ:
		lop := expr(n.Loperand)
		rop := expr(n.Roperand)
		newIR(IRType(n.Type), lop, rop, true, n.Level)
		if compare(len(irs) - 1) {
			irs[len(irs)-1].True = 2
		} else {
			irs[len(irs)-1].True = 1
		}
		nReg--
		return lop
	case ND_IDENT:
		reg := nReg
		m.EnvTable[int(n.Level)].RegMaps[int(reg)] = n.IntVal
		nReg++
		newIR(IR_LOAD, reg, m.EnvTable[int(n.Level)].Variables[n.Name].ElementType.Stacksize, true, n.Level)
		irs[len(irs)-1].Level = n.Level
		return reg
	}
	return -42
}

func GenerateIR(rootNode *RootNode, c *cli.Context) map[*Function][]*IR {
	ft := make(map[*Function][]*IR)
	for _, fn := range rootNode.Functions {
		scopeLevel = 1
		irs = []*IR{}
		newIR(IR_PROLOGUE, 0, 0, false, scopeLevel)
		for _, node := range fn.Nodes {
			stmt(node)
		}
		newIR(IR_EPILOGUE, 0, 0, false, scopeLevel)
		ft[fn] = irs
	}
	return ft
}

func isCompare(ty IRType) bool {
	if ty == IR_LT || ty == IR_GT || ty == IR_LTEQ || ty == IR_GTEQ {
		return true
	}
	return false
}
