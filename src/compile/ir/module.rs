use super::function::Function;
pub struct Module {
    id: String,
    funcs: Vec<Function>,
    entry: String,
}

impl Module {
    pub fn new(id: String, entry: String) -> Self {
        Self {
            id: id,
            funcs: Vec::new(),
            entry: entry,
        }
    }
    pub fn dump(&self) {
        for f in self.funcs.iter() {
            f.dump();
        }
    }
}
