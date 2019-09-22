extern crate colored;
use colored::*;
pub enum Error {
    PARSE,
    TYPE,
    ELF,
    UNDEFINED,
}

impl Error {
    pub fn found(&self, message: &String) {
        eprintln!("{}:{}", self.string().red().bold(), message);
    }
    fn string(&self) -> String {
        match self {
            Error::PARSE => "ParseError".to_string(),
            Error::TYPE => "TypeError".to_string(),
            Error::ELF => "ELFError".to_string(),
            Error::UNDEFINED => "UndefinedError".to_string(),
        }
    }
}
