.intel_syntax noprefix
.global main
main:
  push rbp
  mov rbp, rsp
  mov r10, 1
  mov r11, 1
  cmp r10, r11
  sete al
  movzx r10, al
  mov rax, r10
  call .Lend
.Lend:
  mov rsp, rbp
  pop rbp
  ret
