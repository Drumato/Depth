pub struct Lexer {
    pub input: String,
    pub pos: usize,
    pub npos: usize,
    pub ch: u8,
}

impl Lexer {
    pub fn new(input_str: String) -> Option<Lexer> {
        let ch: u8 = input_str.bytes().nth(0)?;
        Some(Lexer {
            input: input_str,
            pos: 0,
            npos: 1,
            ch: ch,
        })
    }
    pub fn read_char(&mut self) {
        if self.npos >= self.input.len() {
            self.ch = 0; //null termination
        } else {
            match self.input.bytes().nth(self.npos) {
                Some(c) => self.ch = c,
                None => panic!("Error found between calling read_char() function"),
            }
        }
        self.pos = self.npos;
        self.npos += 1;
    }
    pub fn peak_char(self) -> char {
        if self.npos >= self.input.len() {
            return '\0';
        } else {
            match self.input.bytes().nth(self.npos) {
                Some(c) => c as char,
                None => panic!("Error found between calling read_char() function"),
            }
        }
    }
    pub fn peak_byte(self) -> u8 {
        if self.npos >= self.input.len() {
            return 0;
        } else {
            match self.input.bytes().nth(self.npos) {
                Some(c) => c,
                None => panic!("Error found between calling read_char() function"),
            }
        }
    }
    pub fn read_ident(&mut self) -> String {
        let p: usize = self.pos;
        while self.ch.is_ascii_alphabetic() || self.ch == 0x20 {
            self.read_char();
        }
        self.input[p..self.pos].to_string()
    }
    pub fn read_number(&mut self) -> String {
        let p: usize = self.pos;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }
        self.input[p..self.pos].to_string()
    }
    pub fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }
}
