package codegen

import (
	"depth/parse"
	"fmt"
	"os"

	"github.com/sirupsen/logrus"
)

var ()

func genx86(irs []*parse.IR, f *os.File) {
	for _, ir := range irs {
		switch ir.Type {
		case parse.IR_PROLOGUE:
			fmt.Fprintf(f, "    push rbp #prologue\n")
			fmt.Fprintf(f, "    mov rbp, rsp\n")
		case parse.IR_IMM:
			fmt.Fprintf(f, "    mov %s, %#x #loadimm\n", Registers64[ir.Loperand], ir.Roperand)
		case parse.IR_MOV:
			fmt.Fprintf(f, "    mov %s, %s #loadreg\n", Registers64[ir.Loperand], Registers64[ir.Roperand])
		case parse.IR_RETURN:
			fmt.Fprintf(f, "    mov rax, %s #return\n", Registers64[ir.Loperand])
		case parse.IR_ADD:
			if optLevel == 2 {
				fmt.Fprintf(f, "    add %s, %#x #add\n", Registers64[ir.Loperand], ir.Roperand)
			} else {
				fmt.Fprintf(f, "    add %s, %s #add\n", Registers64[ir.Loperand], Registers64[ir.Roperand])
			}
		case parse.IR_SUB:
			if optLevel == 2 {
				fmt.Fprintf(f, "    sub %s, %#x #sub\n", Registers64[ir.Loperand], ir.Roperand)
			} else {
				fmt.Fprintf(f, "    sub %s, %s #sub\n", Registers64[ir.Loperand], Registers64[ir.Roperand])
			}
		case parse.IR_MUL:
			if optLevel == 2 {
				fmt.Fprintf(f, "    mov rax, %#x #mul\n", ir.Roperand)
				fmt.Fprintf(f, "    mul %s\n", Registers64[ir.Loperand])
				fmt.Fprintf(f, "    mov %s, rax\n", Registers64[ir.Loperand])
			} else {
				fmt.Fprintf(f, "    mov rax, %s #mul\n", Registers64[ir.Roperand])
				fmt.Fprintf(f, "    mul %s\n", Registers64[ir.Loperand])
				fmt.Fprintf(f, "    mov %s, rax\n", Registers64[ir.Loperand])
			}
		case parse.IR_DIV:
			fmt.Fprintf(f, "    mov rax, %s #div\n", Registers64[ir.Loperand])
			fmt.Fprintf(f, "    cqo\n")
			fmt.Fprintf(f, "    div %s\n", Registers64[ir.Roperand])
			fmt.Fprintf(f, "    mov %s, rax\n", Registers64[ir.Loperand])
		case parse.IR_ALLOCATE:
			fmt.Fprintf(f, "    sub rsp, %#x #allocate\n", ir.Roperand)
		case parse.IR_STORE:
			fmt.Fprintf(f, "    mov QWORD PTR -%d[rbp], %#x #store\n", ir.Loperand, ir.Roperand)
		case parse.IR_LOAD:
			fmt.Fprintf(f, "    mov %s, QWORD PTR -%d[rbp] #load\n", Registers64[ir.Loperand], ir.Roperand)
		case parse.IR_EPILOGUE:
			fmt.Fprintf(f, "    mov rsp, rbp\n")
			fmt.Fprintf(f, "    pop rbp#load\n")
			fmt.Fprintf(f, "    ret\n")
		case parse.IR_NOP:
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
