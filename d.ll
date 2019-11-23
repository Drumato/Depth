;ModuleID = 'test/with_argument.dep'
source_filename = "test/with_argument.dep"
target triple = "x86_64-pc-linux-gnu"
target datalayout = "e-m:e-i64:64-f80:128-n8:16:32:64-S128"
define i64 @main() {
entry:
  %0 = call i64 @ade(i64 20,i64 40)
  ret i64 %0
}
define i64 @ade(i64,i64) {
entry:
  %0 = alloca i64, align 8
  %1 = alloca i64, align 8
  store i64 %2, i64* %0, align 8
  store i64 %2, i64* %1, align 8
  %2 = load i64, i64* %0, align 8
  %3 = load i64, i64* %1, align 8
  %4 = add nsw i64 %3, %4
  ret i64 %4
}
