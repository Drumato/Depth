.intel_syntax noprefix
.globl main
main:
    push rbp
    mov rbp,rsp
    sub rsp, 0x20
    mov rbx, 0x41
    mov QWORD PTR -4[rbp], rbx
    mov rbx, QWORd PTR -4[rbp]
    mov rax, rbx
    mov rsp,rbp
    pop rbp
    ret
