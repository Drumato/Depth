package asm

import (
	"bytes"
	"encoding/binary"
	"fmt"
	"strconv"
)

func semanticMov(c *Mnemonic) *Mnemonic {
	modrm := &ModRM{Mod: "11", R: register64[c.LOperand.Reg], M: register64[c.ROperand.Reg]}
	/* rexprefix for 64-bit register*/
	rexpre := &REXPrefix{L: "0100", W: "1", R: "0", X: "0", B: "0"}
	switch judgeRegister(c.LOperand.Reg) {
	case Reg64:
		/* case 64-bit register loperand*/
		if judgeRegister64(c.LOperand.Reg) {
			rexpre.B = "1"
		}
		if _, ok := register64[c.ROperand.Reg]; ok {
			/* modr/m with /r suffix*/
			if judgeRegister64(c.ROperand.Reg) {
				rexpre.R = "1"
			}
			modrm2, e := strconv.ParseInt(modrm.String(), 2, 0)
			rex, e := strconv.ParseInt(rexpre.String(), 2, 0)
			errOut(e)
			c.Op.Code, e = strconv.ParseUint(fmt.Sprintf("%x%x%x", rex, MovRM32, modrm2), 16, 64)
		}
		/* load immediate to 64-bit register*/
		rex, e := strconv.ParseInt(rexpre.String(), 2, 0)
		errOut(e)
		buf := new(bytes.Buffer)
		err := binary.Write(buf, binary.LittleEndian, uint8(c.ROperand.Val))
		errOut(err)
		switch judgeImmediate(c.ROperand.Val) {
		case Imm8, Imm16, Imm32:
			if len(modrm.R) == 3 {
				modrm.R = rexpre.R + modrm.R
			}
			modrm.Mod = "1100"
			modrm2, e := strconv.ParseInt(modrm.String(), 2, 0)
			errOut(e)
			c.Op.Code, e = strconv.ParseUint(fmt.Sprintf("%x%x%x%x", rex, MovRMImm32, modrm2, buf.Bytes()), 16, 64)
			errOut(e)
		}

	case Reg32:
		if _, ok := register32[c.ROperand.Reg]; ok {
			modrm := &ModRM{Mod: "11", R: register32[c.LOperand.Reg], M: register32[c.ROperand.Reg]}
			modrm2, e := strconv.ParseInt(modrm.String(), 2, 0)
			errOut(e)
			c.Op.Code, e = strconv.ParseUint(fmt.Sprintf("%x%x", MovRM32, modrm2), 16, 32)
			errOut(e)
		}
		modrm := &ModRM{R: register32[c.LOperand.Reg]}
		i2, e := strconv.ParseInt(modrm.String(), 2, 0)
		errOut(e)
		/* load immediate to 32-bit register*/
		buf := new(bytes.Buffer)
		switch judgeImmediate(c.ROperand.Val) {
		case Imm8:
			err := binary.Write(buf, binary.LittleEndian, uint8(c.ROperand.Val))
			errOut(err)
			c.Op.Code, err = strconv.ParseUint(fmt.Sprintf("%x%x000000", MovRImm32+i2, buf.Bytes()), 16, 64)
			errOut(err)
		case Imm16:
			err := binary.Write(buf, binary.LittleEndian, uint16(c.ROperand.Val))
			errOut(err)
			c.Op.Code, err = strconv.ParseUint(fmt.Sprintf("%x%x0000", MovRImm32+i2, buf.Bytes()), 16, 64)
			errOut(err)
		case Imm32:
			err := binary.Write(buf, binary.LittleEndian, uint32(c.ROperand.Val))
			errOut(err)
			c.Op.Code, err = strconv.ParseUint(fmt.Sprintf("%x%x", MovRImm32+i2, buf.Bytes()), 16, 32)
			errOut(err)
		}
	}
	return c
}
