use super::basicblock::BasicBlock;
use super::instruction::Instruction;
pub struct Function {
    blocks: Vec<BasicBlock>,
    // ty: FuncType
    name: String,
    insert_point: usize,
    label: usize,
}

impl Function {
    pub fn new(name: String) -> Function {
        Self {
            blocks: Vec::new(),
            name: name,
            insert_point: 0,
            label: 0,
        }
    }
    pub fn dump(&self) {
        for bb in self.blocks.iter() {
            println!("define i64 @{}() {}", self.name, "{");
            bb.dump();
            println!("{}", "}");
        }
    }
    pub fn add_inst(&mut self, inst: Instruction) {
        if self.blocks.len() == 0 {
            let entry_block = BasicBlock::new(self.label);
            self.label += 1;
            self.blocks.push(entry_block);
        }
        self.blocks[self.insert_point].insts.push(inst);
    }
}
