package asm

import (
	"fmt"
	"os"
	"strconv"

	"github.com/sirupsen/logrus"
)

const (
	ASCIICode = 0x0a

	AAA = 0x37
	AAD = 0xd5
	AAM = 0xd4
	AAS = 0x3f

	ADCAImm8    = 0x14
	ADCAImm16   = 0x15
	ADCRMImm8   = 0x80
	ADCRMImm8_2 = 0x83
	ADCRMImm16  = 0x81
	ADCRM8      = 0x10
	ADCRM16     = 0x11
	ADCR8       = 0x12
	ADCR16      = 0x13
	ADDRM8      = 0x00
	ADDRM       = 0x01
	ADDR8       = 0x02
	ADDR16      = 0x03
	ADDAL       = 0x04
	ADDAX       = 0x05
	ADDRMImm8   = 0x80
	ADDRMImm    = 0x81

	Leave      = 0xc9
	MovRM32    = 0x89
	MovRImm8   = 0xb0
	MovRImm16  = 0xb8
	MovRImm32  = 0xb8
	MovRMImm8  = 0xc6
	MovRMImm16 = 0xc7
	MovRMImm32 = 0xc7
	PushR32    = 0x50
	PopR32     = 0x58

	SubRM8     = 0x28
	SubRM      = 0x29
	SUBAL      = 0x2c
	SUBAX      = 0x2d
	SubRMImm8  = 0x80
	SubRMImm   = 0x81
	SubRMImm82 = 0x83
	RetNear    = 0xc3
)

type Immediate uint

const (
	Imm8 Immediate = iota
	Imm16
	Imm32
)

type RegType uint

const (
	Reg8 RegType = iota
	Reg16
	Reg32
	Reg64
	Reg_I
)

func Semantic(f *os.File, cds []*Mnemonic) /*[]*Mnemonic*/ {
	for _, c := range cds {
		switch c.Op.Name {
		case aaa_x86:
			/*ASCII Adjust After Addition*/
			c.Op.Code = AAA
		case aad_x86:
			/* ASCII Adjust AX Before Division*/
			var e error
			c.Op.Code, e = strconv.ParseUint(fmt.Sprintf("%x%x", AAD, ASCIICode), 16, 32)
			errOut(e)
		case aam_x86:
			/* ASCII Adjust AX After Multiply*/
			var e error
			c.Op.Code, e = strconv.ParseUint(fmt.Sprintf("%x%x", AAM, ASCIICode), 16, 32)
			errOut(e)
		case aas_x86:
			/* ASCII Adjust AL After Subtraction*/
			c.Op.Code = AAS
		case adc_x86:
			/*Add with Carry*/
			c = semanticAdc(c)
		case add_x86:
			/*Accumulate*/
			c = semanticAdd(c)
		case leave_x86:
			c.Op.Code = Leave
			continue
		case mov_x86:
			c = semanticMov(c)
		case pop_x86:
			switch judgeRegister(c.LOperand.Reg) {
			case Reg64:
				reg64num := register64[c.LOperand.Reg]
				i2, e := strconv.ParseInt(reg64num, 2, 0)
				errOut(e)
				c.Op.Code, e = strconv.ParseUint(fmt.Sprintf("%x", PopR32+i2), 16, 64)
				errOut(e)
				continue
			}
		case push_x86:
			switch judgeRegister(c.LOperand.Reg) {
			case Reg64:
				modrm := &ModRM{R: register64[c.LOperand.Reg]}
				i2, e := strconv.ParseInt(modrm.String(), 2, 0)
				errOut(e)
				c.Op.Code, e = strconv.ParseUint(fmt.Sprintf("%x", PushR32+i2), 16, 64)
				errOut(e)
				continue
			}
		case ret_x86:
			c.Op.Code = RetNear
			continue
		case sub_x86:
			c = semanticSub(c)
		}
	}
}

func judgeImmediate(x int) Immediate {
	if x > -127 && x < 128 {
		return Imm8
	}
	if x > -32768 && x < 32767 {
		return Imm16
	}
	if x > -2147483648 && x < 2147483647 {
		return Imm32
	}
	logrus.Errorf("can't understand:%x", x)
	return Immediate(42)
}

func judgeRegister(reg string) RegType {
	if _, ok := register64[reg]; ok {
		return Reg64
	}
	if _, ok := register32[reg]; ok {
		return Reg32
	}
	if _, ok := register16[reg]; ok {
		return Reg16
	}
	if _, ok := register8[reg]; ok {
		return Reg8
	}
	return Reg_I
}

func judgeRegister64(reg string) bool {
	if _, ok := register642[reg]; ok {
		return true
	}
	return false
}

func errOut(e error) {
	if e != nil {
		logrus.Errorf("%+v\n", e)
	}
}
