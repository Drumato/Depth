.intel_syntax noprefix
.globl main
main:
    push rbp
    mov rbp,rsp
    call today
    mov rsp,rbp
    pop rbp
    ret
today:
    push rbp
    mov rbp,rsp
    mov rbx, 0x9
    mov rax, rbx
    mov rsp,rbp
    pop rbp
    ret
