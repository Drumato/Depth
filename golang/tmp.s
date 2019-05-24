    .file "sample.dep"
    .intel_syntax noprefix
    .text
    .globl main
    .type main, @function
main:
    push rbp
    mov rbp, rsp
    mov rsi, 0x1e
    mov r10, 0x32
    add rsi, r10
    mov rax, rsi
    mov rsp, rbp
    pop rbp
    ret
