package lex

import (
	"depth/golang/token"
	"testing"
)

func TestNumber(t *testing.T) {
	input := `
3+5
3++
3-5
3--
!3
3/5
3*5
3%5
3 += 5
3 -= 5
3 *= 5
3 /= 5
3 %= 5
3 < 5
3 > 5
3 >= 5
3 <= 5
3 == 5
3 != 5	
0x77 + 0xff
0777 + 0222`
	tests := []struct {
		expectedType    token.TokenType
		expectedLiteral string
	}{
		{token.INTLIT, "3"},
		{token.PLUS, "+"},
		{token.INTLIT, "5"},

		{token.INTLIT, "3"},
		{token.INCRE, "++"},

		{token.INTLIT, "3"},
		{token.MINUS, "-"},
		{token.INTLIT, "5"},

		{token.INTLIT, "3"},
		{token.DECRE, "--"},

		{token.BANG, "!"},
		{token.INTLIT, "3"},

		{token.INTLIT, "3"},
		{token.SLASH, "/"},
		{token.INTLIT, "5"},

		{token.INTLIT, "3"},
		{token.ASTERISK, "*"},
		{token.INTLIT, "5"},

		{token.INTLIT, "3"},
		{token.PERCENT, "%"},
		{token.INTLIT, "5"},

		{token.INTLIT, "3"},
		{token.PLUSASSIGN, "+="},
		{token.INTLIT, "5"},

		{token.INTLIT, "3"},
		{token.MINUSASSIGN, "-="},
		{token.INTLIT, "5"},

		{token.INTLIT, "3"},
		{token.ASTERISKASSIGN, "*="},
		{token.INTLIT, "5"},

		{token.INTLIT, "3"},
		{token.SLASHASSIGN, "/="},
		{token.INTLIT, "5"},

		{token.INTLIT, "3"},
		{token.PERCENTASSIGN, "%="},
		{token.INTLIT, "5"},

		{token.INTLIT, "3"},
		{token.LT, "<"},
		{token.INTLIT, "5"},

		{token.INTLIT, "3"},
		{token.GT, ">"},
		{token.INTLIT, "5"},

		{token.INTLIT, "3"},
		{token.GTEQ, ">="},
		{token.INTLIT, "5"},

		{token.INTLIT, "3"},
		{token.LTEQ, "<="},
		{token.INTLIT, "5"},

		{token.INTLIT, "3"},
		{token.EQ, "=="},
		{token.INTLIT, "5"},

		{token.INTLIT, "3"},
		{token.NOT_EQ, "!="},
		{token.INTLIT, "5"},
		{token.INTLIT, "77"},
		{token.PLUS, "+"},
		{token.INTLIT, "ff"},
		{token.INTLIT, "0777"},
		{token.PLUS, "+"},
		{token.INTLIT, "0222"},
	}

	l := New(input, "")
	New(input, "aaa")
	for i, tt := range tests {
		tok := l.NextToken()

		if tok.Type != tt.expectedType {
			t.Fatalf("tests[%d] - tokentype wrong, expected=%q,got=%q", i, tt.expectedType, tok.Type)
		}

		if tok.Literal != tt.expectedLiteral {
			t.Fatalf("tests[%d] - literal wrong. expected=%q,got=%q", i, tt.expectedLiteral, tok.Literal)
		}
	}
}

func TestString(t *testing.T) {
	input := `"string" 'a'`
	tests := []struct {
		expectedType    token.TokenType
		expectedLiteral string
	}{
		{token.STRINGLIT, "string"},
		{token.CHARLIT, "a"},
	}

	l := New(input, "")
	New(input, "aaa")
	for i, tt := range tests {
		tok := l.NextToken()

		if tok.Type != tt.expectedType {
			t.Fatalf("tests[%d] - tokentype wrong, expected=%q,got=%q", i, tt.expectedType, tok.Type)
		}

		if tok.Literal != tt.expectedLiteral {
			t.Fatalf("tests[%d] - literal wrong. expected=%q,got=%q", i, tt.expectedLiteral, tok.Literal)
		}
	}
}

func TestNull(t *testing.T) {
	input := ``
	tests := []struct {
		expectedType    token.TokenType
		expectedLiteral string
	}{
		{token.EOF, ``},
	}

	l := New(input, "")
	New(input, "aaa")
	for i, tt := range tests {
		tok := l.NextToken()

		if tok.Type != tt.expectedType {
			t.Fatalf("tests[%d] - tokentype wrong, expected=%q,got=%q", i, tt.expectedType, tok.Type)
		}

		if tok.Literal != tt.expectedLiteral {
			t.Fatalf("tests[%d] - literal wrong. expected=%q,got=%q", i, tt.expectedLiteral, tok.Literal)
		}
	}
}

func TestFunction(t *testing.T) {
	input := `f add(x : i8, y : i8) -> i8 { return x + y }`
	tests := []struct {
		expectedType    token.TokenType
		expectedLiteral string
	}{
		{token.FUNCTION, "f"},
		{token.IDENT, "add"},
		{token.LPAREN, "("},
		{token.IDENT, "x"},
		{token.COLON, ":"},
		{token.I8, "i8"},
		{token.COMMA, ","},
		{token.IDENT, "y"},
		{token.COLON, ":"},
		{token.I8, "i8"},
		{token.RPAREN, ")"},
		{token.ARROW, "->"},
		{token.I8, "i8"},
		{token.LBRACE, "{"},
		{token.RETURN, "return"},
		{token.IDENT, "x"},
		{token.PLUS, "+"},
		{token.IDENT, "y"},
		{token.RBRACE, "}"},
	}

	l := New(input, "")
	New(input, "aaa")
	for i, tt := range tests {
		tok := l.NextToken()

		if tok.Type != tt.expectedType {
			t.Fatalf("tests[%d] - tokentype wrong, expected=%q,got=%q", i, tt.expectedType, tok.Type)
		}

		if tok.Literal != tt.expectedLiteral {
			t.Fatalf("tests[%d] - literal wrong. expected=%q,got=%q", i, tt.expectedLiteral, tok.Literal)
		}
	}
}
