use super::constant::Constant;
use super::function::Function;
use super::intrinsic::Intrinsic;
use std::collections::HashSet;
pub struct Module {
    id: String,
    funcs: Vec<Function>,
    pub constants: Vec<Constant>,
    pub declares: HashSet<Intrinsic>,
}

impl Module {
    pub fn new(id: String) -> Self {
        Self {
            id: id,
            funcs: Vec::new(),
            constants: Vec::new(),
            declares: HashSet::new(),
        }
    }
    pub fn dump(&self) {
        for f in self.funcs.iter() {
            f.dump();
        }
    }
    pub fn dump_constants(&self) {
        for c in self.constants.iter() {
            c.dump();
        }
    }
    pub fn dump_id(&self) {
        println!(";ModuleID = '{}'", self.id);
    }
    pub fn dump_declare(&self) {
        for dec in self.declares.iter() {
            match dec {
                Intrinsic::Memcpy => {
                    println!("declare void @llvm.memcpy.p0i8.p0i8.i64(i8* nocapture writeonly, i8* nocapture readonly, i64, i1 immarg) #1");
                }
            }
        }
    }
    pub fn add_func(&mut self, f: Function) {
        self.funcs.push(f);
    }
}
