package parse

import (
	"os"

	"github.com/sirupsen/logrus"
)

var (
	Registers   = []string{"rbp", "r10", "r11", "rbx", "r12", "r13", "r14", "r15"}
	Registers8  = []string{"bpl", "r10b", "r11b", "bl", "r12b", "r13b", "r14b", "r15b"}
	Registers32 = []string{"ebp", "r10d", "r11d", "ebx", "r12d", "r13d", "r14d", "r15d"}
	regState    = []bool{true, false, false, false, false, false, false, false}
	registerMap []int64
)

func allocate(regNum int64) int64 {
	if registerMap[regNum] != -1 {
		if r := registerMap[regNum]; regState[r] {
			return r
		}
	}

	for i := range Registers {

		if regState[i] {
			continue
		}
		registerMap[regNum] = int64(i)
		regState[i] = true
		return int64(i)
	}
	logrus.Errorf("register exhausted")
	os.Exit(1)
	return -42
}

func free(r int64) {
	if regState[r] {
		regState[r] = false
	}
}

func checkRegister(irs []*IR) {
	registerMap[0] = 0
	for _, ir := range irs {
		switch ir.Type {

		case IR_IMM:
			ir.Loperand = allocate(ir.Loperand)
		case IR_ADD, IR_SUB, IR_MUL, IR_DIV:
			ir.Loperand = allocate(ir.Loperand)
			ir.Roperand = allocate(ir.Roperand)
		case IR_RETURN:
			break
			/*
				if ir.Type == IR_FREE {
					free(ir.Loperand)
					ir.Type = IR_NOP
				}
			*/
		default:
		}
	}
}

func AllocateRegisters(manager *Manager) {
	for _, irs := range manager.FuncTable {
		registerMap = make([]int64, len(irs))
		initialize(irs)
		checkRegister(irs)
	}
}

func initialize(irs []*IR) {
	for i := range irs {
		registerMap[int64(i)] = -1
	}
}
