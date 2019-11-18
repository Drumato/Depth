use super::instruction::Instruction;
pub struct BasicBlock {
    label: usize,
    pub insts: Vec<Instruction>,
    // prev: &mut BasicBlock
    // next: &mut BasicBlock
}

impl BasicBlock {
    pub fn new(label: usize) -> Self {
        Self {
            label: label,
            insts: Vec::new(),
        }
    }
    pub fn dump(&self) {
        println!("{}:", self.label);
        for inst in self.insts.iter() {
            inst.dump();
        }
    }
}
