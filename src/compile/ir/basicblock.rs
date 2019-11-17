use super::instruction::Instruction;
pub struct BasicBlock {
    label: usize,
    insts: Vec<Instruction>,
    // prev: &mut BasicBlock
    // next: &mut BasicBlock
}

impl BasicBlock {
    pub fn dump(&self) {
        for inst in self.insts.iter() {
            println!("{}:", self.label);
            inst.dump();
        }
    }
}
