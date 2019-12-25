use crate::compile::ir::llvm_type::LLVMType;
use std::fmt;
#[derive(Clone)]
pub enum LLVMValue {
    INTEGER(i128),
    VREG(usize),
    ConstBitCast(LLVMType, String, String, LLVMType),
    Const(String),
    UNKNOWN,
}
impl fmt::Display for LLVMValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::INTEGER(v) => write!(f, "{}", v),
            Self::VREG(i) => write!(f, "%{}", i),
            Self::ConstBitCast(src, func_name, const_name, dst) => write!(
                f,
                "bitcast ({}* @__const.{}.{} to {}*)",
                src, func_name, const_name, dst
            ),
            Self::Const(name) => write!(f, "{}", name),
            Self::UNKNOWN => write!(f, "unknown"),
        }
    }
}

#[derive(Clone)]
pub struct LLVMSymbol {
    pub label: usize,
    pub ty: LLVMType,
}

impl LLVMSymbol {
    pub fn new(label: usize, ty: LLVMType) -> Self {
        Self {
            label: label,
            ty: ty,
        }
    }
}
