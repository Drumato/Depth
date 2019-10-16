extern crate colored;
use colored::*;
pub enum Error {
    PARSE,
    TYPE,
    ELF,
    ASSEMBLE,
}

impl Error {
    pub fn found(&self, message: &String) {
        eprintln!("{}:{}", self.string().red().bold(), message);
    }
    fn string(&self) -> String {
        match self {
            Self::PARSE => "ParseError".to_string(),
            Self::TYPE => "TypeError".to_string(),
            Self::ELF => "ELFError".to_string(),
            Self::ASSEMBLE => "AssembleError".to_string(),
        }
    }
}

pub enum Info {
    TYPE,
}

impl Info {
    pub fn found(&self, message: &String) {
        eprintln!("{}:{}", self.string().blue().bold(), message);
    }
    fn string(&self) -> String {
        match self {
            Self::TYPE => "TypeInfo".to_string(),
        }
    }
}
