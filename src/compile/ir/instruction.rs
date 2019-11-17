use super::llvm_type::LLVMType;
use super::llvm_value::LLVMValue;
pub enum Instruction {
    RetTy(LLVMType, LLVMValue),
}

impl Instruction {
    pub fn dump(&self) {
        match self {
            Self::RetTy(ty, v) => println!("  ret {} {}", ty, v),
        }
    }
}
