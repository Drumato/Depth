use crate::object::elf::elf64;
use elf64::ELF;

pub struct Linker {
    pub obj: ELF,
}
impl Linker {
    pub fn linking(mut elf_file: ELF) -> ELF {
        //exec_file.condition();
        elf_file.linking();
        elf_file
    }
}

pub static BASE_ADDRESS: u64 = 0x400000;
pub static PAGE_SIZE: u64 = 0x1000;
impl ELF {
    fn linking(&mut self) {
        self.init_phdr();
        self.prepare_ehdr_for_staticlink();
        self.padding();
        self.conditioning_section_offset();
        self.link_symbols();
        let text_number: usize = self.get_section_number(".text");
        self.shdrs[text_number].sh_addr = BASE_ADDRESS;
    }
    fn link_symbols(&mut self) {
        let strtab: Vec<u8> = self.get_section(".strtab");
        let mut symbols: Vec<elf64::Symbol> = self.get_symbols();
        for symbol in symbols.iter_mut() {
            if strtab[symbol.st_name as usize] as char == '_' {
                self.ehdr.e_entry = BASE_ADDRESS + symbol.st_value;
            }
            symbol.st_value += BASE_ADDRESS;
        }
        let symtab_number: usize = self.get_section_number(".symtab");
        self.sections[symtab_number] = elf64::symbols_to_vec(symbols);
        self.resolve_symbols();
    }
    fn resolve_symbols(&mut self) {
        let symbols: Vec<elf64::Symbol> = self.get_symbols();
        let mut relas: Vec<elf64::Rela> = self.get_relas(".rela.text");
        for rel in relas.iter_mut() {
            let address = symbols[elf64::Rela::bind(rel.r_info)].st_value as u32;
            for (idx, b) in address.to_le_bytes().to_vec().iter().enumerate() {
                let text_number: usize = self.get_section_number(".text");
                self.sections[text_number][rel.r_offset as usize + idx] = *b;
            }
        }
    }
    fn prepare_ehdr_for_staticlink(&mut self) {
        self.ehdr.e_type = elf64::ET_EXEC;
        self.ehdr.e_phoff = elf64::Ehdr::size() as u64; // sizeof(Ehdr)
        self.ehdr.e_phnum = 1;
        self.ehdr.e_phentsize = elf64::Phdr::size() as u16; // sizeof(Phdr)
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
    fn get_symbols(&self) -> Vec<elf64::Symbol> {
        let symbin: Vec<u8> = self.get_section(".symtab");
        let symbol_number: usize = symbin.len() / elf64::Symbol::size();
        let mut symbols: Vec<elf64::Symbol> = vec![elf64::init_nullsym()];
        for i in 0..symbol_number - 1 {
            symbols.push(elf64::Symbol::new_unsafe(
                symbin[(i + 1) * elf64::Symbol::size()..].to_vec(),
            ));
        }
        symbols
    }
    fn get_relas(&self, name: &str) -> Vec<elf64::Rela> {
        let relbin: Vec<u8> = self.get_section(name);
        let relas_number: usize = relbin.len() / elf64::Rela::size();
        let mut relas: Vec<elf64::Rela> = Vec::new();
        for i in 0..relas_number {
            relas.push(elf64::Rela::new_unsafe(
                relbin[i * elf64::Rela::size()..].to_vec(),
            ));
        }
        relas
    }
    fn init_phdr(&mut self) {
        let mut phdr: elf64::Phdr = elf64::init_phdr();
        phdr.p_type = elf64::PT_LOAD;
        phdr.p_offset = PAGE_SIZE;
        phdr.p_vaddr = BASE_ADDRESS;
        phdr.p_paddr = BASE_ADDRESS;
        phdr.p_align = PAGE_SIZE;
        let text: Vec<u8> = self.get_section(".text");
        phdr.p_filesz = text.len() as u64; // remove the hardcode
        phdr.p_memsz = text.len() as u64; // remove the hardcode
        phdr.p_flags = elf64::PF_R | elf64::PF_X | elf64::PF_W;
        self.phdrs = Some(vec![phdr]);
    }
    fn padding(&mut self) {
        for _ in 0..PAGE_SIZE - elf64::Ehdr::size() as u64 - elf64::Phdr::size() as u64 {
            self.sections[0].push(0x00);
        }
    }
    fn conditioning_section_offset(&mut self) {
        let _ = self
            .shdrs
            .iter_mut()
            .map(|shdr| shdr.sh_offset = PAGE_SIZE - elf64::Ehdr::size() as u64 + shdr.sh_offset)
            .collect::<()>();
    }
}
