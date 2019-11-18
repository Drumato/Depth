use super::function::Function;
pub struct Module {
    id: String,
    funcs: Vec<Function>,
}

impl Module {
    pub fn new(id: String) -> Self {
        Self {
            id: id,
            funcs: Vec::new(),
        }
    }
    pub fn dump(&self) {
        for f in self.funcs.iter() {
            f.dump();
        }
    }
    pub fn dump_id(&self) {
        println!(";ModuleID = '{}'", self.id);
    }
    pub fn add_func(&mut self, f: Function) {
        self.funcs.push(f);
    }
}
