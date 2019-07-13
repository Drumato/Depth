use byteorder::{ByteOrder, LittleEndian};
pub const SHF_WRITE: u64 = 1 << 0;
pub const SHF_ALLOC: u64 = 1 << 1;
pub const SHF_EXECUTE: u64 = 1 << 2;
pub const SHT_NULL: u32 = 0;
pub const SHT_PROGBITS: u32 = 1;
pub const SHT_SYMTAB: u32 = 2;
pub const SHT_STRTAB: u32 = 3;
pub struct ShdrTable {
    pub shdrs: Vec<Shdr>,
}
impl ShdrTable {
    pub fn new(ss: Vec<Shdr>) -> ShdrTable {
        ShdrTable { shdrs: ss }
    }
    pub fn out(&self) {
        for s in self.shdrs.iter() {
            s.out();
        }
    }
}
pub struct Shdr {
    pub sh_name: u32,      /* Section name (string tbl index) */
    pub sh_type: u32,      /* Section type */
    pub sh_flags: u64,     /* Section flags */
    pub sh_addr: u64,      /* Section virtual addr at execution */
    pub sh_offset: u64,    /* Section file offset */
    pub sh_size: u64,      /* Section size in bytes */
    pub sh_link: u32,      /* Link to another section */
    pub sh_info: u32,      /* Additional section information */
    pub sh_addralign: u64, /* Section alignment */
    pub sh_entsize: u64,   /* Entry size if section holds table */
}

impl Shdr {
    pub fn new(b: Vec<u8>) -> Shdr {
        Shdr {
            sh_name: LittleEndian::read_u32(&b[0..4]),
            sh_type: LittleEndian::read_u32(&b[4..8]),
            sh_flags: LittleEndian::read_u64(&b[8..16]),
            sh_addr: LittleEndian::read_u64(&b[16..24]),
            sh_offset: LittleEndian::read_u64(&b[24..32]),
            sh_size: LittleEndian::read_u64(&b[32..40]),
            sh_link: LittleEndian::read_u32(&b[40..44]),
            sh_info: LittleEndian::read_u32(&b[44..48]),
            sh_addralign: LittleEndian::read_u64(&b[48..56]),
            sh_entsize: LittleEndian::read_u64(&b[56..64]),
        }
    }
    pub fn out(&self) {
        println!("sh_name->0x{:x}", self.sh_name);
        println!("sh_type->0x{:x}", self.sh_type);
        println!("sh_flags->0x{:x}", self.sh_flags);
        println!("sh_addr->0x{:x}", self.sh_addr);
        println!("sh_offset->0x{:x}", self.sh_offset);
        println!("sh_size->0x{:x}", self.sh_size);
        println!("sh_link->0x{:x}", self.sh_link);
        println!("sh_info->0x{:x}", self.sh_info);
        println!("sh_addralign->0x{:x}", self.sh_addralign);
        println!("sh_entsize->0x{:x}", self.sh_entsize);
    }
    pub fn bin(&self) -> Vec<u8> {
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
