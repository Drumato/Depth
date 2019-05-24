[![Build Status](https://travis-ci.org/Drumato/Depth.svg?branch=master)](https://travis-ci.org/Drumato/Depth)[![Go Report Card](https://goreportcard.com/badge/github.com/Drumato/Depth)](https://goreportcard.com/report/github.com/Drumato/Depth)

# The Depth Programming Language

This is the main source code repository for Depth.  
**このリポジトリには 言語組み込みのライブラリ､コンパイラ､linter及びその他ツール群が含まれます｡**

**SecHack365の成果物**として開発しています｡

# Directory configuration

- **Golang**
  - 主にプロトタイプとして開発している｡
  - Rustでの本実装に先駆けてコンパイラの実装を置いています｡
- **Rust**
  - **本実装**であり､本成果物の**本体**
  



# Details

## Depth

- **システムプログラミング言語**
  - **厳密な型チェック**と､ **省メモリ**をコンセプトに開発中｡
- 開発言語→ **Golang**
  - **パフォーマンスチューニングのために後々Rustで再実装予定**
  - まずはプロトタイピングとして実装している｡
- 言語組み込みのライブラリ開発も視野に入れている｡  
  - ネットワーク通信を実現するライブラリ
  - ハッシュ化関数等

## asm package

- **x86-64** assemblyをマシン語に変換するアセンブラ｡
  - 後々他アーキテクチャにも対応予定


## Author's Profile

- Name: **[Drumato](https://gihub.com/drumato/)**



