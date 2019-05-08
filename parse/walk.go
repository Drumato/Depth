package parse

import (
	"depth/token"

	"github.com/urfave/cli"
)

var (
	variables map[string]*Node = make(map[string]*Node)
)

func doWalk(n *Node) {
	switch n.Type {
<<<<<<< HEAD
=======
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
		}

>>>>>>> 4936fc66c69832524798f3aae48be72fa7e62c77
	}

}

func Walk(rootNode *RootNode, c *cli.Context) {
	for _, fn := range rootNode.Functions {
		for _, n := range fn.Nodes {
			doWalk(n)
		}
	}
}
