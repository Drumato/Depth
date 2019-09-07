extern crate colored;

type Elf64Half = u16;
type Elf64Word = u32;
//type Elf64SWord = i32;
type Elf64Xword = u64;
//type Elf64Sxword = i64;
type Elf64Addr = u64;
type Elf64Off = u64;
//type Elf64Section = u16;
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
pub static SHF_ALLOC: Elf64Xword = 1 << 1;
pub static SHF_EXECINSTR: Elf64Xword = 1 << 2;
