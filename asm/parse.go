package asm

import (
	"strconv"
	"strings"
	"unicode"

	"github.com/sirupsen/logrus"
)

func Parse(code string) []*Mnemonic {
	var codes []*Mnemonic
	asms := strings.Split(code, "\n")
	for _, as := range asms {

		if strings.Contains(as, "main:") || strings.Contains(as, ".") || as == "" {
			continue
		}
		t := strings.Fields(as)
		var tmp []string
		for _, tt := range t {
			if strings.Contains(tt, "#") {
				continue
			}
			tmp = append(tmp, tt)
		}
		switch len(tmp) {
		case 1:
			codes = append(codes, &Mnemonic{Op: &Opecode{Name: Opetype(tmp[0])}})
		case 2:
			codes = append(codes, &Mnemonic{Op: &Opecode{Name: Opetype(tmp[0])}, LOperand: &Operand{Reg: tmp[1]}})
		case 3:
			tmp[1] = strings.TrimRight(tmp[1], ",")
			digitFlg := true
			for _, r := range tmp[2] {
				if !unicode.IsDigit(r) {
					digitFlg = false
				}
			}
			if digitFlg {
				val, err := strconv.ParseInt(tmp[2], 10, 0)
				if err != nil {
					logrus.Errorf("%+v\n", err)
				}
				codes = append(codes, &Mnemonic{Op: &Opecode{Name: Opetype(tmp[0])}, LOperand: &Operand{Reg: tmp[1]}, ROperand: &Operand{Val: int(val)}})
			} else {
				codes = append(codes, &Mnemonic{Op: &Opecode{Name: Opetype(tmp[0])}, LOperand: &Operand{Reg: tmp[1]}, ROperand: &Operand{Reg: tmp[2]}})
			}
		default:
			logrus.Errorf("Invalid mnemonic:'%s'", as)
		}
	}
	return codes
}
