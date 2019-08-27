.intel_syntax noprefix
.global main
main:
  push rbp
  mov rbp, rsp
  sub rsp, 8
  mov r10, 30
  mov rdi, r10
  mov rsi, r10
  call add
  call .Lend
.Lend:
  mov rsp, rbp
  pop rbp
  ret
add:
  push rbp
  mov rbp, rsp
  push rdi
  mov r10, -8[rbp]
  mov r11, 30
  add r10, r11
  mov rax, r10
  call .Lend
