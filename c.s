.intel_syntax noprefix
.globl main
main:
    push rbp
    mov rbp,rsp
    sub rsp, 0x8
    mov rbx, 0x1e
    mov QWORD PTR -8[rbp], rbx
    mov rbx, QWORD PTR -8[rbp]
    mov rcx, 0x0
    cmp rbx, rcx
    jg .L0
    mov rcx, QWORD PTR -8[rbp]
    mov rax, rcx
    jmp .L1
.L0:
    mov rdx, 0x0
    mov rax, rdx
.L1:
    mov rsp,rbp
    pop rbp
    ret
