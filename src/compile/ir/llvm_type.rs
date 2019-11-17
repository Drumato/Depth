use std::fmt;
pub enum LLVMType {
    I64,
}

impl fmt::Display for LLVMType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::I64 => write!(f, "i64"),
        }
    }
}
