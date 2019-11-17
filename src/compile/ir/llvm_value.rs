use std::fmt;
pub enum LLVMValue {
    INTEGER(i128),
}
impl fmt::Display for LLVMValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::INTEGER(v) => write!(f, "{}", v),
        }
    }
}
