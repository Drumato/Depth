package main

import (
	"depth/codegen"
	"depth/lex"
	"depth/parse"
	"depth/token"
	"fmt"
	"io/ioutil"
	"os"
	"path/filepath"

	"github.com/logrusorgru/aurora"
	"github.com/urfave/cli"
)

const (
	ErrFormat = "Error found: %s\n"
	VERSION   = `0.1.0`
	NAME      = `Depth`
	USAGE     = `./depth [options] [-flags] <sourcefile>or<source>`
	AUTHOR    = `Drumato`
	LINK      = `https://github.com/Drumato/Depth`
)

var (
	app = cli.NewApp()
)

func main() {
	if err := app.Run(os.Args); err != nil {
		fmt.Printf(ErrFormat, aurora.Bold(aurora.Red(fmt.Sprintf("%+v", err))))
	}
}

func init() {
	app.Version = VERSION
	app.Name = NAME
	app.Usage = USAGE
	app.Author = AUTHOR
	app.Email = LINK
	app.Flags = []cli.Flag{
		cli.BoolFlag{Name: "dump-tokens", Usage: "dump tokens by lexer"},
		cli.BoolFlag{Name: "dump-ast", Usage: "dump ast by recursive-descent parser"},
	}
	app.Action = func(c *cli.Context) error {
		if len(os.Args) < 2 {
			return fmt.Errorf("%v\n", aurora.Bold(aurora.Red("not given code")))
		}
		if err := Start(c); err != nil {
			fmt.Printf(ErrFormat, aurora.Bold(aurora.Red(fmt.Sprintf("%+v", err))))
		}
		return nil
	}
}

func Start(c *cli.Context) error {
	var input string
	var lexer *lex.Lexer
	if _, err := os.Open(os.Args[len(os.Args)-1]); err != nil {
		input = string([]rune(os.Args[len(os.Args)-1]))
		lexer = lex.New(input, "")
	} else {
		if filepath.Ext(os.Args[len(os.Args)-1]) != ".dep" {
			return fmt.Errorf("%v\n", aurora.Bold(aurora.Red("Depth only supporting .dep file at the moment!")))
		}
		f, err := os.Open(os.Args[len(os.Args)-1])
		if err != nil {
			return err
		}
		b, err := ioutil.ReadAll(f)
		if err != nil {
			return err
		}
		input = string(b)
		lexer = lex.New(input, os.Args[len(os.Args)-1])
	}
	if c.Bool("dump-tokens") {
		tok := lexer.NextToken()
		for tok.Type != token.EOF {
			fmt.Printf("%+v\n", tok)
			tok = lexer.NextToken()
		}
	}
	parser := parse.New(lexer)
	rootNode := parser.Parse()
	if c.Bool("dump-ast") {
		fmt.Printf("%+v\n", rootNode)
	}
	//fmt.Println(aurora.Bold(aurora.Blue("now compiling...")))
	if lexer.Filename == "" {
		codegen.Gen(rootNode, os.Stdout, "sample.dep")
	} else {
		f, err := os.Create("tmp.s")
		if err != nil {
			return err
		}
		codegen.Gen(rootNode, f, lexer.Filename)
	}
	return nil
}
