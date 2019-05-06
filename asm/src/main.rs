use std::fs::File;
use std::io::BufReader;

extern crate asm;

fn main() {
    let content: String = String::new();
    match File::open("tmp.s") {
        Ok(file) => {
            let mut buf_file = BufReader::new(file);
            asm::lex::iter_lines(&mut buf_file);
        }
        Err(e) => println!("Error found in tmp.s: {}", e),
    }
    println!("{}", content);
}
