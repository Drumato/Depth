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

func genx86(n *parse.Node, f *os.File) string {
	if n.Type == parse.ND_INTEGER {
		if cur == len(Registers64) {
			logrus.Errorf("Register exhausted")
		}
		reg := Registers64[cur]
		cur++
		fmt.Fprintf(f, "    mov %s, %d\n", reg, n.IntVal)
		return reg
	}
	dst := genx86(n.Lhs, f)
	src := genx86(n.Rhs, f)
	switch n.Type {
	case parse.ND_PLUS:
		fmt.Fprintf(f, "    add %s, %s\n", dst, src)
		return dst
	case parse.ND_MINUS:
		fmt.Fprintf(f, "    sub %s, %s\n", dst, src)
		return dst
	default:
		logrus.Errorf("Unknown operaotr")
	}
	return ""
}

func Gen(rootNode *parse.RootNode, f *os.File, filename string) {
	fmt.Fprintf(f, "    .file \"%s\"\n", filename)
	fmt.Fprintf(f, "    .intel_syntax noprefix\n")
	fmt.Fprintf(f, "    .text\n")
	fmt.Fprintf(f, "    .globl main\n")
	fmt.Fprintf(f, "    .type main, @function\n")
	fmt.Fprintf(f, "main:\n")
	lastreg := genx86(rootNode.Functions["main"].Nodes[0], f)
	fmt.Fprintf(f, "    mov rax, %s\n", lastreg)
	fmt.Fprintf(f, "    ret\n")
}
