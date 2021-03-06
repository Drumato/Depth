use crate::ce::types::Error;
use crate::compile::ir::llvm_type::LLVMType;
use crate::compile::ir::llvm_value::LLVMValue;

use std::fmt;
use std::fmt::Write;

type Label = usize;
type TrueLabel = usize;
type FalseLabel = usize;
type IndexType = LLVMType;
type IndexValue = LLVMValue;
type TotalSize = usize;
type Alignment = usize;
type FuncName = String;
type Expr = LLVMValue;
type ReturnType = LLVMType;
type SrcType = LLVMType;
type DstType = LLVMType;
type DstReg = LLVMValue;
type Lop = LLVMValue;
type Rop = LLVMValue;
type Args = Vec<(LLVMValue, LLVMType)>;
type IsVolatile = bool;

#[derive(Clone)]
pub enum Instruction {
    RetTy(ReturnType, Expr),
    RetVoid,
    Alloca(Label, DstType, Alignment),
    Store(DstType, Expr, Label, Alignment),
    Load(Label, DstType, DstReg, Alignment),
    Add(Label, CalcMode, ReturnType, Lop, Rop),
    Sub(Label, CalcMode, ReturnType, Lop, Rop),
    Mul(Label, CalcMode, ReturnType, Lop, Rop),
    Sdiv(Label, ReturnType, Lop, Rop),
    Srem(Label, ReturnType, Lop, Rop),
    Icmp(Label, CompareMode, ReturnType, Lop, Rop),
    Shl(Label, ReturnType, Lop, Rop),
    Ashr(Label, ReturnType, Lop, Rop),
    Call(Label, ReturnType, FuncName, Args),
    BitCast(Label, SrcType, Expr, DstType),
    GetElementPtrInbounds(Label, ReturnType, Expr, IndexType, IndexValue),
    UnconditionalBranch(Label),
    ConditionalBranch(SrcType, Expr, TrueLabel, FalseLabel),

    Memcpy64(Expr, Expr, TotalSize, IsVolatile),
    DoNothing,
    NOP,
}

#[derive(Clone)]
pub enum CalcMode {
    NSW,
}
impl fmt::Display for CalcMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NSW => write!(f, "nsw"),
        }
    }
}
#[derive(Clone)]
pub enum CompareMode {
    EQUAL,
    NOTEQUAL,
    GREATERTHAN,
    GREATERTHANEQUAL,
    LESSTHAN,
    LESSTHANEQUAL,
}
impl fmt::Display for CompareMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::EQUAL => write!(f, "eq"),
            Self::NOTEQUAL => write!(f, "ne"),
            Self::GREATERTHAN => write!(f, "sgt"),
            Self::GREATERTHANEQUAL => write!(f, "sge"),
            Self::LESSTHAN => write!(f, "slt"),
            Self::LESSTHANEQUAL => write!(f, "sle"),
        }
    }
}

impl Instruction {
    pub fn dump(&self) {
        match self {
            Self::RetTy(ty, v) => println!("  ret {} {}", ty, v),
            Self::RetVoid => println!("  ret void"),
            Self::Alloca(dst, ty, alignment) => {
                println!("  %{} = alloca {}, align {}", dst, ty, alignment)
            }
            Self::Store(ty, v, label, alignment) => println!(
                "  store {} {}, {}* %{}, align {}",
                ty, v, ty, label, alignment
            ),
            Self::Load(label, ty, v, alignment) => println!(
                "  %{} = load {}, {}* {}, align {}",
                label, ty, ty, v, alignment
            ),
            Self::Add(label, mode, return_type, lop, rop) => println!(
                "  %{} = add {} {} {}, {}",
                label, mode, return_type, lop, rop,
            ),
            Self::Sub(label, mode, return_type, lop, rop) => println!(
                "  %{} = sub {} {} {}, {}",
                label, mode, return_type, lop, rop,
            ),
            Self::Mul(label, mode, return_type, lop, rop) => println!(
                "  %{} = mul {} {} {}, {}",
                label, mode, return_type, lop, rop,
            ),
            Self::Sdiv(label, return_type, lop, rop) => {
                println!("  %{} = sdiv {} {}, {}", label, return_type, lop, rop)
            }
            Self::Srem(label, return_type, lop, rop) => {
                println!("  %{} = srem {} {}, {}", label, return_type, lop, rop)
            }
            Self::Shl(label, return_type, lop, rop) => {
                println!("  %{} = shl {} {}, {}", label, return_type, lop, rop)
            }
            Self::Ashr(label, return_type, lop, rop) => {
                println!("  %{} = ashr {} {}, {}", label, return_type, lop, rop)
            }
            Self::Icmp(label, compare_type, return_type, lop, rop) => println!(
                "  %{} = icmp {} {} {}, {}",
                label, compare_type, return_type, lop, rop
            ),
            Self::Call(label, return_type, func_name, args) => {
                let mut arg_string = String::new();
                for (i, (arg_value, arg_type)) in args.iter().enumerate() {
                    if i == args.len() - 1 {
                        if let Err(err) =
                            arg_string.write_fmt(format_args!("{} {}", arg_type, arg_value))
                        {
                            Error::LLVM.found(&format!("{}", err));
                        }
                    } else {
                        if let Err(err) =
                            arg_string.write_fmt(format_args!("{} {},", arg_type, arg_value))
                        {
                            Error::LLVM.found(&format!("{}", err));
                        }
                    }
                }
                println!(
                    "  %{} = call {} @{}({})",
                    label, return_type, func_name, arg_string
                );
            }
            Self::BitCast(label, src_type, target, dst_type) => println!(
                "  %{} = bitcast {}* {} to {}*",
                label, src_type, target, dst_type
            ),
            Self::Memcpy64(dst, src, total_size, is_volatile) => println!(
            "  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 16 {}, i8* align 16 {}, i64 {}, i1 {:?})"
            ,dst,src,total_size,is_volatile,
            ),
            Self::DoNothing => println!(
                "  call void @llvm.donothing()"
                ),
            Self::GetElementPtrInbounds(label,return_type,target,idx_type,idx_value) => println!(
                "  %{} = getelementptr inbounds {}, {}* {}, i64 0, {} {}",
                label,return_type,return_type,target,idx_type,idx_value
            ),
            Self::UnconditionalBranch(label) => println!("  br label %{}",label),
            Self::ConditionalBranch(cond_type,cond_value,true_label,false_label) => println!(
                "  br {} {}, label %{}, label %{}",
                cond_type,cond_value,true_label,false_label),
            Self::NOP => (),
        }
    }
}
