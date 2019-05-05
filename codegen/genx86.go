package codegen

import (
	"depth/parse"
	"fmt"
	"os"

	"github.com/sirupsen/logrus"
)

var (
	cur int
)

func genx86(irs []*parse.IR, f *os.File) {
	for _, ir := range irs {
		switch ir.Type {
		case parse.IR_PROLOGUE:
			fmt.Fprintf(f, "    push rbp\n")
			fmt.Fprintf(f, "    mov rbp, rsp\n")
		case parse.IR_IMM:
			fmt.Fprintf(f, "    mov %s, %d\n", Registers64[ir.Loperand], ir.Roperand)
		case parse.IR_MOV:
			fmt.Fprintf(f, "    mov %s, %s\n", Registers64[ir.Loperand], Registers64[ir.Roperand])
		case parse.IR_RETURN:
			fmt.Fprintf(f, "    mov rax, %s\n", Registers64[ir.Loperand])
			fmt.Fprintf(f, "    ret\n")
		case parse.IR_ADD:
			fmt.Fprintf(f, "    add %s, %s\n", Registers64[ir.Loperand], Registers64[ir.Roperand])
		case parse.IR_SUB:
			fmt.Fprintf(f, "    sub %s, %s\n", Registers64[ir.Loperand], Registers64[ir.Roperand])
		case parse.IR_FREE:
		case parse.IR_EPILOGUE:
			break

		default:
			logrus.Errorf("Unknown Operator:%s", ir.Type)
		}
	}
}

func Gen(manager *parse.Manager, f *os.File, filename string) {
	fmt.Fprintf(f, "    .file \"%s\"\n", filename)
	fmt.Fprintf(f, "    .intel_syntax noprefix\n")
	fmt.Fprintf(f, "    .text\n")
	fmt.Fprintf(f, "    .globl main\n")
	fmt.Fprintf(f, "    .type main, @function\n")
	var subRoutine []*parse.Function
	for fn, irs := range manager.FuncTable {
		if fn.Name == "main" {
			fmt.Fprintf(f, "main:\n")
			genx86(irs, f)
		} else {
			subRoutine = append(subRoutine, fn)
		}
	}
	for _, fn := range subRoutine {
		genx86(manager.FuncTable[fn], f)
	}
}
