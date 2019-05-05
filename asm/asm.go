package asm

import "unsafe"

const (
	aaa_x86   = "aaa"
	aad_x86   = "aad"
	aam_x86   = "aam"
	aas_x86   = "aas"
	adc_x86   = "adc"
	leave_x86 = "leave"
	mov_x86   = "mov"
	push_x86  = "push"
	pop_x86   = "pop"
	ret_x86   = "ret"
)

type Mnemonic struct {
	Op       *Opecode
	LOperand *Operand
	ROperand *Operand
}

type Opetype string

type Opecode struct {
	Name Opetype
	Code uint64
}

type Operand struct {
	Reg string
	Val int
}

type ModRM struct {
	Mod, R, M string
}

func (modrm ModRM) String() string {
	return modrm.Mod + modrm.M + modrm.R
}

type REXPrefix struct {
	L, W, R, X, B string
}

func (rex REXPrefix) String() string {
	return rex.L + rex.W + rex.R + rex.X + rex.B
}

type AddressType struct {
	Registers                                      []string
	Address, Immediate, Offset, Base, Displacement string
}

func strToByte(s string) []byte {
	return *(*[]byte)(unsafe.Pointer(&s))
}
