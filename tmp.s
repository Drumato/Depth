    .file "sample.dep"
    .intel_syntax noprefix
    .text
    .globl main
    .type main, @function
main:
    push rbp #prologue
    mov rbp, rsp
    mov rsi, 0x1e #loadimm
    mov r10, 0x28 #loadimm
    mov rax, r10 #mul
    mul rsi
    mov rsi, rax
    mov r10, 0x1e #loadimm
    add rsi, r10 #add
    mov rax, rsi #return
    mov rsp, rbp #epilogue
    pop rbp
    ret
