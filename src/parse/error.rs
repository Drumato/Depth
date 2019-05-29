pub enum CompileError {
    PARSE(String),
    TYPE(String),
    IO(String),
}

impl CompileError {
    pub fn found(&self) {
        match self {
            CompileError::PARSE(msg) => eprintln!("Parse Error:{}", msg),
            CompileError::TYPE(msg) => eprintln!("Type Error:{}", msg),
            CompileError::IO(msg) => eprintln!("I/O Error:{}", msg),
        }
    }
}
