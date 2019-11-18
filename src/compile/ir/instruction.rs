use super::llvm_type::LLVMType;
use super::llvm_value::LLVMValue;
type Label = usize;
type Alignment = usize;
pub enum Instruction {
    RetTy(LLVMType, LLVMValue),
    Alloca(Label, LLVMType, Alignment),
    Store(LLVMType, LLVMValue, Label, Alignment),
    Load(Label, LLVMType, LLVMValue, Alignment),
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
        }
    }
}
