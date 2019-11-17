use super::super::frontend::frontmanager::frontmanager::FrontManager;
use super::super::frontend::parse::node::Func;
use super::context::Context;
use super::module::Module;
pub struct IRBuilder {
    module: Module,
    ctx: Context,
    functions: Vec<Func>,
}
impl IRBuilder {
    fn emit(&self) {
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
    let builder = IRBuilder::new(file_name, "main", manager.functions);
    builder.emit();
}
