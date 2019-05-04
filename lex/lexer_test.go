package lex

import (
	"depth/token"
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
3 != 5	`
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
