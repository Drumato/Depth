pub enum Error {
    PARSE,
}

impl Error {
    pub fn found(&self, message: &String) {
        eprintln!("{}:{}", self.string(), message);
    }
    fn string(&self) -> String {
        match self {
            Error::PARSE => "ParseError".to_string(),
        }
    }
}
