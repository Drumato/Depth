use super::super::object::elf::elf64 as e;
use e::ELF;

impl ELF {
    pub fn linking(&mut self) {
        let mut phdr: e::Phdr = e::init_phdr();
        phdr.p_type = e::PT_LOAD;
        phdr.p_vaddr = 0x400000;
        phdr.p_paddr = 0x400000;
        phdr.p_align = 0x1000;
        phdr.p_filesz = self.sections[1].len() as u64; // remove the hardcode
        phdr.p_memsz = self.sections[1].len() as u64; // remove the hardcode
        phdr.p_flags = e::PF_R | e::PF_X | e::PF_W;
        self.phdrs = Some(vec![phdr]);
        self.ehdr.e_type = e::ET_EXEC;
        self.ehdr.e_phoff = 0x40; // sizeof(Ehdr)
        self.ehdr.e_phnum = 1;
        self.ehdr.e_phentsize = 56; // sizeof(Phdr)
        self.ehdr.e_shoff += self.ehdr.e_phentsize as u64;
        let _ = self
            .shdrs
            .iter_mut()
            .map(|shdr| shdr.sh_offset += 56)
            .collect::<()>();
        self.link_symbols();
    }
    pub fn link_symbols(&mut self) {
        let bin: Vec<u8> = self.sections[2].clone();
        let strtab: Vec<u8> = self.sections[3].clone();
        let symbol_number = bin.len() / 24;
        let mut symbols: Vec<e::Symbol> = vec![e::init_nullsym()];
        for i in 0..symbol_number - 1 {
            let mut symbol: e::Symbol = e::Symbol::new_unsafe(bin[(i + 1) * 24..].to_vec());
            if strtab[symbol.st_name as usize] as char == '_' {
                self.ehdr.e_entry = 0x400040 + symbol.st_value;
            }
            symbol.st_value += 0x400040;
            symbols.push(symbol);
        }
        self.sections[2] = e::symbols_to_vec(symbols);
    }
    pub fn resolve_symbols(&mut self) {
        let symbols: Vec<e::Symbol> = build_symbols(self.sections[2].clone());
        let relbin: Vec<u8> = self.sections[4].clone();
        let rel_number = relbin.len() / 24;
        for i in 0..rel_number {
            let rel: e::Rela = e::Rela::new_unsafe(relbin[i * 24..].to_vec());
            rel.r_info & 0xffff0000;
        }
    }
}

fn build_symbols(bin: Vec<u8>) -> Vec<e::Symbol> {
    let mut symbols: Vec<e::Symbol> = vec![e::init_nullsym()];
    let symbol_number = bin.len() / 24;
    for i in 0..symbol_number - 1 {
        let mut symbol: e::Symbol = e::Symbol::new_unsafe(bin[(i + 1) * 24..].to_vec());
        symbols.push(symbol);
    }
    symbols
}
