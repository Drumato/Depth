.intel_syntax noprefix
.global main
main:
  push rbp
  mov rbp, rsp
  mov r10, 30
  mov r11, 30
  add r10, r11
  mov -8[rbp], r10
  mov r10, 10
  mov -16[rbp], r10
  mov r10, -8[rbp]
  mov r11, -16[rbp]
  add r10, r11
  mov rax, r10
  call .Lend
.Lend:
  mov rsp, rbp
  pop rbp
  ret
