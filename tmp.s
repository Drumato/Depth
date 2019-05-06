    .file "sample.dep"
    .intel_syntax noprefix
    .text
    .globl main
    .type main, @function
main:
    mov rsi, 6 #loadimm
    mov rax, 2 #mul
    mul rsi
    mov rsi, rax
    add rsi, 30 #add
    sub rsi, 2 #sub
    mov rax, rsi #return
    ret
