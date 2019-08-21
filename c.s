.intel_syntax noprefix
.global main
main:
  push rbp
  mov rbp, rsp
  sub rsp, 16
  mov r10, 30
  mov r11, 30
  add r10, r11
  mov -8[rbp], r10
  mov r10, 10
  mov -16[rbp], r10
  mov r10, -8[rbp]
  mov r11, -16[rbp]
  cmp r10, r11
  setg al
  movzx r10, al
  cmp r10, 0
  je .L0
  mov r11, -8[rbp]
  mov r12, -16[rbp]
  add r11, r12
  mov rax, r11
  call .Lend
  jmp .L1
.L0:
  mov r12, -16[rbp]
  mov rax, r12
  call .Lend
.L1:
.Lend:
  mov rsp, rbp
  pop rbp
  ret
