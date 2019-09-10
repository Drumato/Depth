extern crate colored;

type Elf64Half = u16;
type Elf64Word = u32;
//type Elf64SWord = i32;
type Elf64Xword = u64;
//type Elf64Sxword = i64;
type Elf64Addr = u64;
type Elf64Off = u64;
type Elf64Section = u16;
type EIDENT = u128;
pub struct ELF {
    pub ehdr: Ehdr,
    pub shdrs: Vec<Shdr>,
    pub sections: Vec<Vec<u8>>,
    pub phdrs: Option<Vec<Phdr>>,
}

impl ELF {
    pub fn to_vec(&self) -> Vec<u8> {
        let mut bb: Vec<u8> = Vec::new();
        for b in self.ehdr.to_vec() {
            bb.push(b);
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
        self.ehdr.e_shnum = self.sections.len() as u16;
    }
}
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
}

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
}
#[repr(C)]
pub struct Phdr {
    pub p_type: Elf64Word,
    pub p_flags: Elf64Word,
    pub p_offset: Elf64Off,
    pub p_vaddr: Elf64Addr,
    pub p_paddr: Elf64Addr,
    pub p_filesz: Elf64Word,
    pub p_memsz: Elf64Xword,
    pub p_align: Elf64Xword,
}

pub static ET_REL: Elf64Half = 1;

pub static SHT_PROGBITS: Elf64Word = 1;
pub static SHT_SYMTAB: Elf64Word = 2;
pub static SHT_STRTAB: Elf64Word = 3;

pub static SHF_ALLOC: Elf64Xword = 1 << 1;
pub static SHF_EXECINSTR: Elf64Xword = 1 << 2;
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
        e_ehsize: 0x40,
        e_phentsize: 0,
        e_phnum: 0,
        e_shentsize: 0x40,
        e_shnum: 0,
        e_shstrndx: 3,
    }
}
pub fn init_mainhdr(size: u64) -> Shdr {
    Shdr {
        sh_name: 1,
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
        sh_name: 7,
        sh_type: SHT_SYMTAB,
        sh_flags: 0,
        sh_addr: 0,
        sh_offset: 0,
        sh_size: size,
        sh_link: 2,
        sh_info: 0,
        sh_addralign: 8,
        sh_entsize: 24,
    }
}
pub fn init_strtabhdr(size: u64) -> Shdr {
    Shdr {
        sh_name: 15,
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
pub fn init_shstrtabhdr(size: u64) -> Shdr {
    Shdr {
        sh_name: 23,
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
pub fn init_mainsym(size: u64) -> Symbol {
    Symbol {
        st_name: 1,
        st_info: STB_GLOBAL << 4,
        st_other: 0,
        st_shndx: 1,
        st_value: 0,
        st_size: size,
    }
}
