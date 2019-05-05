package asm

import (
	"fmt"
	"strconv"
)

func semanticAdc(c *Mnemonic) *Mnemonic {
	switch judgeRegister(c.LOperand.Reg) {
	case Reg8:
		if c.LOperand.Reg == "al" {
			if c.ROperand.Val != 0 {
				var e error
				c.Op.Code, e = strconv.ParseUint(fmt.Sprintf("%x%x", ADCAImm8, c.ROperand.Val), 16, 8)
				errOut(e)
			}
			modrm := &ModRM{Mod: "11", R: register8[c.LOperand.Reg], M: register8[c.ROperand.Reg]}
			i2, err := strconv.ParseInt(modrm.String(), 2, 0)
			errOut(err)
			c.Op.Code, err = strconv.ParseUint(fmt.Sprintf("%x%x", ADCRM8, i2), 16, 8)
			errOut(err)
		}
	case Reg32:
		if c.ROperand.Val != 0 {
			//buf := new(bytes.Buffer)
			switch judgeImmediate(c.ROperand.Val) {
			case Imm8:
				if c.LOperand.Reg == "al" {
				} else {

				}
				/*
					case Imm16:
						err := binary.Write(buf, binary.LittleEndian, uint16(c.ROperand.Val))
						errOut(err)
						fmt.Sprintf( "%x % x 00 00\n", MovRImm32+i2, buf.Bytes())
					case Imm32:
						err := binary.Write(buf, binary.LittleEndian, uint32(c.ROperand.Val))
						errOut(err)
						fmt.Sprintf( "%x % x\n", MovRImm32+i2, buf.Bytes())
				*/
			}
		}
	}
	return c
}
