;ModuleID = 'test/array.dep'
source_filename = "test/array.dep"
target triple = "x86_64-pc-linux-gnu"
target datalayout = "e-m:e-i64:64-f80:128-n8:16:32:64-S128"

@__const.main.0 = private unnamed_addr constant [3 x i64] [i64 1, i64 5, i64 15], align 8

define i64 @main() {
entry:
  %0 = alloca [3 x i64], align 8
  %1 = bitcast [3 x i64]* %0 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 16 %1, i8* align 16 bitcast ([3 x i64]* @__const.main.0 to i8*), i64 24, i1 false)

  ret unknown unknown
}
