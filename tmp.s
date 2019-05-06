    .file "sample.dep"
    .intel_syntax noprefix
    .text
    .globl main
    .type main, @function
main:
    mov rsi, 300 #loadimm
    mov rax, 2 #mul
    mul r14
    mov r14, rax
    sub rsi, 6 #sub
    add rsi, 30 #add
    mov rax, rsi #return
    ret
