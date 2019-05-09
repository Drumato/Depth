    .file "sample.dep"
    .intel_syntax noprefix
    .text
    .globl main
    .type main, @function
main:
    push rbp #prologue
    mov rbp, rsp
    sub rsp, 0x8 #allocate
    mov QWORD PTR -8[rbp], 0xa #store
    mov rsi, QWORD PTR -8[rbp] #load
    mov r10, 0x0 #loadimm
    cmp rsi, r10
    jg .L2 #gt
    mov rsi, 0x0 #loadimm
    jmp .L3 #jump
.L2: #label
    mov rsi, 0x1 #loadimm
.L3: #label
    mov r11, QWORD PTR -8[rbp] #load
    mov rax, r11 #return
    mov rsp, rbp #epilogue
    pop rbp
    ret
