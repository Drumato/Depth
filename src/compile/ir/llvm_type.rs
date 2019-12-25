use crate::ce::types::Error;

use std::fmt;

type PointerTo = Box<LLVMType>;
type ElemType = Box<LLVMType>;
#[derive(Clone, PartialEq)]
pub enum LLVMType {
    I1,
    I8,
    I64,
    POINTER(PointerTo),
    ARRAY(ElemType, usize),
    UNKNOWN,
}

impl fmt::Display for LLVMType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::I1 => write!(f, "i1"),
            Self::I8 => write!(f, "i8"),
            Self::I64 => write!(f, "i64"),
            Self::POINTER(inner) => write!(f, "{}*", inner),
            Self::ARRAY(elem_type, length) => write!(f, "[{} x {}]", length, elem_type),
            Self::UNKNOWN => write!(f, "unknown"),
        }
    }
}

impl LLVMType {
    pub fn alignment(&self) -> usize {
        match self {
            Self::I1 => 1,
            Self::I8 => 1,
            Self::I64 => 8,
            Self::POINTER(_) => 8,
            Self::ARRAY(elem_type, _) => elem_type.alignment(),
            Self::UNKNOWN => {
                Error::LLVM.found(&"LLVMType::UNKNOWN has not alignment".to_string());
                0
            }
        }
    }
}
