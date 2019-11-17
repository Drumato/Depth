use super::super::frontend::frontmanager::frontmanager::FrontManager;
use super::super::frontend::parse::node::{Func, Node};
use super::context::Context;
use super::function::Function as LLVMFunc;
use super::instruction::Instruction as Inst;
use super::llvm_type::LLVMType;
use super::llvm_value::LLVMValue;
use super::module::Module;
pub struct IRBuilder {
    module: Module,
    ctx: Context,
    functions: Vec<Func>,
}
impl IRBuilder {
    fn build_module(&mut self) {
        for f in self.functions.iter() {
            let llvm_func = self.build_function(f);
            self.module.add_func(llvm_func);
        }
    }
    fn build_function(&self, f: &Func) -> LLVMFunc {
        let mut llvm_func = LLVMFunc::new(f.name.to_string());
        for st in f.stmts.iter() {
            match st {
                Node::RETURN(bexpr) => self.build_return(&mut llvm_func, *bexpr.clone()),
                _ => (),
            }
        }
        llvm_func
    }
    fn build_return(&self, llvm_func: &mut LLVMFunc, expr: Node) {
        if let Node::INTEGER(value) = expr {
            let llvm_value = LLVMValue::INTEGER(value);
            llvm_func.add_inst(Inst::RetTy(LLVMType::I64, llvm_value));
        }
    }
    fn emit(&self) {
        self.module.dump_id();
        self.ctx.dump();
        self.module.dump();
    }
    fn new(module_id: String, entry: &str, funcs: Vec<Func>) -> Self {
        let module = Module::new(module_id.to_string(), entry.to_string());
        let ctx = Context::new(module_id);
        Self {
            module: module,
            ctx: ctx,
            functions: funcs,
        }
    }
}
pub fn emit_llvm(file_name: String, manager: FrontManager) {
    let mut builder = IRBuilder::new(file_name, "main", manager.functions);
    builder.build_module();
    builder.emit();
}
