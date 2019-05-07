package main

import (
	"depth/codegen"
	"depth/lex"
	"depth/parse"
	util "depth/pkg"
	"depth/token"
	"fmt"
	"io/ioutil"
	"os"
	"os/exec"
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
	if _, err := os.Open("asm/target/debug/asm"); err != nil {
		fmt.Printf(util.ColorString("Builds assembler...", "blue"))
		cmd := exec.Command("make", "-c", "asm/")
		err := cmd.Run()
		if err != nil {
			logrus.Errorf(ErrFormat, err)
		}
	}
	if err := app.Run(os.Args); err != nil {
		fmt.Printf(ErrFormat, util.ColorString(fmt.Sprintf("%+v", err), "red"))
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
		cli.IntFlag{Name: "dump-ir", Usage: "dump ir", Value: 0},
		cli.BoolFlag{Name: "print-stdout", Usage: "print stdout the result of the processing"},
		cli.BoolFlag{Name: "until-compile", Usage: "stop processing when succeed compile"},
		cli.BoolFlag{Name: "until-assemble", Usage: "stop processing when succeed assemble"},
		cli.BoolFlag{Name: "verbosity", Usage: "output verbosity flag"},
		cli.StringFlag{Name: "arch", Usage: "specify architecture", Value: "x86-64"},
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
	semantic(c, manager)
	analysis(c, manager)
	optimize(c, manager)
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
	manager := parse.GenerateIR(rootNode, c)
	parse.AllocateRegisters(manager)
	if c.Bool("verbosity") {
		fmt.Println(util.ColorString("Tramslates intermediate representation...", "blue"))
	}
	if c.Int("dump-ir") == 1 {
		fmt.Println(util.ColorString("----------------dump IRs Stage1----------------", "blue"))
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
		logrus.Errorf("%+v", err)
		os.Exit(1)
	}
	if filename == "" {
		if c.Bool("print-stdout") {
			fmt.Printf("%s\n", aurora.Bold(aurora.Blue("----------------assembly----------------")))
			codegen.Gen(manager, os.Stdout, filename)
			return
		} else {
			codegen.Gen(manager, f, "sample.dep")
			return
		}
	} else {

		if c.Bool("print-stdout") {
			fmt.Printf("%s\n", aurora.Bold(aurora.Blue("----------------assembly----------------")))
			codegen.Gen(manager, os.Stdout, filename)
			return
		} else {
			codegen.Gen(manager, f, filename)
			return
		}
	}
}
func semantic(c *cli.Context, manager *parse.Manager) {
	parse.Semantic(manager, c)
}

func analysis(c *cli.Context, manager *parse.Manager) {
	codegen.Analysis(manager, c)
}

func optimize(c *cli.Context, manager *parse.Manager) {
	codegen.Optimize(manager, c)
	if c.Int("dump-ir") == 2 {
		fmt.Println(util.ColorString("----------------dump IRs Stage 2----------------", "blue"))
		for fn, irs := range manager.FuncTable {
			fmt.Printf("%s IR:\n", util.ColorString(fn.Name, "green"))
			for _, ir := range irs {
				fmt.Printf("%+v\n", ir)
			}
		}
	}
}

func generateBinary(c *cli.Context) {
	if c.Bool("verbosity") {
		fmt.Println(util.ColorString("Assemble mnemonics...", "blue"))
	}
	cmd := exec.Command("asm/target/debug/asm")
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	err := cmd.Run()
	if err != nil {
		logrus.Errorf(ErrFormat, err)
	}

}
