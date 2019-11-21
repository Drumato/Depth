use super::super::super::ce::types::Error;
use std::fmt;
#[derive(Clone, PartialEq)]
pub enum LLVMType {
    I1,
    I64,
    UNKNOWN,
}

impl fmt::Display for LLVMType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::I1 => write!(f, "i1"),
            Self::I64 => write!(f, "i64"),
            Self::UNKNOWN => write!(f, "unknown"),
        }
    }
}

impl LLVMType {
    pub fn alignment(&self) -> usize {
        match self {
            Self::I1 => 1,
            Self::I64 => 8,
            Self::UNKNOWN => {
                Error::LLVM.found(&"LLVMType::UNKNOWN has not alignment".to_string());
                0
            }
        }
    }
}
