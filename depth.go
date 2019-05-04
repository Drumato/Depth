package main

import (
	"depth/lex"
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
	VERSION   = `1.0.0`
	NAME      = `Gocc`
	USAGE     = `A Compiler refered to rui314/9cc-Language`
	AUTHOR    = `Drumato`
	LINK      = `https://github.com/Drumato/gocc`
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
	/*app.Flags = []cli.Flag{
		cli.BoolFlag{Name: "dump,d", Usage: "debugging ir"},
	}
	*/
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
	if _, err := os.Open(os.Args[1]); err != nil {
		input = string([]rune(os.Args[1]))
	} else {
		if filepath.Ext(os.Args[1]) != ".dep" {
			return fmt.Errorf("%v\n", aurora.Bold(aurora.Red("Depth only supporting .dep file at the moment!")))
		}
		f, err := os.Open(os.Args[1])
		if err != nil {
			return err
		}
		b, err := ioutil.ReadAll(f)
		if err != nil {
			return err
		}
		input = string(b)
	}
	lexer := lex.New(input, "")
	tok := lexer.NextToken()
	fmt.Println(aurora.Bold(aurora.Blue("now compiling...")))
	for tok.Type != token.EOF {
		fmt.Printf("%+v\n", tok)
		tok = lexer.NextToken()
	}
	return nil
}
