use super::llvm_type::LLVMType;
use std::fmt;
#[derive(Clone)]
pub enum LLVMValue {
    INTEGER(i128),
    VREG(usize),
    UNKNOWN,
}
impl fmt::Display for LLVMValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::INTEGER(v) => write!(f, "{}", v),
            Self::VREG(i) => write!(f, "%{}", i),
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
