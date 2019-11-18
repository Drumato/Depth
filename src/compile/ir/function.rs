use super::basicblock::BasicBlock;
use super::instruction::Instruction;
pub struct Function {
    pub blocks: Vec<BasicBlock>,
    // ty: FuncType
    pub name: String,
    pub insert_point: usize,
    pub label: usize,
}

impl Function {
    pub fn new(name: String) -> Function {
        let entry_block = BasicBlock::new(0);
        Self {
            blocks: vec![entry_block],
            name: name,
            insert_point: 0,
            label: 1,
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
        match inst {
            Instruction::Alloca(_, _, _) => {
                self.label += 1;
            }
            Instruction::Load(_, _, _, _) => {
                self.label += 1;
            }
            _ => (),
        }
        self.blocks[self.insert_point].insts.push(inst);
    }
}
