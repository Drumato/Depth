package parse

import (
	"github.com/urfave/cli"
)

func doWalk(n *Node) {
	switch n.Type {
	case ND_DEFINE:
		doWalk(n.Identifier)
		if _, ok := variables[n.Identifier.Name]; ok {
			variables[n.Identifier.Name].IntVal = n.Expression.IntVal
		}
	case ND_IDENT:
		if _, ok := variables[n.Name]; !ok {
			variables[n.Name] = n
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
