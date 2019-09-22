use super::super::object::elf::elf64 as e;

use e::ELF;

pub static BASE_ADDRESS: u64 = 0x400000;
pub static PAGE_SIZE: u64 = 0x1000;
impl ELF {
    pub fn linking(&mut self) {
        self.init_phdr();
        self.prepare_ehdr_for_staticlink();
        for _ in 0..PAGE_SIZE - e::Ehdr::size() as u64 - e::Phdr::size() as u64 {
            self.sections[0].push(0x00);
        }
        let _ = self
            .shdrs
            .iter_mut()
            .map(|shdr| shdr.sh_offset = PAGE_SIZE - e::Ehdr::size() as u64 + shdr.sh_offset)
            .collect::<()>();
        self.link_symbols();
        self.shdrs[1].sh_addr = BASE_ADDRESS;
    }
    pub fn link_symbols(&mut self) {
        let strtab: Vec<u8> = self.get_section(".strtab");
        let mut symbols: Vec<e::Symbol> = self.get_symbols();
        for symbol in symbols.iter_mut() {
            if strtab[symbol.st_name as usize] as char == '_' {
                self.ehdr.e_entry = BASE_ADDRESS + symbol.st_value;
            }
            symbol.st_value += BASE_ADDRESS;
        }
        let symtab_number: usize = self.get_section_number(".symtab");
        self.sections[symtab_number] = e::symbols_to_vec(symbols);
        self.resolve_symbols();
    }
    pub fn resolve_symbols(&mut self) {
        let symbols: Vec<e::Symbol> = self.get_symbols();
        let mut relas: Vec<e::Rela> = self.get_relas(".relatext");
        for rel in relas.iter_mut() {
            let address = symbols[e::Rela::bind(rel.r_info)].st_value as u32;
            for (idx, b) in address.to_le_bytes().to_vec().iter().enumerate() {
                let text_number: usize = self.get_section_number(".text");
                self.sections[text_number][rel.r_offset as usize + idx] = *b;
            }
        }
    }
    fn prepare_ehdr_for_staticlink(&mut self) {
        self.ehdr.e_type = e::ET_EXEC;
        self.ehdr.e_phoff = e::Ehdr::size() as u64; // sizeof(Ehdr)
        self.ehdr.e_phnum = 1;
        self.ehdr.e_phentsize = e::Phdr::size() as u16; // sizeof(Phdr)
        self.ehdr.e_shoff = PAGE_SIZE
            + self.sections[1..]
                .to_vec()
                .iter()
                .map(|sec| sec.len() as u64)
                .sum::<u64>();
    }
    fn get_section(&self, name: &str) -> Vec<u8> {
        let sec_number: usize = self.get_section_number(name);
        self.sections[sec_number].clone()
    }
    fn get_symbols(&self) -> Vec<e::Symbol> {
        let symbin: Vec<u8> = self.get_section(".symtab");
        let symbol_number: usize = symbin.len() / e::Symbol::size();
        let mut symbols: Vec<e::Symbol> = vec![e::init_nullsym()];
        for i in 0..symbol_number - 1 {
            symbols.push(e::Symbol::new_unsafe(
                symbin[(i + 1) * e::Symbol::size()..].to_vec(),
            ));
        }
        symbols
    }
    fn get_relas(&self, name: &str) -> Vec<e::Rela> {
        let relbin: Vec<u8> = self.get_section(name);
        let relas_number: usize = relbin.len() / e::Rela::size();
        let mut relas: Vec<e::Rela> = Vec::new();
        for i in 0..relas_number {
            relas.push(e::Rela::new_unsafe(relbin[i * e::Rela::size()..].to_vec()));
        }
        relas
    }
    fn init_phdr(&mut self) {
        let mut phdr: e::Phdr = e::init_phdr();
        phdr.p_type = e::PT_LOAD;
        phdr.p_offset = PAGE_SIZE;
        phdr.p_vaddr = BASE_ADDRESS;
        phdr.p_paddr = BASE_ADDRESS;
        phdr.p_align = PAGE_SIZE;
        let text: Vec<u8> = self.get_section(".text");
        phdr.p_filesz = text.len() as u64; // remove the hardcode
        phdr.p_memsz = text.len() as u64; // remove the hardcode
        phdr.p_flags = e::PF_R | e::PF_X | e::PF_W;
        self.phdrs = Some(vec![phdr]);
    }
}
