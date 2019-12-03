extern crate colored;
use super::super::super::ce::types::Error;
use colored::*;
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
        for (idx, shdr) in self.shdrs.iter_mut().enumerate() {
            shdr.sh_offset += offset;
            offset += shdr.sh_size;
            shdr.sh_size = self.sections[idx].len() as u64;
        }
        let mut start: u64 = 0x40;
        if self.ehdr.e_type == ET_EXEC {
            start = 0x1000;
        }
        self.ehdr.e_shoff = self
            .sections
            .iter()
            .fold(start, |sum, sec| sum + sec.len() as u64);
        self.ehdr.e_shnum = self.sections.len() as u16;
        self.ehdr.e_shstrndx = self.ehdr.e_shnum - 1;
        let name_count = self.sections[self.ehdr.e_shstrndx as usize]
            .iter()
            .filter(|num| *num == &0x00)
            .collect::<Vec<&u8>>()
            .len()
            - 1;
        let mut sh_name = 1;
        for (idx, bb) in self.sections[self.ehdr.e_shstrndx as usize]
            .to_vec()
            .splitn(name_count, |num| *num == 0x00)
            .enumerate()
        {
            if idx == 0 || idx >= self.ehdr.e_shnum as usize {
                continue;
            }
            let b: Vec<&u8> = bb
                .iter()
                .take_while(|num| *num != &0x00)
                .collect::<Vec<&u8>>();
            self.shdrs[idx].sh_name = sh_name as u32;
            sh_name += b.len() as u32 + 1;
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
    pub fn init() -> Self {
        Self {
            ehdr: init_ehdr(),
            sections: vec![],
            shdrs: vec![],
            phdrs: None,
            names: HashMap::new(),
        }
    }
}

/* EI_CLASS */
static EI_CLASS: usize = 4;
static ELFCLASSNONE: EIDENT = 0;
static ELFCLASS32: EIDENT = 1;
static ELFCLASS64: EIDENT = 2;
static ELFCLASSNUM: EIDENT = 3;

/* EI_DATA */
static EI_DATA: usize = 5;
static ELFDATANONE: EIDENT = 0; /* Invalid data encoding */
static ELFDATA2LSB: EIDENT = 1; /* 2's complement, little endian */
static ELFDATA2MSB: EIDENT = 2; /* 2's complement, big endian */
static ELFDATANUM: EIDENT = 3;

/* EI_VERSION */
static EI_VERSION: usize = 6;
static EV_CURRENT: EIDENT = 1;

/* EI_OSABI */
static EI_OSABI: usize = 7;

/* EI_OSABIVERSION */
static EI_OSABIVERSION: usize = 8;

static ELFOSABI_NONE: EIDENT = 0; /* UNIX System V ABi */
static ELFOSABI_SYSV: EIDENT = 0; /* Alias */
static ELFOSABI_HPUX: EIDENT = 1; /* HP-UX */
static ELFOSABI_NETBSD: EIDENT = 2; /* NetBSD */
static ELFOSABI_GNU: EIDENT = 3; /* NetBSD */
static ELFOSABI_LINUX: EIDENT = ELFOSABI_GNU; /* Compatibility alias */
static ELFOSABI_SOLARIS: EIDENT = 6; /* Sun Solaris */
static ELFOSABI_AIX: EIDENT = 7; /* IBM AIX. */
static ELFOSABI_IRIX: EIDENT = 8; /* SGI Irix. */
static ELFOSABI_FREEBSD: EIDENT = 9; /* FreeBSD. */
static ELFOSABI_TRU64: EIDENT = 10; /* Compaq TRU64 UNIX. */
static ELFOSABI_MODESTO: EIDENT = 11; /* Compaq TRU64 UNIX. */
static ELFOSABI_OPENBSD: EIDENT = 12; /* OpenBSD. */
static ELFOSABI_ARM_AEABI: EIDENT = 64; /* ARM EABI */
static ELFOSABI_ARM: EIDENT = 97; /* ARM */
static ELFOSABI_STANDALONE: EIDENT = 255; /* Standalone (embedded) application */

/* ELF File Type */
static ET_NONE: Elf64Half = 0;
pub static ET_REL: Elf64Half = 1;
pub static ET_EXEC: Elf64Half = 2;
static ET_DYN: Elf64Half = 3;
static ET_CORE: Elf64Half = 4;
static ET_LOOS: Elf64Half = 0xfe00;
static ET_HIOS: Elf64Half = 0xfeff;
static ET_LOPROC: Elf64Half = 0xff00;
static ET_HIPROC: Elf64Half = 0xffff;

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
    pub fn new_unsafe(binary: Vec<u8>) -> Self {
        unsafe { std::ptr::read(binary.as_ptr() as *const Ehdr) }
    }
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
    pub fn print_to_stdout(&self) {
        println!("{}", "ELF Header:".bold().green());
        println!("  Class: {}", self.get_elf_class());
        println!("  Data: {}", self.get_elf_data());
        println!("  Version: {}", self.get_elf_version());
        println!("  OS/ABI: {}", self.get_elf_osabi());
        println!("  ABI Version: {}", self.get_elf_osabi_version());
        println!("  Type: {}", self.get_file_type());
    }
    fn get_elf_class(&self) -> String {
        let ei_class = self.e_ident.to_le_bytes()[EI_CLASS] as u128;
        let check_class = |const_class| ei_class == const_class;
        return if check_class(ELFCLASSNONE) {
            "None".to_string()
        } else if check_class(ELFCLASS32) {
            "ELF32".to_string()
        } else if check_class(ELFCLASS64) {
            "ELF64".to_string()
        } else if check_class(ELFCLASSNUM) {
            "ELFNUM".to_string()
        } else {
            "Invalid".to_string()
        };
    }
    fn get_elf_data(&self) -> String {
        let ei_data = self.e_ident.to_le_bytes()[EI_DATA] as u128;
        let check_data = |const_data| ei_data == const_data;
        return if check_data(ELFDATANONE) {
            "Invalid Data Encoding".to_string()
        } else if check_data(ELFDATA2LSB) {
            "2's complement, little endian".to_string()
        } else if check_data(ELFDATA2MSB) {
            "2's complement, big endian".to_string()
        } else if check_data(ELFDATANUM) {
            "ELFNUM".to_string()
        } else {
            "Invalid".to_string()
        };
    }
    fn get_elf_version(&self) -> String {
        let ei_version = self.e_ident.to_le_bytes()[EI_VERSION] as u128;
        if ei_version != EV_CURRENT {
            panic!("EI_VERSION must be EV_CURRENT");
        }
        "(current)".to_string()
    }
    fn get_elf_osabi(&self) -> String {
        let ei_osabi = self.e_ident.to_le_bytes()[EI_OSABI] as u128;
        let check_osabi = |const_osabi| ei_osabi == const_osabi;
        return if check_osabi(ELFOSABI_NONE) || check_osabi(ELFOSABI_SYSV) {
            "UNIX - System V".to_string()
        } else if check_osabi(ELFOSABI_HPUX) {
            "UNIX - HP-UX".to_string()
        } else if check_osabi(ELFOSABI_NETBSD) {
            "UNIX - NetBSD".to_string()
        } else if check_osabi(ELFOSABI_LINUX) {
            "UNIX - Linux".to_string()
        } else if check_osabi(ELFOSABI_SOLARIS) {
            "UNIX - Solaris".to_string()
        } else if check_osabi(ELFOSABI_AIX) {
            "UNIX - AIX".to_string()
        } else if check_osabi(ELFOSABI_IRIX) {
            "UNIX - IRIX".to_string()
        } else if check_osabi(ELFOSABI_FREEBSD) {
            "UNIX - FreeBSD".to_string()
        } else if check_osabi(ELFOSABI_TRU64) {
            "UNIX - TRU64".to_string()
        } else if check_osabi(ELFOSABI_MODESTO) {
            "Novell - Modesto".to_string()
        } else if check_osabi(ELFOSABI_OPENBSD) {
            "UNIX - OpenBSD".to_string()
        } else if check_osabi(ELFOSABI_ARM) {
            "ARM".to_string()
        } else if check_osabi(ELFOSABI_ARM_AEABI) {
            "ARM EABI".to_string()
        } else if check_osabi(ELFOSABI_STANDALONE) {
            "Standalone Application".to_string()
        } else {
            "Invalid".to_string()
        };
    }
    fn get_elf_osabi_version(&self) -> String {
        let ei_osabi_version = self.e_ident.to_le_bytes()[EI_OSABIVERSION] as u128;
        if ei_osabi_version != 0 {
            panic!("EI_VERSION must be 0");
        }
        "0".to_string()
    }

    fn get_file_type(&self) -> String {
        let check_file_type = |const_type| self.e_type == const_type;
        return if check_file_type(ET_NONE) {
            "NONE (None)".to_string()
        } else if check_file_type(ET_REL) {
            "REL (Relocation file)".to_string()
        } else if check_file_type(ET_EXEC) {
            "EXEC (Executable file)".to_string()
        } else if check_file_type(ET_DYN) {
            "DYN (Shared object file)".to_string()
        } else if check_file_type(ET_CORE) {
            "CORE (Core file)".to_string()
        } else if ET_LOOS <= self.e_type && self.e_type <= ET_HIOS {
            format!("OS Specific: ({:x})", self.e_type)
        } else if ET_LOPROC <= self.e_type && self.e_type <= ET_HIPROC {
            format!("Processor Specific: ({:x})", self.e_type)
        } else {
            "Invalid".to_string()
        };
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

#[derive(Clone)]
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
    pub fn new_unsafe(binary: Vec<u8>) -> Self {
        unsafe { std::ptr::read(binary.as_ptr() as *const Shdr) }
    }
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
#[derive(Clone)]
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
