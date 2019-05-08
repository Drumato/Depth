package parse

import (
	"github.com/urfave/cli"
)

var (
	variables map[string]*Node = make(map[string]*Node)
)

func doWalk(n *Node) {
	switch n.Type {
	}

}

func Walk(rootNode *RootNode, c *cli.Context) {
	for _, fn := range rootNode.Functions {
		for _, n := range fn.Nodes {
			doWalk(n)
		}
	}
}
