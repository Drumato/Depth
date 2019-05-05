package main

import (
	"depth/asm"
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
		cli.BoolFlag{Name: "dump-source", Usage: "show code like UNIX cat command"},
		cli.BoolFlag{Name: "dump-tokens", Usage: "dump tokens by lexer"},
		cli.BoolFlag{Name: "dump-ast", Usage: "dump ast by recursive-descent parser"},
		cli.BoolFlag{Name: "dump-hex", Usage: "dump binary by hex"},
		cli.BoolFlag{Name: "dump-ir", Usage: "dump ir"},
		cli.BoolFlag{Name: "print-stdout", Usage: "print stdout the result of the processing"},
		cli.BoolFlag{Name: "-until-compile", Usage: "stop processing when succeed compile"},
		cli.BoolFlag{Name: "-until-assemble", Usage: "stop processing when succeed assemble"},
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
	sourcecode := os.Args[len(os.Args)-1]
	if f, err := os.Open(sourcecode); err != nil {
		input = string([]rune(sourcecode))
		lexer = lex.New(input, "")
	} else {
		if filepath.Ext(sourcecode) != ".dep" {
			return fmt.Errorf("%v\n", aurora.Bold(aurora.Red("Depth only supporting .dep file at the moment!")))
		}
		b, err := ioutil.ReadAll(f)
		if err != nil {
			return err
		}
		if c.Bool("dump-source") {
			fmt.Printf("%s\n", aurora.Bold(aurora.Blue("----------------input source----------------")))
			fmt.Println(string(b))
		}
		input = string(b)
		lexer = lex.New(input, sourcecode)
	}
	if c.Bool("dump-tokens") {
		fmt.Printf("%s\n", aurora.Bold(aurora.Blue("----------------dump tokens----------------")))
		tok := lexer.NextToken()
		for tok.Type != token.EOF {
			fmt.Printf("%+v\n", tok)
			tok = lexer.NextToken()
		}
		lexer = lex.New(input, sourcecode)
	}
	parser := parse.New(lexer)
	rootNode := parser.Parse()
	if c.Bool("dump-ast") {
		fmt.Printf("%+v\n", rootNode)
	}
	manager := parse.GenerateIR(rootNode)
	parse.AllocateRegisters(manager)
	//fmt.Println(aurora.Bold(aurora.Blue("now compiling...")))
	if c.Bool("dump-ir") {
		fmt.Printf("%s\n", aurora.Bold(aurora.Blue("----------------dump IRs----------------")))
		for fn, irs := range manager.FuncTable {
			fmt.Printf("%s IR:\n", aurora.Bold(aurora.Blue(fn.Name)))
			for _, ir := range irs {
				fmt.Printf("%+v\n", ir)
			}
		}
	}
	if lexer.Filename == "" {
		codegen.Gen(manager, os.Stdout, "sample.dep")
	} else {
		f, err := os.Create("tmp.s")
		if err != nil {
			return err
		}

		if c.Bool("print-stdout") {
			fmt.Printf("%s\n", aurora.Bold(aurora.Blue("----------------assembly----------------")))
			codegen.Gen(manager, os.Stdout, lexer.Filename)
		} else {
			codegen.Gen(manager, f, lexer.Filename)
		}
	}
	if c.Bool("until-compile") {
		return nil
	}
	asmf, err := os.Open("tmp.s")
	if err != nil {
		return err
	}
	binaries, err := ioutil.ReadAll(asmf)
	if err != nil {
		return err
	}
	asms := asm.Parse(string(binaries))
	asm.Semantic(os.Stdout, asms)
	if c.Bool("dump-hex") {
		fmt.Printf("%s\n", aurora.Bold(aurora.Blue("----------------hexdump----------------")))
		for _, as := range asms {
			if as.Op.Code == 0 {
				fmt.Printf("%s: % x\n", as.Op.Name, as.Op.Code)
				continue
			}
			fmt.Printf("% x\n", as.Op.Code)
		}
	}
	if c.Bool("until-assemble") {
		return nil
	}
	return nil
}
