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
        let symbols: Vec<e::Symbol> = build_symbols(self.sections[2].clone());
        let relbin: Vec<u8> = self.sections[4].clone();
        let rel_number = relbin.len() / e::Rela::size();
        for i in 0..rel_number {
            let rel: e::Rela = e::Rela::new_unsafe(relbin[i * e::Rela::size()..].to_vec());
            let address = symbols[rel.r_info as usize >> 32].st_value as u32;
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
        self.sections[*self.names.get(name).unwrap()].clone()
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
    fn init_phdr(&mut self) {
        let mut phdr: e::Phdr = e::init_phdr();
        phdr.p_type = e::PT_LOAD;
        phdr.p_offset = PAGE_SIZE;
        phdr.p_vaddr = BASE_ADDRESS;
        phdr.p_paddr = BASE_ADDRESS;
        phdr.p_align = PAGE_SIZE;
        let text_number: usize = self.get_section_number(".text");
        phdr.p_filesz = self.sections[text_number].len() as u64; // remove the hardcode
        phdr.p_memsz = self.sections[text_number].len() as u64; // remove the hardcode
        phdr.p_flags = e::PF_R | e::PF_X | e::PF_W;
        self.phdrs = Some(vec![phdr]);
    }
}

fn build_symbols(bin: Vec<u8>) -> Vec<e::Symbol> {
    let mut symbols: Vec<e::Symbol> = vec![e::init_nullsym()];
    let symbol_number = bin.len() / e::Symbol::size();
    for i in 0..symbol_number - 1 {
        let symbol: e::Symbol = e::Symbol::new_unsafe(bin[(i + 1) * e::Symbol::size()..].to_vec());
        symbols.push(symbol);
    }
    symbols
}
