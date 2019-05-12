package codegen

import (
	"depth/golang/parse"

	"github.com/urfave/cli"
)

var optLevel int

func Analysis(manager *parse.Manager, c *cli.Context) {
	optLevel = c.Int("optLevel")
}
