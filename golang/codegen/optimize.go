package codegen

import (
	"depth/golang/parse"
	util "depth/golang/pkg"
	"fmt"

	"github.com/urfave/cli"
)

func optimize(irs []*parse.IR) {
	for i := range irs {
		switch irs[i].Type {
		case parse.IR_ADD, parse.IR_SUB:
			switch optLevel {
			case 2:
				reg := irs[i].Roperand
				foundidx := i - 1
				for {
					if irs[foundidx].Type == parse.IR_IMM && irs[foundidx].Loperand == reg {
						irs[i].Roperand = irs[foundidx].Roperand
						irs[foundidx].Type = parse.IR_NOP
						break
					}
					foundidx--
				}
			}
		case parse.IR_STORE:
			reg := irs[i].Roperand
			foundidx := i - 1
			for {
				if irs[foundidx].Type == parse.IR_IMM && irs[foundidx].Loperand == reg {
					irs[i].Roperand = irs[foundidx].Roperand
					irs[foundidx].Type = parse.IR_NOP
					break
				}
				foundidx--
			}
		case parse.IR_FREE:
			if irs[i].Loperand != 1 {
				irs[i].Type = parse.IR_NOP
			}
		case parse.IR_ALLOCATE:
			for foundidx := i - 1; foundidx > 0; foundidx-- {
				if irs[foundidx].Type == parse.IR_ALLOCATE {
					irs[foundidx].Roperand += irs[i].Roperand
					irs[i].Type = parse.IR_NOP
					break
				}
			}
		default:
			continue
		}
	}
}

func Optimize(manager *parse.Manager, c *cli.Context) {
	optLevel = c.Int("optlevel")
	for fn, irs := range manager.FuncTable {
		if c.Bool("verbosity") {
			fmt.Println(util.ColorString(fmt.Sprintf("optimize %s's ir...", fn.Name), "blue"))
		}
		optimize(irs)
	}
}
