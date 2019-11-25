;ModuleID = 'test/condloop.dep'
source_filename = "test/condloop.dep"
target triple = "x86_64-pc-linux-gnu"
target datalayout = "e-m:e-i64:64-f80:128-n8:16:32:64-S128"
define i64 @main() {
entry:
  %0 = alloca i64, align 8
  store i64 10, i64* %0, align 8
  %1 = alloca i64, align 8
  store i64 0, i64* %1, align 8
  %2 = load i64, i64* %1, align 8
  ret i64 %2
}
