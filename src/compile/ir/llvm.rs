use crate::compile::frontend;
use crate::compile::ir;
use frontend::frontmanager::frontmanager::FrontManager;
use frontend::parse::node::Func;
use ir::context::Context;
use ir::function::Function as LLVMFunc;
use ir::module::Module;

pub struct IRBuilder {
    pub module: Module,
    pub ctx: Context,
    pub functions: Vec<Func>,
}
impl IRBuilder {
    fn emit(&self) {
        self.module.dump_id();
        self.ctx.dump();
        self.module.dump_constants();
        self.module.dump();
        self.module.dump_declare();
    }
    fn build_module(&mut self) {
        let functions = self.functions.clone();
        for f in functions.iter() {
            let mut llvm_func = LLVMFunc::new(f.name.to_string(), f.args.len());
            llvm_func.build_function(f);
            self.module
                .constants
                .append(&mut llvm_func.constants.clone());
            self.module.declares = llvm_func.declares.clone();
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
