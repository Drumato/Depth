package parse

const (
	ParseError            = "ParseError"
	InvalidReferenceError = "InvalidReferenceError"
)

const (
	W_Security WarningType = iota
)

type ErrorType string
type WarningType uint16

type Error struct {
	Type ErrorType
}
type Warning struct {
	Type WarningType
}
