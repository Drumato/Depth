use super::llvm_type::LLVMType;
use super::llvm_value::LLVMValue;
type Label = usize;
type Alignment = usize;
type Expr = LLVMValue;
type ReturnType = LLVMType;
type DstType = LLVMType;
type DstReg = LLVMValue;
type Lop = LLVMValue;
type Rop = LLVMValue;

use std::fmt;
pub enum Instruction {
    RetTy(ReturnType, Expr),
    Alloca(Label, DstType, Alignment),
    Store(DstType, Expr, Label, Alignment),
    Load(Label, DstType, DstReg, Alignment),
    Add(Label, CalcMode, ReturnType, Lop, Rop),
    Sub(Label, CalcMode, ReturnType, Lop, Rop),
    Mul(Label, CalcMode, ReturnType, Lop, Rop),
}

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

impl Instruction {
    pub fn dump(&self) {
        match self {
            Self::RetTy(ty, v) => println!("  ret {} {}", ty, v),
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
        }
    }
}
