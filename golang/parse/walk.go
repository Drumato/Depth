package parse

import (
	"depth/golang/token"
	"fmt"
	"os"

	"github.com/urfave/cli"
)

func newEnv(lev int) *Environment {
	return &Environment{Level: lev, RegMaps: make(map[int]interface{}), Variables: make(map[string]*Node)}
}

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
		m.EnvTable[int(n.Level)].Variables[n.Identifier.Name].Level = n.Level
		doWalk(n.Identifier)
		if _, ok := m.EnvTable[int(n.Level)].Variables[n.Identifier.Name]; ok {
			switch m.EnvTable[int(n.Level)].Variables[n.Identifier.Name].ElementType.Type {
			case token.I8:
				m.EnvTable[int(n.Level)].Variables[n.Identifier.Name].IntVal = n.Expression.IntVal
			case token.CHAR:
				m.EnvTable[int(n.Level)].Variables[n.Identifier.Name].CharVal = n.Expression.CharVal
			case token.F32:
				m.EnvTable[int(n.Level)].Variables[n.Identifier.Name].FloatVal = n.Expression.FloatVal
			}
		}
	case ND_IDENT:
		i := int(n.Level)
		for {
			if i < 1 {
				FoundError(NewError(InvalidReferenceError, fmt.Sprintf("cannot find '%s' in this scope", n.Name)))
				os.Exit(1)
			}
			if _, ok := m.EnvTable[i].Variables[n.Name]; ok {
				break
			}
			i--
		}
		if _, ok := m.EnvTable[int(n.Level)].Variables[n.Name]; !ok {
			m.EnvTable[int(n.Level)].Variables[n.Name] = n
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
