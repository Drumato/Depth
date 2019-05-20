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
}
