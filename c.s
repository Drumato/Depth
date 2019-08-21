.intel_syntax noprefix
.global main
main:
  push rbp
  mov rbp, rsp
  mov r10, 1
  mov r11, 0
  cmp r10, r11
  setl al
  movzx r10, al
  cmp r10, 0
  je .L0
  mov r11, 30
  mov rax, r11
  call .Lend
  jmp .L1
.L0:
  mov r12, 20
  mov rax, r12
  call .Lend
.L1:
.Lend:
  mov rsp, rbp
  pop rbp
  ret
