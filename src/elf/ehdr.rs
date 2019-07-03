use super::super::binary::bytes;
use bytes::Bin;
pub struct Ehdr {
    e_ident: u128,    /* magic number and other info */
    e_type: u16,      /* Object file type */
    e_machine: u8,    /* Architecture */
    e_version: u32,   /* Object file version */
    e_entry: u64,     /* Entry point virtual address */
    e_phoff: u64,     /* Program header table file offset */
    e_shoff: u64,     /* Section header table file offset */
    e_flags: u32,     /* Processor-specific flags */
    e_ehsize: u16,    /* ELF header size in bytes */
    e_phentsize: u16, /* Program header table entry size */
    e_phnum: u16,     /* Program header table entry count */
    e_shentsize: u16, /* Section header table entry size */
    e_shnum: u16,     /* Section header table entry count */
    e_shstrndx: u16,  /* Section header string table index */
}
impl Ehdr {
    pub fn new(b:Vec<u8>) -> Ehdr{
        let mut e_ident : u128;
       Ehdr{
    e_ident: e_ident,
    e_type:
    e_machine:
    e_version:
    e_entry:
    e_phoff:
    e_shoff:
    e_flags:
    e_ehsize:
    e_phentsize:
    e_phnum:
    e_shentsize:
    e_shnum:
    e_shstrndx:
        }
    }
    pub fn check_ehdr(&self) -> bool {
        self.check_emagic() && self.check_etype()
    }
    fn check_emagic(&self) -> bool {
        let elf_mag0: u8 = (self.e_ident << 120) as u8;
        let elf_mag1: u8 = (self.e_ident << 112) as u8;
        let elf_mag2: u8 = (self.e_ident << 104) as u8;
        let elf_mag3: u8 = (self.e_ident << 96) as u8;
        ((elf_mag0 == 0x7f) && (elf_mag1 == 0x45) && (elf_mag2 == 0x4c) && (elf_mag3 == 0x46))
    }
    fn check_etype(&self) -> bool {
        (0 <= self.e_type && self.e_type <= 4)
    }
}
