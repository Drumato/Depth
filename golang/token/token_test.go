package token

import "testing"

func TestLookupIdent(t *testing.T) {
	tests := []struct {
		input    string
		expected TokenType
	}{
		{"mut", MUTABLE},
		{"true", TRUE},
		{"false", FALSE},
		{"bool", BOOL},
		{"let", LET},
		{"const", CONST},
		{"ch", CHAR},
		{"str", STRING},
		{"f32", F32},
		{"f64", F64},
		{"for", FOR},
		{"loop", LOOP},
		{"if", IF},
		{"i8", I8},
		{"i16", I16},
		{"i32", I32},
		{"i64", I64},
		{"u8", U8},
		{"u16", U16},
		{"u32", U32},
		{"u64", U64},
		{"return", RETURN},
		{"x", IDENT},
	}

	for _, tok := range tests {
		if LookupIdent(tok.input) != tok.expected {
			t.Fatalf("expected %s but got %s", tok.expected, LookupIdent(tok.input))
		}
	}
}
