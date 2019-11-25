use std::fmt::Write;

use super::llvm_type::LLVMType;
use super::llvm_value::LLVMValue;
#[derive(Clone)]
pub enum Constant {
    Array(String, LLVMType, Vec<(LLVMType, LLVMValue)>),
}

impl Constant {
    pub fn dump(&self) {
        match self {
            Self::Array(name, ty, elements) => {
                let mut constant_string = String::new();
                let mut alignment = 0;
                for (i, (ty, v)) in elements.iter().enumerate() {
                    if i == elements.len() - 1 {
                        let _ = constant_string.write_fmt(format_args!("{} {}", ty, v));
                        alignment = ty.alignment();
                    } else {
                        let _ = constant_string.write_fmt(format_args!("{} {}, ", ty, v));
                    }
                }
                println!(
                    "\n{} = private unnamed_addr constant {} [{}], align {}\n",
                    name, ty, constant_string, alignment
                );
            }
        }
    }
}
//@__const.main.x = private unnamed_addr constant [3 x i64] [i64 1, i64 5, i64 15], align 16
