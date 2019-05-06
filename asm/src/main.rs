use std::io::BufReader;
use std::fs::File;

extern crate asm;

fn main() {
    let mut content:String = String::new();
    match File::open("tmp.s"){
        Ok(file) => {
            let mut buf_file = BufReader::new(file);
            content = asm::util::get_content(&mut buf_file);
        }
        Err(e) => println!("Error found in tmp.s: {}",e),
    }
    println!("{}",content);
}

