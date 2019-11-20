use super::super::frontend::frontmanager::frontmanager::FrontManager;
use super::super::frontend::parse::node::Func;
use super::context::Context;
use super::function::Function as LLVMFunc;
use super::module::Module;

pub struct IRBuilder {
    module: Module,
    ctx: Context,
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
            let mut llvm_func = LLVMFunc::new(f.name.to_string());
            llvm_func.build_function(f);
            self.module.add_func(llvm_func);
        }
    }
    fn new(module_id: String, funcs: Vec<Func>) -> Self {
        let module = Module::new(module_id.to_string());
        let ctx = Context::new(module_id);
        Self {
            module: module,
            ctx: ctx,
            functions: funcs,
        }
    }
}
pub fn emit_llvm(file_name: String, manager: FrontManager) {
    // manager is no longer used.
    let mut builder = IRBuilder::new(file_name, manager.functions);
    builder.build_module();
    builder.emit();
}
