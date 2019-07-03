use super::super::binary::bytes;
use byteorder::{BigEndian, ByteOrder, LittleEndian};
use bytes::Bin;
pub struct Ehdr {
    pub e_ident: u128,    /* magic number and other info */
    pub e_type: u16,      /* Object file type */
    pub e_machine: u16,   /* Architecture */
    pub e_version: u32,   /* Object file version */
    pub e_entry: u64,     /* Entry point virtual address */
    pub e_phoff: u64,     /* Program header table file offset */
    pub e_shoff: u64,     /* Section header table file offset */
    pub e_flags: u32,     /* Processor-specific flags */
    pub e_ehsize: u16,    /* ELF header size in bytes */
    pub e_phentsize: u16, /* Program header table entry size */
    pub e_phnum: u16,     /* Program header table entry count */
    pub e_shentsize: u16, /* Section header table entry size */
    pub e_shnum: u16,     /* Section header table entry count */
    pub e_shstrndx: u16,  /* Section header string table index */
}
impl Ehdr {
    pub fn new(b: Vec<u8>) -> Ehdr {
        if !Ehdr::check_emagic(&b) {
            println!("invalid elf-magic-number: got {:?}", &b[0..4]);
        }
        if b[4] != 0x02 {
            println!("can't analyze elf32");
        }
        if b[5] != 0x01 {
            println!("invalid endian");
        }
        if b[6] != 0x01 {
            println!("invalid type");
        }
        let e_type: u16 = LittleEndian::read_u16(&b[16..18]);
        if !Ehdr::check_etype(e_type) {
            println!("invalid elf-type-number: got {}", e_type);
        }
        if !Ehdr::check_machine(&b) {
            println!("invalid elf-machine-number");
        }
        Ehdr {
            e_ident: BigEndian::read_u128(&b[..16]),
            e_type: e_type,
            e_machine: LittleEndian::read_u16(&b[18..20]),
            e_version: LittleEndian::read_u32(&b[20..24]),
            e_entry: LittleEndian::read_u64(&b[24..32]),
            e_phoff: LittleEndian::read_u64(&b[32..40]),
            e_shoff: LittleEndian::read_u64(&b[40..48]),
            e_flags: LittleEndian::read_u32(&b[48..52]),
            e_ehsize: LittleEndian::read_u16(&b[52..54]),
            e_phentsize: LittleEndian::read_u16(&b[54..56]),
            e_phnum: LittleEndian::read_u16(&b[56..58]),
            e_shentsize: LittleEndian::read_u16(&b[58..60]),
            e_shnum: LittleEndian::read_u16(&b[60..62]),
            e_shstrndx: LittleEndian::read_u16(&b[62..64]),
        }
    }
    fn check_emagic(u: &Vec<u8>) -> bool {
        ((u[0] == 0x7f) && (u[1] == 0x45) && (u[2] == 0x4c) && (u[3] == 0x46))
    }
    fn check_etype(u: u16) -> bool {
        (0 <= u && u <= 4)
    }
    fn check_machine(u: &Vec<u8>) -> bool {
        ((u[18] == 0x3e) && (u[19] == 0x00))
    }
    pub fn out(&self) {
        println!("e_ident->0x{:x}", self.e_ident);
        println!("e_type->0x{:x}", self.e_type);
        println!("e_machine->0x{:x}", self.e_machine);
        println!("e_version->0x{:x}", self.e_version);
        println!("e_entry->0x{:x}", self.e_entry);
        println!("e_phoff->0x{:x}", self.e_phoff);
        println!("e_shoff->0x{:x}", self.e_shoff);
        println!("e_flags->0x{:x}", self.e_flags);
        println!("e_ehsize->0x{:x}", self.e_ehsize);
        println!("e_phentsize->0x{:x}", self.e_phentsize);
        println!("e_phnum->0x{:x}", self.e_phnum);
        println!("e_shentsize->0x{:x}", self.e_shentsize);
        println!("e_shnum->0x{:x}", self.e_shnum);
        println!("e_shstrndx->0x{:x}", self.e_shstrndx);
    }
}
