package lex

import (
	"depth/golang/token"
	"strconv"
	"strings"

	"github.com/sirupsen/logrus"
)

const (
	HEX uint = iota
	OCTAL
	DECIMAL
)

//Lexer does tokenize specify source
type Lexer struct {
	input                  string
	position, readPosition int
	ch                     rune
	Line                   int
	Col                    int
	Filename               string
}

//New Lexer
func New(input string, filepath string) *Lexer {
	if filepath != "" {
		l := &Lexer{input: input, Filename: filepath}
		l.readChar()
		return l
	}
	l := &Lexer{input: input}
	l.readChar()
	return l
}

func (l *Lexer) readNumber() string {
	position := l.position
	for isDigit(l.ch) {
		l.readChar()
	}
	if l.ch == '.' || l.ch == 'x' {
		l.readChar()
		for isDigit(l.ch) {
			l.readChar()
		}
	}
	return l.input[position:l.position]
}

func (l *Lexer) readChar() {
	if l.readPosition >= len(l.input) {
		l.ch = 0
	} else {
		l.ch = rune(l.input[l.readPosition])
	}
	l.position = l.readPosition
	l.readPosition++
	l.Col++
}

func (l *Lexer) peekChar() rune {
	if l.readPosition >= len(l.input) {
		return 0
	}
	return rune(l.input[l.readPosition])
}

//NextToken start to process next token
func (l *Lexer) NextToken() token.Token {
	var tok token.Token
	l.skipWhitespace()
	switch l.ch {
	case '=':
		if l.peekChar() == '=' {
			ch := l.ch
			l.readChar()
			literal := string(ch) + string(l.ch)
			tok = token.Token{Type: token.EQ, Literal: literal}
		} else {
			tok = newToken(token.ASSIGN, l.ch)
		}
	case '+':
		if l.peekChar() == '+' {
			ch := l.ch
			l.readChar()
			literal := string(ch) + string(l.ch)
			tok = token.Token{Type: token.INCRE, Literal: literal}
		} else {
			if l.peekChar() == '=' {
				ch := l.ch
				l.readChar()
				literal := string(ch) + string(l.ch)
				tok = token.Token{Type: token.PLUSASSIGN, Literal: literal}
			} else {
				tok = newToken(token.PLUS, l.ch)
			}
		}
	case '-':
		if l.peekChar() == '-' {
			ch := l.ch
			l.readChar()
			literal := string(ch) + string(l.ch)
			tok = token.Token{Type: token.DECRE, Literal: literal}
		} else {
			if l.peekChar() == '=' {
				ch := l.ch
				l.readChar()
				literal := string(ch) + string(l.ch)
				tok = token.Token{Type: token.MINUSASSIGN, Literal: literal}
			} else {
				tok = newToken(token.MINUS, l.ch)
			}
		}
	case '!':
		if l.peekChar() == '=' {
			ch := l.ch
			l.readChar()
			literal := string(ch) + string(l.ch)
			tok = token.Token{Type: token.NOT_EQ, Literal: literal}
		} else {
			tok = newToken(token.BANG, l.ch)
		}
	case '/':
		if l.peekChar() == '/' {
			literal := l.readComment()
			tok = token.Token{Type: token.COMMENT, Literal: literal}
		} else {
			if l.peekChar() == '=' {
				ch := l.ch
				l.readChar()
				literal := string(ch) + string(l.ch)
				tok = token.Token{Type: token.SLASHASSIGN, Literal: literal}
			} else {
				tok = newToken(token.SLASH, l.ch)
			}
		}
	case '*':
		if l.peekChar() == '=' {
			ch := l.ch
			l.readChar()
			literal := string(ch) + string(l.ch)
			tok = token.Token{Type: token.ASTERISKASSIGN, Literal: literal}
		} else {
			tok = newToken(token.ASTERISK, l.ch)
		}
	case '<':
		if l.peekChar() == '=' {
			ch := l.ch
			l.readChar()
			literal := string(ch) + string(l.ch)
			tok = token.Token{Type: token.LTEQ, Literal: literal}
		} else {
			tok = newToken(token.LT, l.ch)
		}
	case '>':
		if l.peekChar() == '=' {
			ch := l.ch
			l.readChar()
			literal := string(ch) + string(l.ch)
			tok = token.Token{Type: token.GTEQ, Literal: literal}
		} else {
			tok = newToken(token.GT, l.ch)
		}
	case ',':
		tok = newToken(token.COMMA, l.ch)
	case '(':
		tok = newToken(token.LPAREN, l.ch)
	case ')':
		tok = newToken(token.RPAREN, l.ch)
	case '{':
		tok = newToken(token.LBRACE, l.ch)
	case '}':
		tok = newToken(token.RBRACE, l.ch)
	case '[':
		tok = newToken(token.LBRACKET, l.ch)
	case ']':
		tok = newToken(token.RBRACKET, l.ch)
	case '%':
		if l.peekChar() == '=' {
			ch := l.ch
			l.readChar()
			literal := string(ch) + string(l.ch)
			tok = token.Token{Type: token.PERCENTASSIGN, Literal: literal}
		} else {
			tok = newToken(token.PERCENT, l.ch)
		}
	case ':':
		tok = newToken(token.COLON, l.ch)
	case '|':
		if l.peekChar() == '|' {
			ch := l.ch
			l.readChar()
			literal := string(ch) + string(l.ch)
			tok = token.Token{Type: token.LOGOR, Literal: literal}
		} else {
			tok = newToken(token.PIPE, l.ch)
		}
	case '&':
		if l.peekChar() == '&' {
			ch := l.ch
			l.readChar()
			literal := string(ch) + string(l.ch)
			tok = token.Token{Type: token.LOGAND, Literal: literal}
		} else {
			tok = newToken(token.AMPERSAND, l.ch)
		}
	case ';':
		tok = newToken(token.SEMICOLON, l.ch)
	case '\'':
		tok.Type = token.CHARLIT
		l.readChar()
		tok.Literal = string(l.ch)
		l.readChar()
	case '"':
		tok.Type = token.STRINGLIT
		tok.Literal = l.readString()
	case '.':
		tok = newToken(token.DOT, l.ch)
	case 0:
		tok.Literal = ""
		tok.Type = token.EOF
	default:
		if isLetter(l.ch) {
			tok.Literal = l.readIdentifier()
			tok.Type = token.LookupIdent(tok.Literal)
			return tok
		} else if isDigit(l.ch) {
			tok.Literal = l.readNumber()
			var base int
			if strings.Contains(tok.Literal, ".") {
				tok.Type = token.FLOATLIT
				tok.FloatVal, _ = strconv.ParseFloat(tok.Literal, 64)
			} else {
				tok.Type = token.INTLIT
				switch judgeBase(tok.Literal) {
				case HEX:
					tok.Literal = string(tok.Literal[2:])
					base = 16
				case OCTAL:
					base = 8
				case DECIMAL:
					base = 10
				default:
					logrus.Errorf("Invalid Number:%s", tok.Literal)
				}
				tok.IntVal, _ = strconv.ParseInt(tok.Literal, base, 64)
			}
			return tok
		} else {
			tok = newToken(token.ILLEGAL, l.ch)
		}
	}
	l.readChar()
	return tok
}

func newToken(tokenType token.TokenType, ch rune) token.Token {
	return token.Token{Type: tokenType, Literal: string(ch)}
}

func (l *Lexer) readIdentifier() string {
	position := l.position
	for isValid(l.ch) {
		l.readChar()
	}
	return string([]rune(l.input[position:l.position]))
}

func (l *Lexer) readString() string {
	position := l.position + 1
	for {
		l.readChar()
		if l.ch == '"' || l.ch == 0 {
			break
		}
	}
	return string([]rune(l.input[position:l.position]))
}

func isLetter(ch rune) bool {
	return 'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
}
func isValid(ch rune) bool {
	return 'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_' || '0' <= ch && ch <= '9'
}

func isDigit(ch rune) bool {
	return '0' <= ch && ch <= '9'
}

func (l *Lexer) skipWhitespace() {
	for l.ch == ' ' || l.ch == '\t' || l.ch == '\n' || l.ch == '\r' {
		if l.ch == '\n' {
			l.Line++
			l.Col = 0
		}
		l.readChar()
	}
}

func (l *Lexer) readComment() string {
	position := l.position + 1
	for {
		l.readChar()
		if l.ch == '\n' || l.ch == 0 {
			break
		}
	}
	return string([]rune(l.input[position:l.position]))
}

func judgeBase(num string) uint {
	if strings.HasPrefix(num, "0x") {
		return HEX
	}
	if strings.HasPrefix(num, "0") {
		return OCTAL
	}
	return DECIMAL
}
