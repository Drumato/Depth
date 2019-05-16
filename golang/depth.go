package main

import (
	"bytes"
	"depth/golang/asm"
	"depth/golang/codegen"
	"depth/golang/lex"
	"depth/golang/parse"
	util "depth/golang/pkg"
	"depth/golang/token"
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
	manager := &parse.Manager{}
	manager.EnvTable = make(map[int]*parse.Environment)
	manager.Lexer = lexing(c, sourcecode)
	builtAST(c, manager)
	walkAST(manager.Root, c)
	translateIRs(c, manager)
	semantic(c, manager)
	analysis(c, manager)
	optimize(c, manager)
	generateCode(c, manager, manager.Lexer.Filename)
	generateBinary(c, manager)
	if c.Bool("until-compile") {
		return nil
	}
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
		input = string(b)
		lexer = lex.New(input, sourcecode)
	}
	if c.Bool("dump-source") {
		fmt.Println(util.ColorString("----------------input source----------------", "blue"))
		fmt.Println(input)
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

func builtAST(c *cli.Context, manager *parse.Manager) {
	if c.Bool("verbosity") {
		fmt.Println(util.ColorString("Builds ast...", "blue"))
	}
	parser := parse.New(manager.Lexer)
	manager.Root = parser.Parse(manager)
	if c.Bool("dump-ast") {
		fmt.Printf("%+v\n", manager.Root)
	}
}

func walkAST(rootNode *parse.RootNode, c *cli.Context) {
	if c.Bool("verbosity") {
		fmt.Println(util.ColorString("Walking ast...", "blue"))
	}
	parse.Walk(rootNode, c)
}

func translateIRs(c *cli.Context, manager *parse.Manager) {
	manager.FuncTable = parse.GenerateIR(manager.Root, c)
	parse.AllocateRegisters(manager.FuncTable)
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
		filename = "sample.dep"
	}
	if c.Bool("print-stdout") {
		fmt.Printf("%s\n", aurora.Bold(aurora.Blue("----------------assembly----------------")))
		codegen.Gen(manager, os.Stdout, filename)
		return
	} else {
		codegen.Gen(manager, f, filename)
		return
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

func generateBinary(c *cli.Context, manager *parse.Manager) {
	if c.Bool("verbosity") {
		fmt.Println(util.ColorString("Generate binary...", "blue"))
	}
	var bmanager *util.ByteManager = util.NewBytes("sample.o", "w")
	var bins [][]byte
	var syms []byte
	/* text */
	texts := []byte{0x55, 0x48, 0x89, 0xe5, 0x8b, 0x14, 0x00, 0x00, 0x00, 0x5d, 0xc3}
	bins = append(bins, texts)

	/* for using symnames */
	syms = append(syms, '\x00')
	for i := range manager.Lexer.Filename {
		syms = append(syms, manager.Lexer.Filename[i])
	}
	for name := range manager.EnvTable[1].Variables {
		syms = append(syms, '\x00')
		for i := range name {
			syms = append(syms, name[i])
		}
	}
	syms = append(syms, '\x00')

	/* symtab*/
	var symtab []*asm.Elf64_Sym
	symtab = append(symtab, &asm.Elf64_Sym{0, 0, 0, 0, 0, 0}) //NULL Symbols
	symtab = append(symtab, &asm.Elf64_Sym{                   //filename
		Name:  uint32(bytes.Index(syms, []byte(manager.Lexer.Filename))),
		Info:  asm.NewInfo(asm.STB_LOCAL, asm.STT_FILE),
		Other: asm.STV_DEFAULT,
		Shndx: asm.SHN_ABS,
		Value: 0, // byte offset at the start
		Size:  0,
	})

	for _, idx := range [...]uint16{1, 2, 3, 5, 6, 4} {
		symtab = append(symtab, &asm.Elf64_Sym{
			Name:  '\x00',
			Info:  asm.NewInfo(asm.STB_LOCAL, asm.STT_SECTION),
			Other: asm.STV_DEFAULT,
			Shndx: idx, // index indicates the position of .data section
			Value: 0,   // byte offset at the start of .data section
			Size:  0,   //bits -> bytes
		}) //sections
	}

	var addr uint64
	for name, n := range manager.EnvTable[1].Variables {
		idx := bytes.Index(syms, []byte(name))
		if idx == -1 {
			logrus.Errorf("Invalid Symbols: %s is not defined", name)
			os.Exit(1)
		}
		symtab = append(symtab, &asm.Elf64_Sym{
			Name:  uint32(idx),
			Info:  asm.NewInfo(asm.STB_GLOBAL, asm.STT_OBJECT),
			Other: asm.STV_DEFAULT,
			Shndx: 2,                                   // index indicates the position of .data section
			Value: addr,                                // byte offset at the start of .data section
			Size:  uint64(n.ElementType.Stacksize / 8), //bits -> bytes
		}) //variables
		addr += uint64(n.ElementType.Stacksize) / uint64(8)
	}

	symtab = append(symtab, &asm.Elf64_Sym{ //main functions
		Name:  uint32(bytes.Index(syms, []byte("main"))),
		Info:  asm.NewInfo(asm.STB_GLOBAL, asm.STT_FUNC),
		Other: asm.STV_DEFAULT,
		Shndx: 1, // index indicates the position of .text section
		Value: 0, // byte offset at the start of .text section
		Size:  uint64(len(texts)),
	})

	/* strtab */
	bins = append(bins, syms)

	/* shstrtab */
	bins = append(bins, []byte("\x00.text\x00.data\x00.symtab\x00.strtab\x00.shstrtab\x00"))
	elf := asm.GenObject(bins, nil)
	bmanager.Input = elf.Dump()
	if err := bmanager.Flush(); err != nil {
		logrus.Errorf("%+v\n", err)
	}
}
