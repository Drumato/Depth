use super::super::object::elf::elf64 as e;
use e::ELF;

impl ELF {
    pub fn linking(&mut self) {
        self.ehdr.e_type = e::ET_EXEC;
        self.ehdr.e_shoff = 0;
        self.ehdr.e_shentsize = 0;
        self.ehdr.e_shnum = 0;
        self.ehdr.e_shstrndx = 0;
        self.ehdr.e_phoff = 0x40; // sizeof(Ehdr)
        self.ehdr.e_phnum = 1;
        self.ehdr.e_phentsize = 52; // sizeof(Phdr)
        self.sections.clear();
        self.shdrs.clear();
        let _phdr: e::Phdr = e::init_phdr(e::PT_LOAD, 0x400000);
    }
}
