.intel_syntax noprefix
.global main
fact:
  push rbp
  push r12
  push r13
  push r14
  push r15
  mov rbp, rsp
  push rdi
  mov r10, -8[rbp]
  mov r11, 0
  cmp r10, r11
  sete al
  movzx r10, al
  cmp r10, 0
  je .L0
  mov r11, 1
  mov rax, r11
  call .Lend
.L0:
  mov r11, -8[rbp]
  mov r12, -8[rbp]
  mov r13, 1
  sub r12, r13
  mov rdi, r12
  call fact
  mov r13, rax
  mov rax, r11
  imul r13
  mov r11, rax
  mov rax, r12
  call .Lend
main:
  push rbp
  push r12
  push r13
  push r14
  push r15
  mov rbp, rsp
  mov r10, 4
  mov rdi, r10
  call fact
  mov r11, rax
  mov rax, r11
  call .Lend
.Lend:
  mov rsp, rbp
  pop r15
  pop r14
  pop r13
  pop r12
  pop rbp
  ret
