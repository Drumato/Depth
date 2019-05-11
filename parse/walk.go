package parse

import (
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
	case ND_RETURN:
		doWalk(n.Expression)
	case ND_IF:
		doWalk(n.Condition)
		scopeLevel++
		for _, st := range n.Body {
			doWalk(st)
		}
		for _, st := range n.Alternative {
			doWalk(st)
		}
		scopeLevel--
	case ND_PLUS, ND_MINUS, ND_MUL, ND_DIV, ND_GT, ND_LT, ND_LTEQ, ND_GTEQ:
		doWalk(n.Loperand)
		doWalk(n.Roperand)
	case ND_DEFINE:
		variables[n.Identifier.Name].Level = n.Level
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
		if scopeLevel < variables[n.Name].Level {
			FoundError(NewError(InvalidReferenceError, fmt.Sprintf("can not access '%s' by outer", n.Name)))
			os.Exit(1)
		}
		if _, ok := variables[n.Name]; !ok {
			variables[n.Name] = n
			return
		}
	default:
		return
	}
}

func Walk(rootNode *RootNode, c *cli.Context) {
	for _, fn := range rootNode.Functions {
		scopeLevel = 1
		for _, n := range fn.Nodes {
			doWalk(n)
		}
	}
}
