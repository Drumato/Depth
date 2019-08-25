.intel_syntax noprefix
.global main
main:
  push rbp
  mov rbp, rsp
  sub rsp, 16
  mov r10, 30
  mov r11, 30
  add r10, r11
  #start IR::STORE
  mov -8[rbp], r10
  mov r10, 10
  #start IR::STORE
  mov -16[rbp], r10
  #start IR::LOAD
  mov r10, -8[rbp]
  #start IR::LOAD
  mov r11, -16[rbp]
  cmp r10, r11
  setg al
  movzx r10, al
  cmp r10, 0
  je .L0
  #start IR::LOAD
  mov r11, -8[rbp]
  #start IR::LOAD
  mov r12, -16[rbp]
  add r11, r12
  mov rax, r11
  call .Lend
  jmp .L1
.L0:
  #start IR::LOAD
  mov r11, -16[rbp]
  mov rax, r11
  call .Lend
.L1:
.Lend:
  mov rsp, rbp
  pop rbp
  ret
