use std::fs::File;
use std::io::{BufReader, Write};

extern crate asm;

fn main() -> Result<(), Box<std::error::Error>> {
    let magicnumber: u128 = 0x7f454c46020101000000000000000000; //0~15 byte
    let fty: u16 = 0x1; //16~17
    let march: u16 = 0x3e; //18~19
    let fver: u32 = 0x1; //20~23
    let epoint: u64 = 0x0; // 24~31
    let phoff: u32 = 0x00; //
    let shoff: u32 = 0x82;
    let flags: u32 = 0x0;
    let size: u16 = 0x40;
    let phsize: u16 = 0x00;
    let phnum: u16 = 0x0;
    let shsize: u16 = 0x40;
    let shnum: u16 = 0xb;
    let shstridx: u16 = 0xa;
    let param = (
        magicnumber,
        fty,
        march,
        fver,
        epoint,
        phoff,
        shoff,
        flags,
        size,
        phsize,
        phnum,
        shsize,
        shnum,
        shstridx,
    );
    let ehdr = asm::elf::Elf64Header::new(param);
    let mut file = File::create("sample.o")?;
    write!(file, "{}", ehdr.binary_dump(false))?;
    file.flush()?;
    Ok(())
    //lexing();
}

fn lexing() {
    let mut content: String = String::new();
    match File::open("tmp.s") {
        Ok(file) => {
            let mut buf_file = BufReader::new(file);
            asm::lex::iter_lines(&mut buf_file);
        }
        Err(e) => println!("Error found in tmp.s: {}", e),
    }
    println!("{}", content);
}
