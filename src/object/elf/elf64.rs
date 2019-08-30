extern crate colored;
use super::super::super::compile::ce::types::Error;
use colored::*;
type Elf64Half = u16;
type Elf64Word = u32;
//type Elf64SWord = i32;
type Elf64Xword = u64;
//type Elf64Sxword = i64;
type Elf64Addr = u64;
type Elf64Off = u64;
//type Elf64Section = u16;
type EIDENT = u128;
struct ELF {
    ehdr: Ehdr,
    shdrs: Vec<Shdr>,
}

impl ELF {
    fn new(binary: Vec<u8>) -> ELF {
        let ehdr: Ehdr = Ehdr::new_unsafe(binary[0..64].to_vec());
        let shdrs: Vec<Shdr> = ELF::build_shdrs(&ehdr, binary[ehdr.e_shoff as usize..].to_vec());
        ELF {
            ehdr: ehdr,
            shdrs: shdrs,
        }
    }
    fn build_shdrs(ehdr: &Ehdr, binary: Vec<u8>) -> Vec<Shdr> {
        let mut shdrs: Vec<Shdr> = Vec::new();
        for i in 0..ehdr.e_shnum {
            shdrs.push(Shdr::new_unsafe(
                binary[(i * ehdr.e_shentsize) as usize..].to_vec(),
            ));
        }
        shdrs
    }
    fn dump(&self) {
        self.ehdr.dump();
        for shdr in self.shdrs.iter() {
            shdr.dump();
        }
    }
}
#[repr(C)]
#[derive(Debug)]
struct Ehdr {
    e_ident: EIDENT,
    e_type: Elf64Half,
    e_machine: Elf64Half,
    e_version: Elf64Word,
    e_entry: Elf64Addr,
    e_phoff: Elf64Off,
    e_shoff: Elf64Off,
    e_flags: Elf64Word,
    e_ehsize: Elf64Half,
    e_phentsize: Elf64Half,
    e_phnum: Elf64Half,
    e_shentsize: Elf64Half,
    e_shnum: Elf64Half,
    e_shstrndx: Elf64Half,
}
impl Ehdr {
    fn new_unsafe(binary: Vec<u8>) -> Ehdr {
        if binary.len() < 0x40 {
            Error::ELF.found(&format!("not enough bytes: got {:x}", binary.len()));
        }
        unsafe { std::ptr::read(binary.as_ptr() as *const Ehdr) }
    }
    fn dump(&self) {
        eprintln!("{}", "--------dump EHDR--------".green().bold());
        eprintln!("Magicnumber(Little Endian) -> {:x}", self.e_ident);
        eprintln!("Type -> {}", self.e_type);
        eprintln!("Machine -> 0x{:x}", self.e_machine);
        eprintln!("Version -> {}", self.e_version);
        eprintln!("Entrypoint -> 0x{:x}", self.e_entry);
        eprintln!("Program Header Table Offset -> 0x{:x}", self.e_phoff);
        eprintln!("Section Header Table Offset -> 0x{:x}", self.e_shoff);
        eprintln!("Flags -> {:b}", self.e_flags);
        eprintln!("ELF-Header Size -> {}(bytes)", self.e_ehsize);
        eprintln!("Program-Header Size -> {}(bytes)", self.e_phentsize);
        eprintln!("Program-Header Number -> {}", self.e_phnum);
        eprintln!("Section-Header Size -> {}(bytes)", self.e_shentsize);
        eprintln!("Section-Header Number -> {}", self.e_shnum);
        eprintln!(".shstrtab Index -> {}", self.e_shstrndx);
    }
}

#[repr(C)]
#[derive(Debug)]
struct Shdr {
    sh_name: Elf64Word,
    sh_type: Elf64Word,
    sh_flags: Elf64Xword,
    sh_addr: Elf64Addr,
    sh_offset: Elf64Off,
    sh_size: Elf64Xword,
    sh_link: Elf64Word,
    sh_info: Elf64Word,
    sh_addralign: Elf64Xword,
    sh_entsize: Elf64Xword,
}

impl Shdr {
    fn new_unsafe(binary: Vec<u8>) -> Shdr {
        if binary.len() < 0x40 {
            Error::ELF.found(&format!("not enough bytes: got {:x}", binary.len()));
        }
        unsafe { std::ptr::read(binary.as_ptr() as *const Shdr) }
    }
    fn dump(&self) {}
}

pub fn dump_elf_in_detail(binary: Vec<u8>) {
    let elf_file: ELF = ELF::new(binary);
    elf_file.dump();
}
