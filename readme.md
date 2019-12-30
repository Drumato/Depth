# The Depth Programming Language

a Toy Infrastructure of executable program from scratch.  
Depth can compile depth-lang, assemble x86_64 assembly,  
link a object file, load virtual memory by `--run`.

# Details

## compile package

- translate AST to three-address code
- depth-lang -> lex -> parse -> sema -> transIR -> liveness -> codegen -> x86_64 asm
- can emit LLVM-IR with `--emit-llvm` flag.

## assemble package

- a assembler which can assemble x86_64 assembly and generate ET_REL object file.

## link package  

- a static linker
- can link a objectfile
- this linker can resolve symbols, determine entry point.

## load package

- a loader implemented in user space.
- execute a static-linked binary with `--run` flag.
- don't use `execve(2)` syscall.
  
## readelf

- a analyzer which can be used as GNU readelf.
- `--readelf [-a/-h/-r/-l/-S/-s/-d/--debug]`
- can analyze self-desined debug informations with `--debug` flag

## checksec

- a checker which can detect some security-mechanisms are in a binary.
  - RELRO
  - NX-bit( stack execution )
  - stack protector( canary )
  - PIE
  - `DT_RPATH`
  - `DT_RUNPATH`


## Author's Profile


# Profile

- screenName: **Drumato**
- Team: [IPFactory](https://ipfactory.github.io/) / OtakuAssembly
- Language: Rust/C/Haskell/Zen
- Editor: Neovim
- Occupation: Student
- Interests: compiler/assembler/linker/OS/binary analysis(esp ELF)/ **all low-level programming**
