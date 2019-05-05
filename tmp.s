    .file "sample.dep"
    .intel_syntax noprefix
    .text
    .globl main
    .type main, @function
main:
    mov rsi, 6
    mov r10, 30
    add rsi, r10
    mov r11, 60
    sub rsi, r11
    mov r12, 20
    sub rsi, r12
    mov rax, rsi
    ret
