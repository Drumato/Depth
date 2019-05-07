    .file "sample.dep"
    .intel_syntax noprefix
    .text
    .globl main
    .type main, @function
main:
    mov rsi, 300 #loadimm
    mov r10, 6 #loadimm
    mov r11, 2 #loadimm
    mov rax, r11 #mul
    mul r10
    mov r10, rax
    sub rsi, r10 #sub
    mov r10, 30 #loadimm
    mov r11, 2 #loadimm
    mov rax, r10 #div
    cqo
    div r11
    mov r10, rax
    add rsi, r10 #add
    mov rax, rsi #return
    ret
