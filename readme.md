[![Build Status](https://travis-ci.org/Drumato/Depth.svg?branch=master)](https://travis-ci.org/Drumato/Depth)

# The Depth Programming Language

This is the main source code repository for Depth.  
**このリポジトリには 言語組み込みのライブラリ､コンパイラ､linter及びその他ツール群が含まれます｡**

**SecHack365の成果物**として開発しています

# Details

## compile package

- とても強い静的型付け言語  
- 開発言語→ **Rust**

## assemble package

- **x86-64** assemblyをマシン語に変換するアセンブラ｡   
- 再配置可能オブジェクトファイルを吐く  

## link package  

- オブジェクトファイルを実行形式に変換  
- 現在スタティックリンクのみサポート

## load package

- `--run` でELFの実行
- `execve(2)`を **用いていない**
  - メモリ上にELFバイナリをロードして,関数ポインタにキャスト,実行.
  
## readelf

- `--readelf [-a/-h/-r/-l/-S/-s/-d/--debug]`
  - 独自デバッグ情報を読むには `--debug`

## Author's Profile


# Profile

- screenName: **Drumato**
- Team: [IPFactory](https://ipfactory.github.io/) / OtakuAssembly
- Language: Rust/C/Zen
- Editor: Neovim
- Age: 19
- Occupation: Student
- Interests: compiler/assembler/linker/OS/binary analysis(esp ELF)/ **all low-level programming**
