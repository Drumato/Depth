use super::super::super::ce::types::Error;
use super::super::frontend::frontmanager::frontmanager::Symbol;
use super::super::frontend::parse::node::{Func, Node};
use super::super::frontend::sema::semantics::Type;
use super::basicblock::BasicBlock;
use super::instruction::Instruction;
use super::instruction::Instruction as Inst;
use super::llvm_type::LLVMType;
use super::llvm_value::{LLVMSymbol, LLVMValue};
use std::collections::BTreeMap;
pub struct Function {
    pub blocks: Vec<BasicBlock>,
    // ty: FuncType
    pub name: String,
    pub insert_point: usize,
    pub label: usize,
    pub env: BTreeMap<String, LLVMSymbol>,
}

impl Function {
    pub fn new(name: String) -> Function {
        let entry_block = BasicBlock::new(0);
        Self {
            blocks: vec![entry_block],
            name: name,
            insert_point: 0,
            label: 1,
            env: BTreeMap::new(),
        }
    }
    pub fn dump(&self) {
        for bb in self.blocks.iter() {
            println!("define i64 @{}() {}", self.name, "{");
            bb.dump();
            println!("{}", "}");
        }
    }
    pub fn add_inst(&mut self, inst: Instruction) {
        match inst {
            Instruction::Alloca(_, _, _) => {
                self.label += 1;
            }
            Instruction::Load(_, _, _, _) => {
                self.label += 1;
            }
            _ => (),
        }
        self.blocks[self.insert_point].insts.push(inst);
    }
    pub fn build_function(&mut self, f: &Func) {
        for st in f.stmts.iter() {
            match st {
                Node::RETURN(bexpr) => self.build_return(*bexpr.clone()),
                Node::LET(ident_name, bexpr) => {
                    if let Some(v) = f.env.sym_table.get(ident_name) {
                        self.build_let(ident_name.to_string(), v, *bexpr.clone())
                    } else {
                        Error::LLVM.found(&format!("{} is not defined", &ident_name));
                    }
                }
                _ => (),
            }
        }
    }
    fn build_let(&mut self, ident_name: String, symbol: &Symbol, expr: Node) {
        if let Ok(ty) = &symbol.ty {
            let llvm_type = self.get_llvmtype_from_type(ty.clone());
            let alignment = llvm_type.alignment();
            let label = self.label;
            let llvm_symbol = LLVMSymbol::new(label, llvm_type.clone());
            self.env.insert(ident_name, llvm_symbol);
            self.add_inst(Inst::Alloca(label, llvm_type.clone(), alignment));
            match expr {
                Node::INTEGER(value) => {
                    let llvm_value = LLVMValue::INTEGER(value);
                    self.add_inst(Inst::Store(llvm_type, llvm_value, label, alignment));
                }
                Node::IDENT(name) => {
                    if let Some(llvm_symbol) = self.env.get(&name) {
                        let llvm_value = LLVMValue::VREG(llvm_symbol.label);
                        self.add_inst(Inst::Store(llvm_type, llvm_value, label, alignment));
                    }
                }
                _ => (),
            }
        }
    }
    fn get_llvmtype_from_type(&mut self, ty: Type) -> LLVMType {
        match ty {
            Type::INTEGER => LLVMType::I64,
            _ => LLVMType::UNKNOWN,
        }
    }
    fn build_return(&mut self, expr: Node) {
        match expr {
            Node::INTEGER(value) => {
                let llvm_value = LLVMValue::INTEGER(value);
                self.add_inst(Inst::RetTy(LLVMType::I64, llvm_value));
            }
            Node::IDENT(name) => {
                if let Some(llvm_symbol) = self.env.get(&name) {
                    let llvm_type = llvm_symbol.ty.clone();
                    let alignment = llvm_type.alignment();
                    let llvm_value = LLVMValue::VREG(llvm_symbol.label);
                    let label = self.label;
                    self.add_inst(Inst::Load(label, llvm_type.clone(), llvm_value, alignment));
                    let dst_value = LLVMValue::VREG(label);
                    self.add_inst(Inst::RetTy(llvm_type, dst_value));
                }
            }
            _ => (),
        }
    }
}
