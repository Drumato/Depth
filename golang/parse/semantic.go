package parse

import (
	util "depth/golang/pkg"
	"fmt"

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
	lop, ok := envTable[int(irs[i].Level)].RegMaps[int(irs[i].Loperand)].(int64)
	rop, ok2 := envTable[int(irs[i].Level)].RegMaps[int(irs[i].Roperand)].(int64)
	if !ok && !ok2 {
		logrus.Errorf("not mapped register")
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
	return false
}
