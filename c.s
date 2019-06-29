.intel_syntax noprefix
.globl main
main:
    push rbp
    mov rbp,rsp
    sub rsp, 0x20
    mov rbx, 0x1e
    mov QWORD PTR -16[rbp], rbx
    mov rbx, 0x1e
    mov QWORD PTR -32[rbp], rbx
    mov rbx, QWORD PTR -16[rbp]
    mov rcx, QWORD PTR -32[rbp]
    add rbx, rcx
    mov rax, rbx
    mov rsp,rbp
    pop rbp
    ret
