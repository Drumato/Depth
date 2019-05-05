    .file "sample.dep"
    .intel_syntax noprefix
    .text
    .globl main
    .type main, @function
main:
    mov rdi, 6
    mov rsi, 30
    add rdi, rsi
    mov r10, 60
    sub rdi, r10
    mov r11, 20
    sub rdi, r11
    mov rax, rdi
    ret
