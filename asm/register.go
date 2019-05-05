package asm

var (
	register64 = map[string]string{
		"rax": "000",
		"rcx": "001",
		"rdx": "010",
		"rbx": "011",
		"rsp": "100",
		"rbp": "101",
		"rsi": "110",
		"rdi": "111",
		"r8":  "000",
		"r9":  "001",
		"r10": "010",
		"r11": "011",
		"r12": "100",
		"r13": "101",
		"r14": "110",
		"r15": "111",
	}
	register642 = map[string]string{
		"r8":  "1000",
		"r9":  "1001",
		"r10": "1010",
		"r11": "1011",
		"r12": "1100",
		"r13": "1101",
		"r14": "1110",
		"r15": "1111",
	}
	register32 = map[string]string{
		"eax": "000",
		"ecx": "001",
		"edx": "010",
		"ebx": "011",
		"esp": "100",
		"ebp": "101",
		"esi": "110",
		"edi": "111",
	}
	register16 = map[string]string{
		"ax": "000",
		"cx": "001",
		"dx": "010",
		"bx": "011",
		"sp": "100",
		"bp": "101",
		"si": "110",
		"di": "111",
	}
	register8 = map[string]string{
		"al": "000",
		"cl": "001",
		"dl": "010",
		"bl": "011",
		"ah": "100",
		"ch": "101",
		"dh": "110",
		"bh": "111",
	}
)
