extern crate colored;
use super::super::super::ce::types::Error;
use super::super::debug;
use colored::*;
use debug::DebugSymbol;
extern crate cli_table;
use cli_table::{Cell, Row};

use std::collections::BTreeMap;

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
    pub names: BTreeMap<String, usize>,
}

impl ELF {
    pub fn to_vec(&self) -> Vec<u8> {
        let mut bb: Vec<u8> = Vec::new();
        for b in self.ehdr.to_vec() {
            bb.push(b);
        }
        if let Some(phdrs) = &self.phdrs {
            for phdr in phdrs.iter() {
                for b in phdr.to_vec() {
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
    pub fn check_whether_given_section_is_exist(&self, name: &str) -> bool {
        self.names.contains_key(name)
    }
    pub fn init() -> Self {
        Self {
            ehdr: init_ehdr(),
            sections: vec![],
            shdrs: vec![],
            phdrs: None,
            names: BTreeMap::new(),
        }
    }
    pub fn program_header_columns() -> Row {
        let mut cells: Vec<Cell> = Vec::new();
        Self::add_cell(&mut cells, &format!("{}", "Type".bold().green()));
        Self::add_cell(&mut cells, &format!("{}", "Offset".bold().green()));
        Self::add_cell(&mut cells, &format!("{}", "VirtAddr".bold().green()));
        Self::add_cell(&mut cells, &format!("{}", "PhysAddr".bold().green()));
        Self::add_cell(&mut cells, &format!("{}", "FileSiz".bold().green()));
        Self::add_cell(&mut cells, &format!("{}", "MemSiz".bold().green()));
        Self::add_cell(&mut cells, &format!("{}", "Flags".bold().green()));
        Self::add_cell(&mut cells, &format!("{}", "Align".bold().green()));
        Row::new(cells)
    }
    pub fn section_header_columns() -> Row {
        let mut cells: Vec<Cell> = Vec::new();
        Self::add_cell(&mut cells, &format!("{}", "Name".bold().green()));
        Self::add_cell(&mut cells, &format!("{}", "Type".bold().green()));
        Self::add_cell(&mut cells, &format!("{}", "Flags".bold().green()));
        Self::add_cell(&mut cells, &format!("{}", "Address".bold().green()));
        Self::add_cell(&mut cells, &format!("{}", "Offset".bold().green()));
        Self::add_cell(&mut cells, &format!("{}", "Size".bold().green()));
        Self::add_cell(&mut cells, &format!("{}", "EntSize".bold().green()));
        Self::add_cell(&mut cells, &format!("{}", "Link".bold().green()));
        Self::add_cell(&mut cells, &format!("{}", "Info".bold().green()));
        Self::add_cell(&mut cells, &format!("{}", "Align".bold().green()));
        Row::new(cells)
    }
    pub fn symbol_table_columns() -> Row {
        let mut cells: Vec<Cell> = Vec::new();
        Self::add_cell(&mut cells, &format!("{}", "Value".bold().green()));
        Self::add_cell(&mut cells, &format!("{}", "Size".bold().green()));
        Self::add_cell(&mut cells, &format!("{}", "Type".bold().green()));
        Self::add_cell(&mut cells, &format!("{}", "Bind".bold().green()));
        Self::add_cell(&mut cells, &format!("{}", "Vis".bold().green()));
        Self::add_cell(&mut cells, &format!("{}", "Ndx".bold().green()));
        Self::add_cell(&mut cells, &format!("{}", "Name".bold().green()));
        Row::new(cells)
    }
    pub fn relocation_table_columns() -> Row {
        let mut cells: Vec<Cell> = Vec::new();
        Self::add_cell(&mut cells, &format!("{}", "Offset".bold().green()));
        Self::add_cell(&mut cells, &format!("{}", "Info".bold().green()));
        Self::add_cell(&mut cells, &format!("{}", "Type".bold().green()));
        Self::add_cell(&mut cells, &format!("{}", "Sym. Value".bold().green()));
        Self::add_cell(
            &mut cells,
            &format!("{}", "Sym. Name + Addend".bold().green()),
        );
        Row::new(cells)
    }
    pub fn dynamic_table_columns() -> Row {
        let mut cells: Vec<Cell> = Vec::new();
        Self::add_cell(&mut cells, &format!("{}", "Tag".bold().green()));
        Self::add_cell(&mut cells, &format!("{}", "Type".bold().green()));
        Self::add_cell(&mut cells, &format!("{}", "Name / Value".bold().green()));
        Row::new(cells)
    }

    pub fn debug_table_columns() -> Row {
        let mut cells: Vec<Cell> = Vec::new();
        Self::add_cell(&mut cells, &format!("{}", "ReturnType".bold().green()));
        Self::add_cell(&mut cells, &format!("{}", "ArgNumbers".bold().green()));
        Self::add_cell(&mut cells, &format!("{}", "Name".bold().green()));
        Self::add_cell(&mut cells, &format!("{}", "ArgType".bold().green()));
        Row::new(cells)
    }
    pub fn documents_columns() -> Row {
        let mut cells: Vec<Cell> = Vec::new();
        Self::add_cell(&mut cells, &format!("{}", "Name".bold().green()));
        Self::add_cell(&mut cells, &format!("{}", "Documents".bold().green()));
        Row::new(cells)
    }
    pub fn add_cell(vec: &mut Vec<Cell>, contents: &String) {
        vec.push(Cell::new(contents, Default::default()));
    }
    fn specified_ptype_exist(&self, p_type: Elf64Word) -> Option<&Phdr> {
        if let Some(phdrs) = &self.phdrs {
            for phdr in phdrs.iter() {
                if phdr.p_type == p_type {
                    return Some(phdr);
                }
            }
        }
        None
    }
    pub fn check_execstack(&self) -> bool {
        if let Some(phdr) = self.specified_ptype_exist(PT_GNU_STACK) {
            (phdr.p_flags & PF_X) != 0
        } else {
            false
        }
    }
    pub fn specified_dtag_exist(&self, d_tag: Elf64Sxword) -> Option<Dyn> {
        for (i, shdr) in self.shdrs.iter().enumerate() {
            if shdr.sh_type as u64 == SHT_DYNAMIC {
                let dynamics_number = shdr.sh_size as usize / Dyn::size();
                let dyntab = self.sections[i].clone();
                for i in 0..dynamics_number as usize {
                    let dyn_binary = dyntab[i * Dyn::size()..(i + 1) * Dyn::size()].to_vec();
                    let dyn_sym = Dyn::new_unsafe(dyn_binary);
                    if dyn_sym.d_tag == d_tag {
                        return Some(dyn_sym);
                    }
                }
            }
        }

        None
    }
    pub fn check_relro(&self) -> RELRO {
        let mut relro = RELRO::DISABLE;
        if let Some(_phdr) = self.specified_ptype_exist(PT_GNU_RELRO) {
            relro = RELRO::PARTIAL;
        }

        if let Some(dyn_sym) = self.specified_dtag_exist(DT_FLAGS) {
            if dyn_sym.d_un == DF_BIND_NOW as u64 && relro == RELRO::PARTIAL {
                relro = RELRO::ENABLE;
            }
        }
        relro
    }
    pub fn check_pie(&self) -> PIE {
        let mut pie = PIE::DISABLE;
        if self.ehdr.e_type != ET_DYN {
            return pie;
        }
        if let Some(_dyn_sym) = self.specified_dtag_exist(DT_DEBUG) {
            pie = PIE::ENABLE;
        } else {
            pie = PIE::DSO;
        }
        pie
    }
    pub fn check_rpath(&self) -> bool {
        return if let Some(_dyn_sym) = self.specified_dtag_exist(DT_RPATH) {
            true
        } else {
            false
        };
    }
    pub fn check_runpath(&self) -> bool {
        return if let Some(_dyn_sym) = self.specified_dtag_exist(DT_RUNPATH) {
            true
        } else {
            false
        };
    }
}

#[derive(PartialEq)]
pub enum RELRO {
    ENABLE,
    PARTIAL,
    DISABLE,
}

#[derive(PartialEq)]
pub enum PIE {
    ENABLE,
    DSO,
    DISABLE,
}

/* EI_CLASS */
const EI_CLASS: usize = 4;
const ELFCLASSNONE: EIDENT = 0;
const ELFCLASS32: EIDENT = 1;
const ELFCLASS64: EIDENT = 2;
const ELFCLASSNUM: EIDENT = 3;

/* EI_DATA */
const EI_DATA: usize = 5;
const ELFDATANONE: EIDENT = 0; /* Invalid data encoding */
const ELFDATA2LSB: EIDENT = 1; /* 2's complement, little endian */
const ELFDATA2MSB: EIDENT = 2; /* 2's complement, big endian */
const ELFDATANUM: EIDENT = 3;

/* EI_VERSION */
const EI_VERSION: usize = 6;
const EV_CURRENT: EIDENT = 1;

/* EI_OSABI */
const EI_OSABI: usize = 7;

/* EI_OSABIVERSION */
const EI_OSABIVERSION: usize = 8;

const ELFOSABI_NONE: EIDENT = 0; /* UNIX System V ABi */
const ELFOSABI_SYSV: EIDENT = 0; /* Alias */
const ELFOSABI_HPUX: EIDENT = 1; /* HP-UX */
const ELFOSABI_NETBSD: EIDENT = 2; /* NetBSD */
const ELFOSABI_GNU: EIDENT = 3; /* NetBSD */
const ELFOSABI_LINUX: EIDENT = ELFOSABI_GNU; /* Compatibility alias */
const ELFOSABI_SOLARIS: EIDENT = 6; /* Sun Solaris */
const ELFOSABI_AIX: EIDENT = 7; /* IBM AIX. */
const ELFOSABI_IRIX: EIDENT = 8; /* SGI Irix. */
const ELFOSABI_FREEBSD: EIDENT = 9; /* FreeBSD. */
const ELFOSABI_TRU64: EIDENT = 10; /* Compaq TRU64 UNIX. */
const ELFOSABI_MODESTO: EIDENT = 11; /* Compaq TRU64 UNIX. */
const ELFOSABI_OPENBSD: EIDENT = 12; /* OpenBSD. */
const ELFOSABI_ARM_AEABI: EIDENT = 64; /* ARM EABI */
const ELFOSABI_ARM: EIDENT = 97; /* ARM */
const ELFOSABI_STANDALONE: EIDENT = 255; /* Standalone (embedded) application */

/* ELF File Type */
const ET_NONE: Elf64Half = 0;
pub const ET_REL: Elf64Half = 1;
pub const ET_EXEC: Elf64Half = 2;
const ET_DYN: Elf64Half = 3;
const ET_CORE: Elf64Half = 4;
const ET_LOOS: Elf64Half = 0xfe00;
const ET_HIOS: Elf64Half = 0xfeff;
const ET_LOPROC: Elf64Half = 0xff00;
const ET_HIPROC: Elf64Half = 0xffff;

/* Machine Architecture */
const EM_X86_64: Elf64Half = 0x3e;

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
        println!("  Machine: {}", self.get_machine_name());
        println!("  Version: 0x{:x}", self.e_version);
        println!("  Entry point address: 0x{:x}", self.e_entry);
        println!(
            "  Start of program headers: {} (bytes into file)",
            self.e_phoff
        );
        println!(
            "  Start of section headers: {} (bytes into file)",
            self.e_shoff
        );
        println!("  Flags: 0x{:x}", self.e_flags);
        println!("  Size of this header: {} (bytes)", self.e_ehsize);
        println!("  Size of program header: {} (bytes)", self.e_phentsize);
        println!("  Number of program header: {}", self.e_phnum);
        println!("  Size of section header: {} (bytes)", self.e_shentsize);
        println!("  Number of program header: {}", self.e_shnum);
        println!("  Section header string table index: {}", self.e_shstrndx);
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
    fn get_machine_name(&self) -> String {
        return if self.e_machine == EM_X86_64 {
            "Advanced Micro Devices X86-64".to_string()
        } else {
            format!("ERROR: not implement 0x{:x}", self.e_machine)
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
pub const SHT_NULL: Elf64Xword = 0;
pub const SHT_PROGBITS: Elf64Xword = 1;
pub const SHT_SYMTAB: Elf64Xword = 2;
pub const SHT_STRTAB: Elf64Xword = 3;
pub const SHT_RELA: Elf64Xword = 4;
pub const SHT_HASH: Elf64Xword = 5;
pub const SHT_DYNAMIC: Elf64Xword = 6;
pub const SHT_NOTE: Elf64Xword = 7;
pub const SHT_NOBITS: Elf64Xword = 8;
pub const SHT_REL: Elf64Xword = 9;
pub const SHT_SHLIB: Elf64Xword = 10;
pub const SHT_DYNSYM: Elf64Xword = 11;
pub const SHT_INIT_ARRAY: Elf64Xword = 14;
pub const SHT_FINI_ARRAY: Elf64Xword = 15;
pub const SHT_PREINIT_ARRAY: Elf64Xword = 16;
pub const SHT_GROUP: Elf64Xword = 17;
pub const SHT_SYMTAB_SHNDX: Elf64Xword = 18;
pub const SHT_GNU_HASH: Elf64Xword = 0x6ffffff6;
pub const SHT_GNU_VERDEF: Elf64Xword = 0x6ffffffd;
pub const SHT_GNU_VERNEED: Elf64Xword = 0x6ffffffe;
pub const SHT_GNU_VERSYM: Elf64Xword = 0x6fffffff;

pub const SHF_WRITE: Elf64Xword = 1 << 0;
pub const SHF_ALLOC: Elf64Xword = 1 << 1;
pub const SHF_EXECINSTR: Elf64Xword = 1 << 2;
pub const SHF_MERGE: Elf64Xword = 1 << 4;
pub const SHF_STRINGS: Elf64Xword = 1 << 5;
pub const SHF_INFO_LINK: Elf64Xword = 1 << 6;
pub const SHF_LINK_ORDER: Elf64Xword = 1 << 7;
pub const SHF_OS_NONCONFORMING: Elf64Xword = 1 << 8;
pub const SHF_GROUP: Elf64Xword = 1 << 9;
pub const SHF_TLS: Elf64Xword = 1 << 10;
pub const SHF_COMPRESSED: Elf64Xword = 1 << 11;
pub const SHF_EXCLUDE: Elf64Xword = 1 << 31;

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
    pub fn to_stdout(&self, elf_file: &ELF) -> Row {
        let mut cells: Vec<Cell> = Vec::new();
        ELF::add_cell(&mut cells, &self.get_name(elf_file));
        ELF::add_cell(&mut cells, &self.get_type());
        ELF::add_cell(&mut cells, &self.get_flags());
        ELF::add_cell(&mut cells, &format!("0x{:x}", self.sh_addr));
        ELF::add_cell(&mut cells, &format!("0x{:x}", self.sh_offset));
        ELF::add_cell(&mut cells, &format!("{}", self.sh_size));
        ELF::add_cell(&mut cells, &format!("{}", self.sh_entsize));
        ELF::add_cell(&mut cells, &format!("{}", self.sh_link));
        ELF::add_cell(&mut cells, &format!("{}", self.sh_info));
        ELF::add_cell(&mut cells, &format!("{}", self.sh_addralign));
        Row::new(cells)
    }
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
    fn get_name(&self, elf_file: &ELF) -> String {
        let shstrtab = elf_file.sections[elf_file.ehdr.e_shstrndx as usize].clone();
        let mut section_name = ELF::collect_name(shstrtab[self.sh_name as usize..].to_vec());
        let length = section_name.len();
        if length == 0 {
            section_name = "(NULL)".to_string();
        }
        section_name
    }
    fn get_type(&self) -> String {
        let check_type = |const_type| self.sh_type as u64 == const_type;
        return if check_type(SHT_NULL) {
            "NULL".to_string()
        } else if check_type(SHT_PROGBITS) {
            "PROGBITS".to_string()
        } else if check_type(SHT_SYMTAB) {
            "SYMTAB".to_string()
        } else if check_type(SHT_STRTAB) {
            "STRTAB".to_string()
        } else if check_type(SHT_RELA) {
            "RELA".to_string()
        } else if check_type(SHT_HASH) {
            "HASH".to_string()
        } else if check_type(SHT_DYNAMIC) {
            "DYNAMIC".to_string()
        } else if check_type(SHT_NOTE) {
            "NOTE".to_string()
        } else if check_type(SHT_NOBITS) {
            "NOBITS".to_string()
        } else if check_type(SHT_REL) {
            "REL".to_string()
        } else if check_type(SHT_SHLIB) {
            "SHLIB".to_string()
        } else if check_type(SHT_DYNSYM) {
            "DYNSYM".to_string()
        } else if check_type(SHT_INIT_ARRAY) {
            "INIT_ARRAY".to_string()
        } else if check_type(SHT_FINI_ARRAY) {
            "FINI_ARRAY".to_string()
        } else if check_type(SHT_PREINIT_ARRAY) {
            "PREINIT_ARRAY".to_string()
        } else if check_type(SHT_GROUP) {
            "GROUP".to_string()
        } else if check_type(SHT_SYMTAB_SHNDX) {
            "SYMTAB SECTION INDICES".to_string()
        } else if check_type(SHT_GNU_HASH) {
            "GNU_HASH".to_string()
        } else if check_type(SHT_GNU_VERDEF) {
            "VERDEF".to_string()
        } else if check_type(SHT_GNU_VERNEED) {
            "VERNEED".to_string()
        } else if check_type(SHT_GNU_VERSYM) {
            "VERSYM".to_string()
        } else {
            "Invalid".to_string()
        };
    }
    fn get_flags(&self) -> String {
        let mut flag_string = String::new();
        let check_flag = |const_flag| self.sh_flags & const_flag != 0;
        Self::write_string_with_condition(&mut flag_string, 'W', check_flag(SHF_WRITE));
        Self::write_string_with_condition(&mut flag_string, 'A', check_flag(SHF_ALLOC));
        Self::write_string_with_condition(&mut flag_string, 'X', check_flag(SHF_EXECINSTR));
        Self::write_string_with_condition(&mut flag_string, 'M', check_flag(SHF_MERGE));
        Self::write_string_with_condition(&mut flag_string, 'S', check_flag(SHF_STRINGS));
        Self::write_string_with_condition(&mut flag_string, 'I', check_flag(SHF_INFO_LINK));
        Self::write_string_with_condition(&mut flag_string, 'L', check_flag(SHF_LINK_ORDER));
        Self::write_string_with_condition(&mut flag_string, 'O', check_flag(SHF_OS_NONCONFORMING));
        Self::write_string_with_condition(&mut flag_string, 'G', check_flag(SHF_GROUP));
        Self::write_string_with_condition(&mut flag_string, 'T', check_flag(SHF_TLS));
        Self::write_string_with_condition(&mut flag_string, 'E', check_flag(SHF_EXCLUDE));
        Self::write_string_with_condition(&mut flag_string, 'C', check_flag(SHF_COMPRESSED));
        flag_string
    }
    fn write_string_with_condition(s: &mut String, c: char, condition: bool) {
        if condition {
            s.push(c);
        }
    }
}
pub fn init_texthdr(size: u64) -> Shdr {
    Shdr {
        sh_name: 0,
        sh_type: SHT_PROGBITS as u32,
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
        sh_type: SHT_SYMTAB as u32,
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
        sh_type: SHT_STRTAB as u32,
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
        sh_type: SHT_RELA as u32,
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
pub fn init_debughdr(size: u64) -> Shdr {
    Shdr {
        sh_name: 0,
        sh_type: SHT_PROGBITS as u32,
        sh_flags: 0,
        sh_addr: 0,
        sh_offset: 0,
        sh_size: size,
        sh_link: 0,
        sh_info: 0,
        sh_addralign: 8,
        sh_entsize: DebugSymbol::size() as u64,
    }
}
pub fn init_documenthdr(size: u64) -> Shdr {
    Shdr {
        sh_name: 0,
        sh_type: SHT_NOTE as u32,
        sh_flags: 0,
        sh_addr: 0,
        sh_offset: 0,
        sh_size: size,
        sh_link: 0,
        sh_info: 0,
        sh_addralign: 8,
        sh_entsize: debug::LIMIT_DOCUMENTS as u64,
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
pub const PT_NULL: Elf64Word = 0;
pub const PT_LOAD: Elf64Word = 1;
pub const PT_DYNAMIC: Elf64Word = 2;
pub const PT_INTERP: Elf64Word = 3;
pub const PT_NOTE: Elf64Word = 4;
pub const PT_SHLIB: Elf64Word = 5;
pub const PT_PHDR: Elf64Word = 6;
pub const PT_TLS: Elf64Word = 7;
pub const PT_GNU_EH_FRAME: Elf64Word = 0x6474e550;
pub const PT_GNU_STACK: Elf64Word = 0x6474e551;
pub const PT_GNU_RELRO: Elf64Word = 0x6474e552;

pub const PF_X: Elf64Word = 1 << 0;
pub const PF_W: Elf64Word = 1 << 1;
pub const PF_R: Elf64Word = 1 << 2;
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
    pub fn new_unsafe(binary: Vec<u8>) -> Self {
        unsafe { std::ptr::read(binary.as_ptr() as *const Phdr) }
    }
    pub fn to_stdout(&self) -> Row {
        let mut cells: Vec<Cell> = Vec::new();
        ELF::add_cell(&mut cells, &self.get_type());
        ELF::add_cell(&mut cells, &format!("0x{:x}", self.p_offset));
        ELF::add_cell(&mut cells, &format!("0x{:x}", self.p_vaddr));
        ELF::add_cell(&mut cells, &format!("0x{:x}", self.p_paddr));
        ELF::add_cell(&mut cells, &format!("0x{:x}", self.p_filesz));
        ELF::add_cell(&mut cells, &format!("0x{:x}", self.p_memsz));
        ELF::add_cell(&mut cells, &self.get_flags());
        ELF::add_cell(&mut cells, &format!("0x{:x}", self.p_align));
        Row::new(cells)
    }
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
    fn get_type(&self) -> String {
        let check_type = |const_type| self.p_type == const_type;
        return if check_type(PT_NULL) {
            "NULL".to_string()
        } else if check_type(PT_LOAD) {
            "LOAD".to_string()
        } else if check_type(PT_DYNAMIC) {
            "DYNAMIC".to_string()
        } else if check_type(PT_INTERP) {
            "INTERP".to_string()
        } else if check_type(PT_NOTE) {
            "NOTE".to_string()
        } else if check_type(PT_SHLIB) {
            "SHLIB".to_string()
        } else if check_type(PT_PHDR) {
            "PHDR".to_string()
        } else if check_type(PT_TLS) {
            "TLS".to_string()
        } else if check_type(PT_GNU_EH_FRAME) {
            "GNU_EH_FRAME".to_string()
        } else if check_type(PT_GNU_STACK) {
            "GNU_STACK".to_string()
        } else if check_type(PT_GNU_RELRO) {
            "GNU_RELRO".to_string()
        } else {
            "INVALID".to_string()
        };
    }
    fn get_flags(&self) -> String {
        let mut flag_string = String::new();
        let check_flag = |const_flag| self.p_flags & const_flag != 0;
        Self::write_string_with_condition(&mut flag_string, 'R', check_flag(PF_R));
        Self::write_string_with_condition(&mut flag_string, 'W', check_flag(PF_W));
        Self::write_string_with_condition(&mut flag_string, 'E', check_flag(PF_X));
        flag_string
    }
    fn write_string_with_condition(s: &mut String, c: char, condition: bool) {
        if condition {
            s.push(c);
        }
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
/* Bind */
pub const STB_LOCAL: u8 = 0;
pub const STB_GLOBAL: u8 = 1;
pub const STB_WEAK: u8 = 2;

/* Type */
pub const STT_NOTYPE: u8 = 0;
pub const STT_OBJECT: u8 = 1;
pub const STT_FUNC: u8 = 2;
pub const STT_SECTION: u8 = 3;
pub const STT_FILE: u8 = 4;
pub const STT_COMMON: u8 = 5;
pub const STT_TLS: u8 = 6;
pub const STT_GNU_IFUNC: u8 = 10;

/* Visibility */
pub const STV_DEFAULT: u8 = 0;
pub const STV_INTERNAL: u8 = 1;
pub const STV_HIDDEN: u8 = 2;
pub const STV_PROTECTED: u8 = 3;

/* Index */
pub const SHN_UNDEF: Elf64Section = 0;
pub const SHN_ABS: Elf64Section = 0xff3f;
pub const SHN_COMMON: Elf64Section = 0xfff2;

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
    pub fn to_stdout(&self, elf_file: &ELF, link: u32) -> Row {
        let mut cells: Vec<Cell> = Vec::new();
        ELF::add_cell(&mut cells, &format!("0x{:x}", self.st_value));
        ELF::add_cell(&mut cells, &format!("0x{:x}", self.st_size));
        ELF::add_cell(&mut cells, &self.get_type());
        ELF::add_cell(&mut cells, &self.get_bind());
        ELF::add_cell(&mut cells, &self.get_visibility());
        ELF::add_cell(&mut cells, &self.get_index());
        ELF::add_cell(&mut cells, &self.get_name(elf_file, link));
        Row::new(cells)
    }
    fn get_type(&self) -> String {
        let check_type = |const_type| self.st_info & 0xf == const_type;
        return if check_type(STT_NOTYPE) {
            "NOTYPE".to_string()
        } else if check_type(STT_OBJECT) {
            "OBJECT".to_string()
        } else if check_type(STT_FUNC) {
            "FUNC".to_string()
        } else if check_type(STT_SECTION) {
            "SECTION".to_string()
        } else if check_type(STT_FILE) {
            "FILE".to_string()
        } else if check_type(STT_COMMON) {
            "COMMON".to_string()
        } else if check_type(STT_TLS) {
            "TLS".to_string()
        } else if check_type(STT_GNU_IFUNC) {
            "IFUNC".to_string()
        } else {
            "Invalid".to_string()
        };
    }
    fn get_bind(&self) -> String {
        let check_bind = |const_bind| (self.st_info as u8) >> 4 == const_bind;
        return if check_bind(STB_LOCAL) {
            "LOCAL".to_string()
        } else if check_bind(STB_GLOBAL) {
            "GLOBAL".to_string()
        } else if check_bind(STB_WEAK) {
            "WEAK".to_string()
        } else {
            "Invalid".to_string()
        };
    }
    fn get_visibility(&self) -> String {
        let check_vis = |const_vis| (self.st_other as u8) & 0x03 == const_vis;
        return if check_vis(STV_DEFAULT) {
            "DEFAULT".to_string()
        } else if check_vis(STV_INTERNAL) {
            "INTERNAL".to_string()
        } else if check_vis(STV_HIDDEN) {
            "HIDDEN".to_string()
        } else if check_vis(STV_PROTECTED) {
            "PROTECTED".to_string()
        } else {
            "Invalid".to_string()
        };
    }
    fn get_index(&self) -> String {
        let check_index = |const_index| self.st_shndx == const_index;
        return if check_index(SHN_UNDEF) {
            "UND".to_string()
        } else if check_index(SHN_COMMON) {
            "COM".to_string()
        } else if check_index(SHN_ABS) {
            "ABS".to_string()
        } else if check_index(0xfff1) {
            "ABS".to_string()
        } else {
            format!("{}", self.st_shndx)
        };
    }
    pub fn get_name(&self, elf_file: &ELF, link: u32) -> String {
        let strtab: Vec<u8> = if (self.st_info & 0xf) == STT_SECTION {
            elf_file.sections[elf_file.ehdr.e_shstrndx as usize].clone()
        } else {
            elf_file.sections[link as usize].clone()
        };
        let mut section_name = ELF::collect_name(strtab[self.st_name as usize..].to_vec());
        let length = section_name.len();
        if length == 0 {
            section_name = "(NULL)".to_string();
        }
        section_name
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

pub const R_X86_64_NONE: u64 = 0;
pub const R_X86_64_64: u64 = 1;
pub const R_X86_64_PC32: u64 = 2;
pub const R_X86_64_GOT32: u64 = 3;
pub const R_X86_64_PLT32: u64 = 4;
pub const R_X86_64_COPY: u64 = 5;
pub const R_X86_64_GLOB_DAT: u64 = 6;
pub const R_X86_64_JUMP_SLOT: u64 = 7;
pub const R_X86_64_RELATIVE: u64 = 8;
pub const R_X86_64_GOTPCREL: u64 = 9;
pub const R_X86_64_32: u64 = 9;

/* AMD x86-64 relocations.  */
//#define R_X86_64_NONE		0	/* No reloc */
/* AMD x86-64 relocations.  */
//#define R_X86_64_NONE		0	/* No reloc */
//#define R_X86_64_64		1	/* Direct 64 bit  */
//#define R_X86_64_PC32		2	/* PC relative 32 bit signed */
//#define R_X86_64_GOT32		3	/* 32 bit GOT entry */
//#define R_X86_64_PLT32		4	/* 32 bit PLT address */
//#define R_X86_64_COPY		5	/* Copy symbol at runtime */
//#define R_X86_64_GLOB_DAT	6	/* Create GOT entry */
//#define R_X86_64_JUMP_SLOT	7	/* Create PLT entry */
//#define R_X86_64_RELATIVE	8	/* Adjust by program base */
//#define R_X86_64_GOTPCREL	9	/* 32 bit signed PC relative
//offset to GOT */
//#define R_X86_64_32		10	/* Direct 32 bit zero extended */
//#define R_X86_64_32S		11	/* Direct 32 bit sign extended */
//#define R_X86_64_16		12	/* Direct 16 bit zero extended */
//#define R_X86_64_PC16		13	/* 16 bit sign extended pc relative */
//#define R_X86_64_8		14	/* Direct 8 bit sign extended  */
//#define R_X86_64_PC8		15	/* 8 bit sign extended pc relative */
////#define R_X86_64_DTPMOD64	16	/* ID of module containing symbol */
//#define R_X86_64_DTPOFF64	17	/* Offset in module's TLS block */
//#define R_X86_64_TPOFF64	18	/* Offset in initial TLS block */
//#define R_X86_64_TLSGD		19	/* 32 bit signed PC relative offset
//to two GOT entries for GD symbol */
//#define R_X86_64_TLSLD		20	/* 32 bit signed PC relative offset
//to two GOT entries for LD symbol */
//#define R_X86_64_DTPOFF32	21	/* Offset in TLS block */
//#define R_X86_64_GOTTPOFF	22	/* 32 bit signed PC relative offset
//to GOT entry for IE symbol */
//#define R_X86_64_TPOFF32	23	/* Offset in initial TLS block */
////#define R_X86_64_PC64		24	/* PC relative 64 bit */
//#define R_X86_64_GOTOFF64	25	/* 64 bit offset to GOT */
//#define R_X86_64_GOTPC32	26	/* 32 bit signed pc relative
//offset to GOT */
//#define R_X86_64_GOT64		27	/* 64-bit GOT entry offset */
//#define R_X86_64_GOTPCREL64	28	/* 64-bit PC relative offset
//to GOT entry */
//#define R_X86_64_GOTPC64	29	/* 64-bit PC relative offset to GOT */
//#define R_X86_64_GOTPLT64	30 	/* like GOT64, says PLT entry needed */
//#define R_X86_64_PLTOFF64	31	/* 64-bit GOT relative offset
//to PLT entry */
//#define R_X86_64_SIZE32		32	/* Size of symbol plus 32-bit addend */
//#define R_X86_64_SIZE64		33	/* Size of symbol plus 64-bit addend */
//#define R_X86_64_GOTPC32_TLSDESC 34	/* GOT offset for TLS descriptor.  */
//#define R_X86_64_TLSDESC_CALL   35	/* Marker for call through TLS
//descriptor.  */
//#define R_X86_64_TLSDESC        36	/* TLS descriptor.  */
//#define R_X86_64_IRELATIVE	37	/* Adjust indirectly by program base */
//#define R_X86_64_RELATIVE64	38	/* 64-bit adjust by program base */
/////* 39 Reserved was R_X86_64_PC32_BND */
/////* 40 Reserved was R_X86_64_PLT32_BND */
//#define R_X86_64_GOTPCRELX	41	/* Load from 32 bit signed pc relative
//offset to GOT entry without REX
//prefix, relaxable.  */
//#define R_X86_64_REX_GOTPCRELX	42	/* Load from 32 bit signed pc relative
//offset to GOT entry with REX prefix,
//relaxable.  */
//#define R_X86_64_NUM		43

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
    pub fn to_stdout(&self, elf_file: &ELF, related_symtab_sh_link: usize) -> Row {
        let mut cells: Vec<Cell> = Vec::new();
        ELF::add_cell(&mut cells, &format!("0x{:x}", self.r_offset));
        ELF::add_cell(&mut cells, &format!("0x{:x}", self.r_info));
        ELF::add_cell(&mut cells, &self.get_type());

        let related_symbol_index = self.r_info >> 32;

        ELF::add_cell(
            &mut cells,
            &self.get_sym_value(elf_file, related_symbol_index, related_symtab_sh_link),
        );
        ELF::add_cell(
            &mut cells,
            &self.get_symbol_name_and_addend(
                elf_file,
                related_symbol_index,
                related_symtab_sh_link,
            ),
        );
        Row::new(cells)
    }
    fn get_type(&self) -> String {
        let check_type = |const_type| self.r_info & 0xffffffff == const_type;
        return if check_type(R_X86_64_NONE) {
            "R_X86_64_NONE".to_string()
        } else if check_type(R_X86_64_64) {
            "R_X86_64_64".to_string()
        } else if check_type(R_X86_64_PC32) {
            "R_X86_64_PC32".to_string()
        } else if check_type(R_X86_64_GOT32) {
            "R_X86_64_GOT32".to_string()
        } else if check_type(R_X86_64_PLT32) {
            "R_X86_64_PLT32".to_string()
        } else if check_type(R_X86_64_COPY) {
            "R_X86_64_COPY".to_string()
        } else if check_type(R_X86_64_GLOB_DAT) {
            "R_X86_64_GLOB_DAT".to_string()
        } else if check_type(R_X86_64_JUMP_SLOT) {
            "R_X86_64_JUMP_SLOT".to_string()
        } else if check_type(R_X86_64_RELATIVE) {
            "R_X86_64_RELATIVE".to_string()
        } else if check_type(R_X86_64_GOTPCREL) {
            "R_X86_64_GOTPCREL".to_string()
        } else if check_type(R_X86_64_32) {
            "R_X86_64_32".to_string()
        } else {
            "Invalid".to_string()
        };
    }
    fn get_sym_value(
        &self,
        elf_file: &ELF,
        related_symbol_index: u64,
        related_symtab_sh_link: usize,
    ) -> String {
        let symtab = elf_file.sections[related_symtab_sh_link].clone();
        let symbol_binary = symtab[related_symbol_index as usize * Symbol::size() as usize
            ..(related_symbol_index as usize + 1) * Symbol::size() as usize]
            .to_vec();
        let symbol = Symbol::new_unsafe(symbol_binary);
        format!("0x{:x}", symbol.st_value)
    }
    fn get_symbol_name_and_addend(
        &self,
        elf_file: &ELF,
        related_symbol_index: u64,
        related_symtab_sh_link: usize,
    ) -> String {
        let symtab = elf_file.sections[related_symtab_sh_link].clone();
        let symtab_shdr = elf_file.shdrs[related_symtab_sh_link].clone();
        let symbol_binary = symtab[related_symbol_index as usize * Symbol::size()
            ..(related_symbol_index as usize + 1) * Symbol::size()]
            .to_vec();
        let symbol = Symbol::new_unsafe(symbol_binary);

        let strtab = elf_file.sections[symtab_shdr.sh_link as usize].clone();
        let symbol_name = ELF::collect_name(strtab[symbol.st_name as usize..].to_vec());

        if symbol_name.len() == 0 {
            return format!("{:x}", self.r_addend);
        }

        if self.r_addend < 0 {
            format!("{} - {}", symbol_name, !self.r_addend)
        } else {
            format!("{} + {}", symbol_name, self.r_addend)
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

pub const DT_NULL: Elf64Sxword = 0;
pub const DT_NEEDED: Elf64Sxword = 1;
pub const DT_PLTRELSZ: Elf64Sxword = 2;
pub const DT_PLTGOT: Elf64Sxword = 3;
pub const DT_STRTAB: Elf64Sxword = 5;
pub const DT_SYMTAB: Elf64Sxword = 6;
pub const DT_RELA: Elf64Sxword = 7;
pub const DT_RELASZ: Elf64Sxword = 8;
pub const DT_RELAENT: Elf64Sxword = 9;
pub const DT_STRSZ: Elf64Sxword = 10;
pub const DT_SYMENT: Elf64Sxword = 11;
pub const DT_INIT: Elf64Sxword = 12;
pub const DT_FINI: Elf64Sxword = 13;
pub const DT_RPATH: Elf64Sxword = 15;
pub const DT_PLTREL: Elf64Sxword = 20;
pub const DT_DEBUG: Elf64Sxword = 21;
pub const DT_JMPREL: Elf64Sxword = 23;
pub const DT_INIT_ARRAY: Elf64Sxword = 25;
pub const DT_FINI_ARRAY: Elf64Sxword = 26;
pub const DT_INIT_ARRAYSZ: Elf64Sxword = 27;
pub const DT_FINI_ARRAYSZ: Elf64Sxword = 28;
pub const DT_RUNPATH: Elf64Sxword = 29;
pub const DT_FLAGS: Elf64Sxword = 30;
pub const DT_GNU_HASH: Elf64Sxword = 0x6ffffef5;
pub const DT_VERSYM: Elf64Sxword = 0x6ffffff0;
pub const DT_RELACOUNT: Elf64Sxword = 0x6ffffff9;
pub const DT_FLAGS_1: Elf64Sxword = 0x6ffffffb;
pub const DT_VERNEED: Elf64Sxword = 0x6ffffffe;
pub const DT_VERNEEDNUM: Elf64Sxword = 0x6fffffff;

pub const DF_1_PIE: Elf64Sxword = 0x08000000;
pub const DF_BIND_NOW: Elf64Sxword = 0x00000008;

pub struct Dyn {
    d_tag: Elf64Sxword,
    d_un: Elf64Xword, // d_val || d_ptr
}

impl Dyn {
    pub fn new_unsafe(binary: Vec<u8>) -> Dyn {
        unsafe { std::ptr::read(binary.as_ptr() as *const Dyn) }
    }
    pub fn size() -> usize {
        16
    }
    pub fn to_stdout(&self, elf_file: &ELF, related_symtab_sh_link: usize) -> Row {
        let mut cells: Vec<Cell> = Vec::new();
        ELF::add_cell(&mut cells, &format!("0x{:x}", self.d_tag));
        ELF::add_cell(&mut cells, &self.get_type());
        ELF::add_cell(
            &mut cells,
            &self.get_name_or_value(elf_file, related_symtab_sh_link),
        );
        Row::new(cells)
    }
    fn get_dynamic_type(tag: Elf64Sxword) -> String {
        let check_type = |const_type| tag == const_type;
        if check_type(DT_NULL) {
            "(NULL)".to_string()
        } else if check_type(DT_NEEDED) {
            "(NEEDED)".to_string()
        } else if check_type(DT_PLTGOT) {
            "(PLTGOT)".to_string()
        } else if check_type(DT_PLTRELSZ) {
            "(PLTRELSZ)".to_string()
        } else if check_type(DT_STRTAB) {
            "(STRTAB)".to_string()
        } else if check_type(DT_SYMTAB) {
            "(SYMTAB)".to_string()
        } else if check_type(DT_RELA) {
            "(RELA)".to_string()
        } else if check_type(DT_RELASZ) {
            "(RELASZ)".to_string()
        } else if check_type(DT_RELAENT) {
            "(RELAENT)".to_string()
        } else if check_type(DT_STRSZ) {
            "(STRSZ)".to_string()
        } else if check_type(DT_JMPREL) {
            "(JMPREL)".to_string()
        } else if check_type(DT_SYMENT) {
            "(SYMENT)".to_string()
        } else if check_type(DT_INIT) {
            "(INIT)".to_string()
        } else if check_type(DT_FINI) {
            "(FINI)".to_string()
        } else if check_type(DT_INIT_ARRAY) {
            "(INIT_ARRAY)".to_string()
        } else if check_type(DT_FINI_ARRAY) {
            "(FINI_ARRAY)".to_string()
        } else if check_type(DT_INIT_ARRAYSZ) {
            "(INIT_ARRAYSZ)".to_string()
        } else if check_type(DT_FINI_ARRAYSZ) {
            "(FINI_ARRAYSZ)".to_string()
        } else if check_type(DT_PLTREL) {
            "(PLTREL)".to_string()
        } else if check_type(DT_DEBUG) {
            "(DEBUG)".to_string()
        } else if check_type(DT_GNU_HASH) {
            "(GNU_HASH)".to_string()
        } else if check_type(DT_RELACOUNT) {
            "(RELACOUNT)".to_string()
        } else if check_type(DT_VERSYM) {
            "(VERSYM)".to_string()
        } else if check_type(DT_VERNEED) {
            "(VERNEED)".to_string()
        } else if check_type(DT_VERNEEDNUM) {
            "(VERNEEDNUM)".to_string()
        } else if check_type(DT_FLAGS_1) {
            "(FLAGS_1)".to_string()
        } else {
            "Invalid".to_string()
        }
    }
    fn get_type(&self) -> String {
        Self::get_dynamic_type(self.d_tag)
    }
    fn get_name_or_value(&self, elf_file: &ELF, related_symtab_sh_link: usize) -> String {
        let strtab = elf_file.sections[related_symtab_sh_link].clone();
        match Self::get_name_kind(self.d_tag) {
            DynNameKind::Library => format!(
                "Shared Library: [{}]",
                ELF::collect_name(strtab[self.d_un as usize..].to_vec())
            ),
            DynNameKind::DynamicType => Self::get_dynamic_type(self.d_un as i64),
            DynNameKind::Flag1 => format!("Flag: {}", Self::get_flags_1(self.d_un)),
            DynNameKind::Size => format!("{} (bytes)", self.d_un),
            DynNameKind::Else => format!("0x{:x}", self.d_un),
        }
    }
    fn get_name_kind(d_un: Elf64Sxword) -> DynNameKind {
        match d_un {
            DT_NEEDED => DynNameKind::Library,
            DT_FLAGS_1 => DynNameKind::Flag1,
            DT_PLTREL => DynNameKind::DynamicType,
            DT_STRSZ => DynNameKind::Size,
            DT_SYMENT => DynNameKind::Size,
            DT_RELASZ => DynNameKind::Size,
            DT_RELAENT => DynNameKind::Size,
            DT_PLTRELSZ => DynNameKind::Size,
            DT_INIT_ARRAYSZ => DynNameKind::Size,
            DT_FINI_ARRAYSZ => DynNameKind::Size,
            _ => {
                eprintln!("else found -> {}", d_un);
                DynNameKind::Else
            }
        }
    }
    fn get_flags_1(d_un: Elf64Xword) -> String {
        let check_un = |const_un| d_un == const_un as u64;
        if check_un(DF_1_PIE) {
            "PIE".to_string()
        } else {
            "Invalid".to_string()
        }
    }
}

enum DynNameKind {
    Library,
    Flag1,
    DynamicType,
    Size,
    Else,
}
