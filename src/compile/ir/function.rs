use super::basicblock::BasicBlock;
pub struct Function {
    blocks: Vec<BasicBlock>,
    // ty: FuncType
    name: String,
}

impl Function {
    pub fn dump(&self) {
        for bb in self.blocks.iter() {
            println!("define @{}() {}", self.name, "{");
            bb.dump();
            println!("{}", "}");
        }
    }
}
