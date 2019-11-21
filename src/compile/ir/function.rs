use super::super::super::ce::types::Error;
use super::super::frontend::frontmanager::frontmanager::Symbol;
use super::super::frontend::parse::node::{Func, Node};
use super::super::frontend::sema::semantics::Type;
use super::basicblock::BasicBlock;
use super::instruction::CalcMode;
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
    pub fn add_inst(&mut self, inst: Inst) {
        match inst {
            Inst::Alloca(_, _, _) => {
                self.label += 1;
            }
            Inst::Load(_, _, _, _) => {
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
            let llvm_type = self.get_llvmtype_from_type(ty);
            let alignment = llvm_type.alignment();
            let label = self.label;
            let llvm_symbol = LLVMSymbol::new(label, llvm_type.clone());
            self.env.insert(ident_name, llvm_symbol);
            self.add_inst(Inst::Alloca(label, llvm_type.clone(), alignment));
            let (llvm_value, _) = self.build_expr(expr);
            self.add_inst(Inst::Store(llvm_type, llvm_value, label, alignment));
        }
    }
    fn build_expr(&mut self, expr: Node) -> (LLVMValue, LLVMType) {
        let label = self.label;
        match expr {
            Node::INTEGER(value) => (LLVMValue::INTEGER(value), LLVMType::I64),
            Node::IDENT(name) => {
                let llvm_symbol = self.get_symbol_if_defined(&name);
                let label = llvm_symbol.label;
                let llvm_type = llvm_symbol.ty.clone();
                let alignment = llvm_type.alignment();
                let llvm_value = LLVMValue::VREG(llvm_symbol.label);
                self.add_inst(Inst::Load(
                    label + 1,
                    llvm_type.clone(),
                    llvm_value,
                    alignment,
                ));
                (LLVMValue::VREG(label + 1), llvm_type)
            }
            Node::ADD(blop, brop) => {
                let (lop, lop_type) = self.build_expr(*blop);
                let (rop, rop_type) = self.build_expr(*brop);
                if lop_type == rop_type {
                    self.add_inst(Inst::Add(label + 1, CalcMode::NSW, lop_type, lop, rop));
                    return (LLVMValue::VREG(self.label), rop_type);
                } else {
                    Error::LLVM.found(&format!(
                        "type inference failed between {} and {}",
                        lop, rop
                    ));
                    return (LLVMValue::UNKNOWN, LLVMType::UNKNOWN);
                }
            }
            Node::SUB(blop, brop) => {
                let (lop, lop_type) = self.build_expr(*blop);
                let (rop, rop_type) = self.build_expr(*brop);
                if lop_type == rop_type {
                    self.add_inst(Inst::Sub(label + 1, CalcMode::NSW, lop_type, lop, rop));
                    return (LLVMValue::VREG(self.label), rop_type);
                } else {
                    Error::LLVM.found(&format!(
                        "type inference failed between {} and {}",
                        lop, rop
                    ));
                    return (LLVMValue::UNKNOWN, LLVMType::UNKNOWN);
                }
            }
            Node::MUL(blop, brop) => {
                let (lop, lop_type) = self.build_expr(*blop);
                let (rop, rop_type) = self.build_expr(*brop);
                if lop_type == rop_type {
                    self.add_inst(Inst::Mul(label + 1, CalcMode::NSW, lop_type, lop, rop));
                    return (LLVMValue::VREG(self.label), rop_type);
                } else {
                    Error::LLVM.found(&format!(
                        "type inference failed between {} and {}",
                        lop, rop
                    ));
                    return (LLVMValue::UNKNOWN, LLVMType::UNKNOWN);
                }
            }
            Node::DIV(blop, brop) => {
                let (lop, lop_type) = self.build_expr(*blop);
                let (rop, rop_type) = self.build_expr(*brop);
                if lop_type == rop_type {
                    self.add_inst(Inst::Sdiv(label + 1, lop_type, lop, rop));
                    return (LLVMValue::VREG(self.label), rop_type);
                } else {
                    Error::LLVM.found(&format!(
                        "type inference failed between {} and {}",
                        lop, rop
                    ));
                    return (LLVMValue::UNKNOWN, LLVMType::UNKNOWN);
                }
            }
            _ => (LLVMValue::UNKNOWN, LLVMType::UNKNOWN),
        }
    }
    fn get_llvmtype_from_type(&mut self, ty: &Type) -> LLVMType {
        match ty {
            Type::INTEGER => LLVMType::I64,
            _ => LLVMType::UNKNOWN,
        }
    }
    fn build_return(&mut self, expr: Node) {
        let (llvm_value, llvm_type) = self.build_expr(expr);
        self.add_inst(Inst::RetTy(llvm_type, llvm_value));
    }
    fn get_symbol_if_defined(&mut self, name: &str) -> &LLVMSymbol {
        if let Some(llvm_symbol) = self.env.get(name) {
            return llvm_symbol;
        } else {
            Error::LLVM.found(&format!("{} is not defined", &name));
            return &LLVMSymbol {
                label: 0,
                ty: LLVMType::UNKNOWN,
            };
        }
    }
}
