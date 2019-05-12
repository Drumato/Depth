package parse

import (
	util "depth/pkg"
	"fmt"
	"os"
)

const (
	ParseError            = "ParseError"
	InvalidReferenceError = "InvalidReferenceError"
)

const (
	W_Security WarningType = iota
)

func NewError(ety ErrorType, msg string) *Error {
	return &Error{Type: ety, Message: msg}
}
func FoundError(e *Error) {
	fmt.Fprintf(os.Stderr, "%s: %s\n", util.ColorString(string(e.Type), "red"), e.Message)
}

type Warning struct {
	Type WarningType
}
