package parse

import (
	util "depth/pkg"
	"depth/token"
	"fmt"
	"os"

	"github.com/urfave/cli"
)

var (
	variables map[string]*Node = make(map[string]*Node)
)

func doWalk(n *Node) {
	switch n.Type {
	case ND_IF:
		doWalk(n.Condition)
		scopeLevel++
		for _, st := range n.Body {
			doWalk(st)
		}
		scopeLevel--
	case ND_PLUS, ND_MINUS, ND_MUL, ND_DIV, ND_GT, ND_LT:
		doWalk(n.Loperand)
		doWalk(n.Roperand)
	case ND_DEFINE:
		doWalk(n.Identifier)
		if _, ok := variables[n.Identifier.Name]; ok {
			switch variables[n.Identifier.Name].ElementType.Type {
			case token.I8:
				variables[n.Identifier.Name].IntVal = n.Expression.IntVal
			case token.CHAR:
				variables[n.Identifier.Name].CharVal = n.Expression.CharVal
			}
		}
	case ND_IDENT:
		if _, ok := variables[n.Name]; !ok {
			variables[n.Name] = n
			return
		}
		fmt.Printf("%s:%d-%d", n.Name, scopeLevel, n.Level)
		if scopeLevel < n.Level {
			fmt.Printf(util.ColorString(fmt.Sprintf("%s: %s\n", InvalidReferenceError, n.Name), "red"))
			os.Exit(1)
		}
	}

}

func Walk(rootNode *RootNode, c *cli.Context) {
	for _, fn := range rootNode.Functions {
		for _, n := range fn.Nodes {
			doWalk(n)
		}
	}
}
