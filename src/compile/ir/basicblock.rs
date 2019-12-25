use crate::compile::ir::instruction::Instruction;

#[derive(Clone)]
pub struct BasicBlock {
    entry: String,
    pub insts: Vec<Instruction>,
    // prev: &mut BasicBlock
    // next: &mut BasicBlock
}

impl BasicBlock {
    pub fn new(e: String) -> Self {
        Self {
            entry: e,
            insts: Vec::new(),
        }
    }
    pub fn dump(&self) {
        println!("{}:", self.entry);
        for inst in self.insts.iter() {
            inst.dump();
        }
    }
}
