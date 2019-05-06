package main

import (
	"depth/asm"
	"depth/codegen"
	"depth/lex"
	"depth/parse"
	util "depth/pkg"
	"depth/token"
	"fmt"
	"io/ioutil"
	"os"
	"path/filepath"

	"github.com/logrusorgru/aurora"
	"github.com/sirupsen/logrus"
	"github.com/urfave/cli"
)

const (
	ErrFormat = "Error found: %+v\n"
	VERSION   = `0.1.1`
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
		fmt.Printf(ErrFormat, util.ColorString(fmt.Sprintf("%+v\n", err), "red"))
	}
}

func init() {
	app.Version = VERSION
	app.Name = NAME
	app.Usage = USAGE
	app.Author = AUTHOR
	app.Email = LINK
	app.Flags = []cli.Flag{
		cli.BoolFlag{Name: "dump-source", Usage: "show code like UNIX cat command"},
		cli.BoolFlag{Name: "dump-tokens", Usage: "dump tokens by lexer"},
		cli.BoolFlag{Name: "dump-ast", Usage: "dump ast by recursive-descent parser"},
		cli.BoolFlag{Name: "dump-hex", Usage: "dump binary by hex"},
		cli.BoolFlag{Name: "dump-ir", Usage: "dump ir"},
		cli.BoolFlag{Name: "print-stdout", Usage: "print stdout the result of the processing"},
		cli.BoolFlag{Name: "until-compile", Usage: "stop processing when succeed compile"},
		cli.BoolFlag{Name: "until-assemble", Usage: "stop processing when succeed assemble"},
		cli.BoolFlag{Name: "verbosity", Usage: "output verbosity flag"},
		cli.IntFlag{Name: "optlevel", Usage: "specify optimization levels", Value: 0},
	}
	app.Action = func(c *cli.Context) error {
		if len(os.Args) < 2 {
			return fmt.Errorf(ErrFormat, util.ColorString("not given code", "red"))
		}
		if err := Start(c); err != nil {
			return fmt.Errorf(ErrFormat, util.ColorString(fmt.Sprintf("%+v", err), "red"))
		}
		return nil
	}
}

func Start(c *cli.Context) error {
	sourcecode := os.Args[len(os.Args)-1]
	lexer := lexing(c, sourcecode)
	rootNode := builtAST(c, lexer)
	manager := translateIRs(c, rootNode)
	generateCode(c, manager, lexer.Filename)
	if c.Bool("until-compile") {
		return nil
	}
	generateBinary(c)
	if c.Bool("until-assemble") {
		return nil
	}
	return nil
}

func lexing(c *cli.Context, sourcecode string) *lex.Lexer {
	var input string
	var lexer *lex.Lexer
	if c.Bool("verbosity") {
		fmt.Println(util.ColorString("Tokenize...", "blue"))
	}
	if f, err := os.Open(sourcecode); err != nil {
		input = string([]rune(sourcecode))
		lexer = lex.New(input, "")
	} else {
		if filepath.Ext(sourcecode) != ".dep" {
			logrus.Errorf(ErrFormat, util.ColorString("Depth only supporting .dep ext at the moment!", "red"))
			os.Exit(1)
		}
		b, err := ioutil.ReadAll(f)
		if err != nil {
			logrus.Errorf(ErrFormat, err)
			os.Exit(1)
		}
		if c.Bool("dump-source") {
			fmt.Println(util.ColorString("----------------input source----------------", "blue"))
			fmt.Println(string(b))
		}
		input = string(b)
		lexer = lex.New(input, sourcecode)
	}
	if c.Bool("dump-tokens") {
		fmt.Println(util.ColorString("----------------dump tokens----------------", "blue"))
		tok := lexer.NextToken()
		for tok.Type != token.EOF {
			fmt.Printf("%+v\n", tok)
			tok = lexer.NextToken()
		}
		lexer = lex.New(input, sourcecode)
	}
	return lexer
}

func builtAST(c *cli.Context, lexer *lex.Lexer) *parse.RootNode {
	if c.Bool("verbosity") {
		fmt.Println(util.ColorString("Builds ast...", "blue"))
	}
	parser := parse.New(lexer)
	rootNode := parser.Parse()
	if c.Bool("dump-ast") {
		fmt.Printf("%+v\n", rootNode)
	}
	return rootNode
}

func translateIRs(c *cli.Context, rootNode *parse.RootNode) *parse.Manager {
	manager := parse.GenerateIR(rootNode, c.Int("optlevel"))
	parse.AllocateRegisters(manager)
	if c.Bool("verbosity") {
		fmt.Println(util.ColorString("Tramslates intermediate representation...", "blue"))
	}
	if c.Bool("dump-ir") {
		fmt.Println(util.ColorString("----------------dump IRs----------------", "blue"))
		for fn, irs := range manager.FuncTable {
			fmt.Printf("%s IR:\n", util.ColorString(fn.Name, "green"))
			for _, ir := range irs {
				fmt.Printf("%+v\n", ir)
			}
		}
	}
	return manager
}

func generateCode(c *cli.Context, manager *parse.Manager, filename string) {
	if c.Bool("verbosity") {
		fmt.Println(util.ColorString("Compile source...", "blue"))
	}
	f, err := os.Create("tmp.s")
	if err != nil {
		logrus.Errorf("%+v\n", err)
		os.Exit(1)
	}
	if filename == "" {
		codegen.Gen(manager, f, "sample.dep", c.Int("optlevel"))
	} else {

		if c.Bool("print-stdout") {
			fmt.Printf("%s\n", aurora.Bold(aurora.Blue("----------------assembly----------------")))
			codegen.Gen(manager, os.Stdout, filename, c.Int("optlevel"))
		} else {
			codegen.Gen(manager, f, filename, c.Int("optlevel"))
		}
	}
}

func generateBinary(c *cli.Context) {
	if c.Bool("verbosity") {
		fmt.Println(util.ColorString("Assemble mnemonics...", "blue"))
	}
	asmf, err := os.Open("tmp.s")
	if err != nil {
		logrus.Errorf("%+v\n", err)
		os.Exit(1)
	}
	binaries, err := ioutil.ReadAll(asmf)
	if err != nil {
		logrus.Errorf("%+v\n", err)
		os.Exit(1)
	}
	asms := asm.Parse(string(binaries))
	asm.Semantic(os.Stdout, asms)
	if c.Bool("dump-hex") {
		fmt.Println(util.ColorString("----------------hexdump----------------", "blue"))
		for _, as := range asms {
			if as.Op.Code == 0 {
				fmt.Printf("%s: % x\n", as.Op.Name, as.Op.Code)
				continue
			}
			fmt.Printf("% x\n", as.Op.Code)
		}
	}
}
