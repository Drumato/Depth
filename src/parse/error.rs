extern crate colored;
use colored::*;
pub enum CompileError {
    PARSE(String),
    TYPE(String),
    IO(String),
}

impl CompileError {
    pub fn found(&self) {
        match self {
            CompileError::PARSE(msg) => eprintln!("{} {}", "Parse Error:".red().bold(), msg),
            CompileError::TYPE(msg) => eprintln!("{} {}", "Type Error:".red().bold(), msg),
            CompileError::IO(msg) => eprintln!("{} {}", "I/O Error:".red().bold(), msg),
        }
        std::process::exit(1);
    }
}
