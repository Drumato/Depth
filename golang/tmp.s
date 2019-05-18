    .file "sample/sample1.dep"
    .intel_syntax noprefix
    .text
    .globl main
    .type main, @function
main:
    push rbp #prologue
    mov rbp, rsp
    sub rsp, 0x10 #allocate
    mov QWORD PTR -8[rbp], 0x1e #store
    mov QWORD PTR -16[rbp], 0x32 #store
    mov rsi, QWORD PTR -8[rbp] #load
    mov r10, QWORD PTR -16[rbp] #load
    add rsi, r10 #add
    mov rax, rsi #return
    mov rsp, rbp #epilogue
    pop rbp
    ret
