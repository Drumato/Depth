pub enum Instruction {
    RET,
}

impl Instruction {
    pub fn dump(&self) {
        match self {
            Self::RET => println!("  Ret inst"),
        }
    }
}
