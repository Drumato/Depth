package parse

const (
	ParseError ErrorType = iota
)

const (
	W_Security WarningType = iota
)

type ErrorType uint16
type WarningType uint16

type Error struct {
	Type ErrorType
}
type Warning struct {
	Type WarningType
}
