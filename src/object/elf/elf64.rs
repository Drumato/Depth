extern crate colored;
use super::super::super::ce::types::Error;
use std::collections::HashMap;

type Elf64Half = u16;
type Elf64Word = u32;
//type Elf64SWord = i32;
type Elf64Xword = u64;
type Elf64Sxword = i64;
type Elf64Addr = u64;
type Elf64Off = u64;
type Elf64Section = u16;
type EIDENT = u128;
pub struct ELF {
    pub ehdr: Ehdr,
    pub shdrs: Vec<Shdr>,
    pub sections: Vec<Vec<u8>>,
    pub phdrs: Option<Vec<Phdr>>,
    pub names: HashMap<String, usize>,
}

impl ELF {
    pub fn to_vec(&self) -> Vec<u8> {
        let mut bb: Vec<u8> = Vec::new();
        for b in self.ehdr.to_vec() {
            bb.push(b);
        }
        if let Some(phdrs) = &self.phdrs {
            for shdr in phdrs.iter() {
                for b in shdr.to_vec() {
                    bb.push(b);
                }
            }
        }
        for sec in self.sections.iter() {
            for b in sec.to_vec() {
                bb.push(b);
            }
        }
        for shdr in self.shdrs.iter() {
            for b in shdr.to_vec() {
                bb.push(b);
            }
        }
        bb
    }
    pub fn add_section(&mut self, section: Vec<u8>, shdr: Shdr, name: &str) {
        self.names.insert(name.to_string(), self.sections.len());
        self.sections.push(section);
        self.shdrs.push(shdr);
    }
    pub fn condition(&mut self) {
        let mut offset = 0x40;
        for shdr in self.shdrs.iter_mut() {
            shdr.sh_offset += offset;
            offset += shdr.sh_size;
        }
        self.ehdr.e_shoff = self
            .sections
            .iter()
            .fold(0x40, |sum, sec| sum + sec.len() as u64);
        self.ehdr.e_shnum = self.sections.len() as u16; // 1 -> nullhdr
        self.ehdr.e_shstrndx = self.ehdr.e_shnum - 1;
        let name_count = self.sections[self.ehdr.e_shstrndx as usize]
            .iter()
            .filter(|num| *num == &('.' as u8))
            .collect::<Vec<&u8>>()
            .len();
        let mut sh_name = 1;
        for (idx, bb) in self.sections[self.ehdr.e_shstrndx as usize][1..]
            .to_vec()
            .splitn(name_count, |num| *num == 0x00)
            .enumerate()
        {
            let b: Vec<&u8> = bb.iter().filter(|num| *num != &0x00).collect::<Vec<&u8>>();
            self.shdrs[idx + 1].sh_name = sh_name as u32;
            sh_name += (b.len() + 1) as u32;
        }
    }
    pub fn get_section_number(&self, name: &str) -> usize {
        if let Some(number) = self.names.get(name) {
            return *number;
        } else {
            Error::ELF.found(&format!("not found such an section -> {}", name));
            00
        }
    }
}
pub static ET_REL: Elf64Half = 1;
pub static ET_EXEC: Elf64Half = 2;
#[repr(C)]
pub struct Ehdr {
    pub e_ident: EIDENT,
    pub e_type: Elf64Half,
    pub e_machine: Elf64Half,
    pub e_version: Elf64Word,
    pub e_entry: Elf64Addr,
    pub e_phoff: Elf64Off,
    pub e_shoff: Elf64Off,
    pub e_flags: Elf64Word,
    pub e_ehsize: Elf64Half,
    pub e_phentsize: Elf64Half,
    pub e_phnum: Elf64Half,
    pub e_shentsize: Elf64Half,
    pub e_shnum: Elf64Half,
    pub e_shstrndx: Elf64Half,
}
impl Ehdr {
    pub fn to_vec(&self) -> Vec<u8> {
        let mut bb: Vec<u8> = Vec::new();
        for b in self.e_ident.to_be_bytes().to_vec() {
            bb.push(b);
        }
        for b in self.e_type.to_le_bytes().to_vec() {
            bb.push(b);
        }
        for b in self.e_machine.to_le_bytes().to_vec() {
            bb.push(b);
        }
        for b in self.e_version.to_le_bytes().to_vec() {
            bb.push(b);
        }
        for b in self.e_entry.to_le_bytes().to_vec() {
            bb.push(b);
        }
        for b in self.e_phoff.to_le_bytes().to_vec() {
            bb.push(b);
        }
        for b in self.e_shoff.to_le_bytes().to_vec() {
            bb.push(b);
        }
        for b in self.e_flags.to_le_bytes().to_vec() {
            bb.push(b);
        }
        for b in self.e_ehsize.to_le_bytes().to_vec() {
            bb.push(b);
        }
        for b in self.e_phentsize.to_le_bytes().to_vec() {
            bb.push(b);
        }
        for b in self.e_phnum.to_le_bytes().to_vec() {
            bb.push(b);
        }
        for b in self.e_shentsize.to_le_bytes().to_vec() {
            bb.push(b);
        }
        for b in self.e_shnum.to_le_bytes().to_vec() {
            bb.push(b);
        }
        for b in self.e_shstrndx.to_le_bytes().to_vec() {
            bb.push(b);
        }
        bb
    }
    pub fn size() -> usize {
        0x40
    }
}

pub fn init_ehdr() -> Ehdr {
    Ehdr {
        e_ident: 0x7f454c46020101000000000000000000,
        e_type: ET_REL,
        e_machine: 0x3e,
        e_version: 1,
        e_entry: 0,
        e_phoff: 0,
        e_shoff: 0,
        e_flags: 0,
        e_ehsize: Ehdr::size() as u16,
        e_phentsize: 0,
        e_phnum: 0,
        e_shentsize: Shdr::size() as u16,
        e_shnum: 0,
        e_shstrndx: 0,
    }
}
pub static SHT_PROGBITS: Elf64Word = 1;
pub static SHT_SYMTAB: Elf64Word = 2;
pub static SHT_STRTAB: Elf64Word = 3;
pub static SHT_RELA: Elf64Word = 4;

pub static SHF_ALLOC: Elf64Xword = 1 << 1;
pub static SHF_EXECINSTR: Elf64Xword = 1 << 2;
pub static SHF_INFO_LINK: Elf64Xword = 1 << 6;

#[repr(C)]
pub struct Shdr {
    pub sh_name: Elf64Word,
    pub sh_type: Elf64Word,
    pub sh_flags: Elf64Xword,
    pub sh_addr: Elf64Addr,
    pub sh_offset: Elf64Off,
    pub sh_size: Elf64Xword,
    pub sh_link: Elf64Word,
    pub sh_info: Elf64Word,
    pub sh_addralign: Elf64Xword,
    pub sh_entsize: Elf64Xword,
}

impl Shdr {
    pub fn to_vec(&self) -> Vec<u8> {
        let mut bb: Vec<u8> = Vec::new();
        for b in self.sh_name.to_le_bytes().to_vec() {
            bb.push(b);
        }
        for b in self.sh_type.to_le_bytes().to_vec() {
            bb.push(b);
        }
        for b in self.sh_flags.to_le_bytes().to_vec() {
            bb.push(b);
        }
        for b in self.sh_addr.to_le_bytes().to_vec() {
            bb.push(b);
        }
        for b in self.sh_offset.to_le_bytes().to_vec() {
            bb.push(b);
        }
        for b in self.sh_size.to_le_bytes().to_vec() {
            bb.push(b);
        }
        for b in self.sh_link.to_le_bytes().to_vec() {
            bb.push(b);
        }
        for b in self.sh_info.to_le_bytes().to_vec() {
            bb.push(b);
        }
        for b in self.sh_addralign.to_le_bytes().to_vec() {
            bb.push(b);
        }
        for b in self.sh_entsize.to_le_bytes().to_vec() {
            bb.push(b);
        }
        bb
    }
    pub fn size() -> usize {
        0x40
    }
}
pub fn init_texthdr(size: u64) -> Shdr {
    Shdr {
        sh_name: 0,
        sh_type: SHT_PROGBITS,
        sh_flags: SHF_ALLOC | SHF_EXECINSTR,
        sh_addr: 0,
        sh_offset: 0,
        sh_size: size,
        sh_link: 0,
        sh_info: 0,
        sh_addralign: 1,
        sh_entsize: 0,
    }
}
pub fn init_symtabhdr(size: u64) -> Shdr {
    Shdr {
        sh_name: 0,
        sh_type: SHT_SYMTAB,
        sh_flags: 0,
        sh_addr: 0,
        sh_offset: 0,
        sh_size: size,
        sh_link: 3,
        sh_info: 1,
        sh_addralign: 8,
        sh_entsize: Symbol::size() as u64,
    }
}
pub fn init_strtabhdr(size: u64) -> Shdr {
    Shdr {
        sh_name: 0,
        sh_type: SHT_STRTAB,
        sh_flags: 0,
        sh_addr: 0,
        sh_offset: 0,
        sh_size: size,
        sh_link: 0,
        sh_info: 0,
        sh_addralign: 1,
        sh_entsize: 0,
    }
}

pub fn init_relahdr(size: u64) -> Shdr {
    Shdr {
        sh_name: 0,
        sh_type: SHT_RELA,
        sh_flags: SHF_INFO_LINK,
        sh_addr: 0,
        sh_offset: 0,
        sh_size: size,
        sh_link: 2,
        sh_info: 1,
        sh_addralign: 8,
        sh_entsize: Rela::size() as u64,
    }
}
pub fn init_nullhdr() -> Shdr {
    Shdr {
        sh_name: 0,
        sh_type: 0,
        sh_flags: 0,
        sh_addr: 0,
        sh_offset: 0,
        sh_size: 0,
        sh_link: 0,
        sh_info: 0,
        sh_addralign: 0,
        sh_entsize: 0,
    }
}
pub static PT_LOAD: Elf64Word = 1;
pub static PF_X: Elf64Word = 1 << 0;
pub static PF_W: Elf64Word = 1 << 1;
pub static PF_R: Elf64Word = 1 << 2;
#[repr(C)]
pub struct Phdr {
    pub p_type: Elf64Word,
    pub p_flags: Elf64Word,
    pub p_offset: Elf64Off,
    pub p_vaddr: Elf64Addr,
    pub p_paddr: Elf64Addr,
    pub p_filesz: Elf64Xword,
    pub p_memsz: Elf64Xword,
    pub p_align: Elf64Xword,
}

impl Phdr {
    pub fn to_vec(&self) -> Vec<u8> {
        let mut bb: Vec<u8> = Vec::new();
        for b in self.p_type.to_le_bytes().to_vec() {
            bb.push(b);
        }
        for b in self.p_flags.to_le_bytes().to_vec() {
            bb.push(b);
        }
        for b in self.p_offset.to_le_bytes().to_vec() {
            bb.push(b);
        }
        for b in self.p_vaddr.to_le_bytes().to_vec() {
            bb.push(b);
        }
        for b in self.p_paddr.to_le_bytes().to_vec() {
            bb.push(b);
        }
        for b in self.p_filesz.to_le_bytes().to_vec() {
            bb.push(b);
        }
        for b in self.p_memsz.to_le_bytes().to_vec() {
            bb.push(b);
        }
        for b in self.p_align.to_le_bytes().to_vec() {
            bb.push(b);
        }
        bb
    }
    pub fn size() -> usize {
        56
    }
}

pub fn init_phdr() -> Phdr {
    Phdr {
        p_type: 0,
        p_flags: 0,
        p_offset: 0,
        p_vaddr: 0,
        p_paddr: 0,
        p_filesz: 0,
        p_memsz: 0,
        p_align: 0,
    }
}

pub fn strtab(names: Vec<&str>) -> Vec<u8> {
    let mut b: Vec<u8> = Vec::new();
    b.push(0x00);
    for name in names {
        for byte in name.as_bytes() {
            b.push(*byte);
        }
        b.push(0x00);
    }
    let md = b.len() % 4;
    for _ in 0..(4 - md) {
        b.push(0x00);
    }
    b
}
pub static STB_GLOBAL: u8 = 1;
//pub static STB_LOCAL: u8 = 0;
pub static STT_FUNC: u8 = 2;
#[repr(C)]
pub struct Symbol {
    pub st_name: Elf64Word,
    pub st_info: u8,
    pub st_other: u8,
    pub st_shndx: Elf64Section,
    pub st_value: Elf64Addr,
    pub st_size: Elf64Xword,
}
impl Symbol {
    pub fn size() -> usize {
        24
    }
    pub fn new_unsafe(binary: Vec<u8>) -> Symbol {
        unsafe { std::ptr::read(binary.as_ptr() as *const Symbol) }
    }
    pub fn to_vec(&self) -> Vec<u8> {
        let mut bb: Vec<u8> = Vec::new();
        for b in self.st_name.to_le_bytes().to_vec() {
            bb.push(b);
        }
        for b in self.st_info.to_le_bytes().to_vec() {
            bb.push(b);
        }
        for b in self.st_other.to_le_bytes().to_vec() {
            bb.push(b);
        }
        for b in self.st_shndx.to_le_bytes().to_vec() {
            bb.push(b);
        }
        for b in self.st_value.to_le_bytes().to_vec() {
            bb.push(b);
        }
        for b in self.st_size.to_le_bytes().to_vec() {
            bb.push(b);
        }
        bb
    }
}
pub fn symbols_to_vec(symbols: Vec<Symbol>) -> Vec<u8> {
    let mut bb: Vec<u8> = Vec::new();
    for sym in symbols {
        for b in sym.to_vec() {
            bb.push(b);
        }
    }
    bb
}
pub fn init_sym(name: Elf64Word, bind: u8, size: u64, value: u64) -> Symbol {
    Symbol {
        st_name: name,
        st_info: (bind << 4) + STT_FUNC,
        st_other: 0,
        st_shndx: 1,
        st_value: value,
        st_size: size,
    }
}
pub fn init_refsym(name: Elf64Word, bind: u8) -> Symbol {
    Symbol {
        st_name: name,
        st_info: (bind << 4) + STT_FUNC,
        st_other: 0,
        st_shndx: 0,
        st_value: 0,
        st_size: 0,
    }
}
pub fn init_nullsym() -> Symbol {
    Symbol {
        st_name: 0,
        st_info: 0,
        st_other: 0,
        st_shndx: 0,
        st_value: 0,
        st_size: 0,
    }
}

#[derive(Debug)]
pub struct Rela {
    pub r_offset: Elf64Addr,
    pub r_info: Elf64Xword,
    pub r_addend: Elf64Sxword,
}
impl Rela {
    pub fn size() -> usize {
        24
    }
    pub fn bind(info: u64) -> usize {
        info as usize >> 32
    }
    pub fn new_unsafe(binary: Vec<u8>) -> Rela {
        unsafe { std::ptr::read(binary.as_ptr() as *const Rela) }
    }
    pub fn to_vec(&self) -> Vec<u8> {
        let mut bb: Vec<u8> = Vec::new();
        for b in self.r_offset.to_le_bytes().to_vec() {
            bb.push(b);
        }
        for b in self.r_info.to_le_bytes().to_vec() {
            bb.push(b);
        }
        for b in self.r_addend.to_le_bytes().to_vec() {
            bb.push(b);
        }
        bb
    }
    pub fn new() -> Rela {
        Rela {
            r_offset: 0,
            r_info: 0,
            r_addend: -4,
        }
    }
}
pub fn relas_to_vec(relas: Vec<&Rela>) -> Vec<u8> {
    let mut bb: Vec<u8> = Vec::new();
    for rela in relas {
        for b in rela.to_vec() {
            bb.push(b);
        }
    }
    bb
}
