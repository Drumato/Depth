package asm

import (
	"bytes"
	"encoding/binary"
	"fmt"
	"strconv"
)

func semanticAdd(c *Mnemonic) *Mnemonic {
	modrm := &ModRM{Mod: "11", R: register64[c.LOperand.Reg], M: register64[c.ROperand.Reg]}
	/* rexprefix for 64-bit register*/
	rexpre := &REXPrefix{L: "0100", W: "1", R: "0", X: "0", B: "0"}
	switch judgeRegister(c.LOperand.Reg) {
	case Reg64:
		if judgeRegister64(c.LOperand.Reg) {
			rexpre.B = "1"
		}
		if _, ok := register64[c.ROperand.Reg]; ok {
			if judgeRegister64(c.ROperand.Reg) {
				rexpre.R = "1"
			}
			modrm2, e := strconv.ParseInt(modrm.String(), 2, 0)
			rex, e := strconv.ParseInt(rexpre.String(), 2, 0)
			errOut(e)
			add := fmt.Sprintf("%x", ADDRM)
			if len(add) == 1 {
				add = "0" + add
			}
			c.Op.Code, e = strconv.ParseUint(fmt.Sprintf("%x%s%x", rex, add, modrm2), 16, 64)
			errOut(e)
			return c
		}
		switch judgeImmediate(c.ROperand.Val) {
		case Imm8, Imm16, Imm32:
			if len(modrm.R) == 3 {
				modrm.R = rexpre.R + modrm.R
			}
			modrm.Mod = "1100"
			modrm2, e := strconv.ParseInt(modrm.String(), 2, 0)
			errOut(e)
			rex, e := strconv.ParseInt(rexpre.String(), 2, 0)
			errOut(e)
			buf := new(bytes.Buffer)
			err := binary.Write(buf, binary.LittleEndian, uint8(c.ROperand.Val))
			errOut(err)
			c.Op.Code, e = strconv.ParseUint(fmt.Sprintf("%x%x%x%x", rex, ADDRMImm, modrm2, buf.Bytes()), 16, 64)
			errOut(e)
			return c
		}
	case Reg32:
	default:
		fmt.Println(c.LOperand.Reg)
	}
	return c
}
