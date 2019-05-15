package parse

import (
	util "depth/golang/pkg"
	"fmt"
	"os"

	"github.com/sirupsen/logrus"
	"github.com/urfave/cli"
)

func semantics(irs []*IR) {
	for i := range irs {
		switch irs[i].Type {
		case IR_LT, IR_GT, IR_LTEQ, IR_GTEQ: //<
		case IR_IF:
		}
	}
}
func Semantic(manager *Manager, c *cli.Context) {
	for fn, irs := range manager.FuncTable {
		if c.Bool("verbosity") {
			fmt.Println(util.ColorString(fmt.Sprintf("optimize %s's ir...", fn.Name), "blue"))
		}
		semantics(irs)
	}
}

func compare(i int) bool {
	lop, ok := m.EnvTable[int(irs[i].Level)].RegMaps[int(irs[i].Loperand)].(int64)
	rop, ok2 := m.EnvTable[int(irs[i].Level)].RegMaps[int(irs[i].Roperand)].(int64)
	if !ok && !ok2 {
		logrus.Errorf("not mapped register")
		os.Exit(1)
	}
	switch irs[i].Type {
	case IR_LT:
		if ok && ok2 {
			return lop < rop
		}
		logrus.Errorf("Invalid values:%d-%d", lop, rop)
	case IR_GT:
		if ok && ok2 {
			return lop > rop
		}
		logrus.Errorf("Invalid values:%d-%d", lop, rop)
	case IR_LTEQ:
		if ok && ok2 {
			return lop <= rop
		}
		logrus.Errorf("Invalid values:%d-%d", lop, rop)
	case IR_GTEQ:
		if ok && ok2 {
			return lop >= rop
		}
		logrus.Errorf("Invalid values:%d-%d", lop, rop)
	}
	logrus.Errorf("Invalid IR:%s", irs[i].Type)
	os.Exit(1)
	return false
}

func accumulate(i int) int64 {
	lop, ok := m.EnvTable[int(irs[i].Level)].RegMaps[int(irs[i].Loperand)].(int64)
	rop, ok2 := m.EnvTable[int(irs[i].Level)].RegMaps[int(irs[i].Roperand)].(int64)
	if !ok && !ok2 {
		logrus.Errorf("not mapped register")
		os.Exit(1)
	}

	switch irs[i].Type {
	case IR_ADD:
		if ok && ok2 {
			return lop + rop
		}
		logrus.Errorf("Invalid values:%d-%d", lop, rop)
	case IR_SUB:
		if ok && ok2 {
			return lop - rop
		}
		logrus.Errorf("Invalid values:%d-%d", lop, rop)
	case IR_MUL:
		if ok && ok2 {
			return lop * rop
		}
		logrus.Errorf("Invalid values:%d-%d", lop, rop)
	case IR_DIV:
		if ok && ok2 {
			return lop / rop
		}
		logrus.Errorf("Invalid values:%d-%d", lop, rop)
	}
	logrus.Errorf("Invalid IR:%s", irs[i].Type)
	os.Exit(1)
	return -9223372036854775808

}
