pub mod lex;
struct Assembly {
    opcode: Mnemonic,
    lop: Operand,
    rop: Operand,
}
struct Mnemonic {
    code: u8,
    name: String,
}
struct Operand {
    reg: String,
    val: u64,
}
