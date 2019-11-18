use super::super::super::ce::types::Error;
use super::super::frontend::frontmanager::frontmanager::{FrontManager, Symbol};
use super::super::frontend::parse::node::{Func, Node};
use super::super::frontend::sema::semantics::Type;
use super::context::Context;
use super::function::Function as LLVMFunc;
use super::instruction::Instruction as Inst;
use super::llvm_type::LLVMType;
use super::llvm_value::{LLVMSymbol, LLVMValue};
use super::module::Module;

use std::collections::BTreeMap;
pub struct IRBuilder {
    module: Module,
    ctx: Context,
    env: BTreeMap<String, LLVMSymbol>,
    functions: Vec<Func>,
}
impl IRBuilder {
    fn emit(&self) {
        self.module.dump_id();
        self.ctx.dump();
        self.module.dump();
    }
    fn build_module(&mut self) {
        let functions = self.functions.clone();
        for f in functions.iter() {
            let llvm_func = self.build_function(f);
            self.module.add_func(llvm_func);
        }
    }
    fn build_function(&mut self, f: &Func) -> LLVMFunc {
        let mut llvm_func = LLVMFunc::new(f.name.to_string());
        for st in f.stmts.iter() {
            match st {
                Node::RETURN(bexpr) => self.build_return(&mut llvm_func, *bexpr.clone()),
                Node::LET(ident_name, bexpr) => {
                    if let Some(v) = f.env.sym_table.get(ident_name) {
                        self.build_let(&mut llvm_func, ident_name.to_string(), v, *bexpr.clone())
                    } else {
                        Error::LLVM.found(&format!("{} is not defined", &ident_name));
                    }
                }
                _ => (),
            }
        }
        llvm_func
    }
    fn build_return(&mut self, llvm_func: &mut LLVMFunc, expr: Node) {
        match expr {
            Node::INTEGER(value) => {
                let llvm_value = LLVMValue::INTEGER(value);
                llvm_func.add_inst(Inst::RetTy(LLVMType::I64, llvm_value));
            }
            Node::IDENT(name) => {
                if let Some(llvm_symbol) = self.env.get(&name) {
                    let llvm_type = llvm_symbol.ty.clone();
                    let alignment = llvm_type.alignment();
                    let llvm_value = LLVMValue::VREG(llvm_symbol.label);
                    let label = llvm_func.label;
                    llvm_func.add_inst(Inst::Load(label, llvm_type.clone(), llvm_value, alignment));
                    let dst_value = LLVMValue::VREG(label);
                    llvm_func.add_inst(Inst::RetTy(llvm_type, dst_value));
                }
            }
            _ => (),
        }
    }
    fn build_let(
        &mut self,
        llvm_func: &mut LLVMFunc,
        ident_name: String,
        symbol: &Symbol,
        expr: Node,
    ) {
        if let Ok(ty) = &symbol.ty {
            let llvm_type = self.get_llvmtype_from_type(ty.clone());
            let alignment = llvm_type.alignment();
            let label = llvm_func.label;
            let llvm_symbol = LLVMSymbol::new(label, llvm_type.clone());
            self.env.insert(ident_name, llvm_symbol);
            llvm_func.add_inst(Inst::Alloca(label, llvm_type.clone(), alignment));
            match expr {
                Node::INTEGER(value) => {
                    let llvm_value = LLVMValue::INTEGER(value);
                    llvm_func.add_inst(Inst::Store(llvm_type, llvm_value, label, alignment));
                }
                Node::IDENT(name) => {
                    if let Some(llvm_symbol) = self.env.get(&name) {
                        let llvm_value = LLVMValue::VREG(llvm_symbol.label);
                        llvm_func.add_inst(Inst::Store(llvm_type, llvm_value, label, alignment));
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
    fn new(module_id: String, funcs: Vec<Func>) -> Self {
        let module = Module::new(module_id.to_string());
        let ctx = Context::new(module_id);
        Self {
            module: module,
            ctx: ctx,
            functions: funcs,
            env: BTreeMap::new(),
        }
    }
}
pub fn emit_llvm(file_name: String, manager: FrontManager) {
    // manager is no longer used.
    let mut builder = IRBuilder::new(file_name, manager.functions);
    builder.build_module();
    builder.emit();
}
