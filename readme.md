[![Build Status](https://travis-ci.org/Drumato/Depth.svg?branch=master)](https://travis-ci.org/Drumato/Depth)[![Go Report Card](https://goreportcard.com/badge/github.com/Drumato/Depth)](https://goreportcard.com/report/github.com/Drumato/Depth)

# The Depth Programming Language

This is the main source code repository for Depth.  
**このリポジトリには 言語組み込みのライブラリ､コンパイラ､linter及びその他ツール群が含まれます｡**

**SecHack365の成果物**として開発しています

# Details

## Depth

- **システムプログラミング言語**
  - **厳密な型チェック**と､ **省メモリ**をコンセプトに開発中｡
- 開発言語→ **Rust**
- 言語組み込みのライブラリ開発も視野に入れている｡  
  - ネットワーク通信を実現するライブラリ
  - ハッシュ化関数等

## asm package

- **x86-64** assemblyをマシン語に変換するアセンブラ｡
  - 後々他アーキテクチャにも対応予定


## Author's Profile

- Name: **[Drumato](https://gihub.com/drumato/)**

## checklist

### parsing

- [x] some functions( `f main(){} f add(){}`)
- [x] for-stmt( `for elem in ary{ some stmts }` )
- [x] loop-stmt( `loop { some stmts }`)
- [x] let-stmt()
- [x] return-stmt( `return 30` )
- [x] if-else-stmt( `if 0 == 0 { some stmts } else { some stmts }`)
- [x] define-struct-stmt( `struct { x : i8 y : u32 }` )
- [x] binary-op
  - [x] add-sub( `10 + 30 - 4`)
  - [x] mul-div( `10 * 30 / 4`)
  - [x] shift( `10 << 2 >> 1`)
  - [x] compare( `0 <= 1`, ` 1 > 0`, `0 >= 0`, `1 < 2`)
  - [x] equal( `0 != 1`, `0 -= 0`)
  - [x] logistic-and( `2 && 1`)
  - [x] logistic-or( `3 || 1`)
- [x] array
  - [x] percent-style( `%s[abc def ghi]`)
- [x] unary-op
  - [x] signed-minus( `-30` )
  - [x] call-expression( `func()` )
- [] expr-stmt ( if 0 == 0 { 3 })
- [] pattern-match( `let (x,y) : (i8,i8) = 30,40` )
- [] return when last evaluated expression( describe below )

### compiling

- [x] load-immediate( `mov <register>, <immediate>` )
  - [x] signed int
  - [x] unsigned int(probably)
  - [x] char-lit
  - [] string-lit
- [x] return-reg( `mov rax,<register>` )
- [x] prologue( `push rbp`, `mov rbp,rsp`, `sub rsp,<stack-offset-each-environment>` )
- [x] epilogue( `mov rsp,rbp`,`pop rbp`)
- [x] label( `main:` )
- [x] store value to stack for using defined-identifier( `mov <register>, QWORD PTR <offset>[rbp]`)
