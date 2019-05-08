    .file "sample.dep"
    .intel_syntax noprefix
    .text
    .globl main
    .type main, @function
main:
    push rbp #prologue
    mov rbp, rsp
    sub rsp, 0x20 #allocate
    mov QWORD PTR -32[rbp], 0x41 #store
    mov r10, QWORD PTR -32[rbp] #load
    mov rax, r10 #return
    mov rsp, rbp
    pop rbp#load
    ret
