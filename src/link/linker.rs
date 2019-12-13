use super::super::object::elf::elf64 as e;
use e::{Rela, Symbol, ELF};
use std::collections::HashMap;
pub struct Linker {
    pub objs: Vec<ELF>,
}

impl Linker {
    pub fn linking(elf_files: Vec<ELF>) -> ELF {
        let linker: Linker = Self::new(elf_files);
        let mut exec_file: ELF = linker.build_binary();
        exec_file.condition();
        exec_file.linking();
        exec_file
    }
    fn build_binary(&self) -> ELF {
        let mut exec_file: ELF = ELF::init();
        self.build_null(&mut exec_file);
        self.build_text(&mut exec_file);
        self.build_symtab(&mut exec_file);
        self.build_strtab(&mut exec_file);
        self.build_relatext(&mut exec_file);
        self.build_debug(&mut exec_file);
        self.build_shstrtab(&mut exec_file);
        self.condition_symbols(&mut exec_file);
        exec_file
    }
    fn build_null(&self, exec_file: &mut ELF) {
        exec_file.add_section(vec![], e::init_nullhdr(), "null");
    }
    fn build_text(&self, exec_file: &mut ELF) {
        let text: Vec<u8> = self.conbine_vec(self.get_sections(".text"));
        let total_len: u64 = text.len() as u64;
        exec_file.add_section(text, e::init_texthdr(total_len), ".text");
    }
    fn build_symtab(&self, exec_file: &mut ELF) {
        let symtabs: Vec<Vec<u8>> = self.get_sections(".symtab");
        let mut symbols: Vec<Symbol> = Vec::new();
        for (idx, _tab) in symtabs.iter().enumerate() {
            for sym in self.objs[idx].get_symbols().iter() {
                if idx != 0 && sym.st_shndx == 0 && sym.st_info == 0 {
                    continue;
                }
                symbols.push(sym.to_owned());
            }
        }
        let symtab: Vec<u8> = e::symbols_to_vec(symbols);
        let symtab_number: u64 = symtab.len() as u64 / Symbol::size() as u64;
        exec_file.add_section(
            symtab,
            e::init_symtabhdr(Symbol::size() as u64 * symtab_number as u64),
            ".symtab",
        );
    }
    fn build_strtab(&self, exec_file: &mut ELF) {
        let strtab: Vec<u8> = self.conbine_vec(self.get_sections(".strtab"));
        let strtab_len: u64 = strtab.len() as u64;
        exec_file.add_section(strtab, e::init_strtabhdr(strtab_len), ".strtab");
    }
    fn build_relatext(&self, exec_file: &mut ELF) {
        let relatext: Vec<u8> = self.conbine_vec(self.get_sections(".rela.text"));
        let rela_number: u64 = relatext.len() as u64 / Rela::size() as u64;
        exec_file.add_section(
            relatext,
            e::init_relahdr(Rela::size() as u64 * rela_number),
            ".rela.text",
        );
    }
    fn build_debug(&self, exec_file: &mut ELF) {
        let debug_table: Vec<u8> = self.conbine_vec(self.get_sections(".dbg.depth"));
        let total_len: u64 = debug_table.len() as u64;
        exec_file.add_section(debug_table, e::init_debughdr(total_len), ".dbg.depth");
    }
    fn build_shstrtab(&self, exec_file: &mut ELF) {
        let shstrtab: Vec<u8> = e::strtab(vec![
            ".text",
            ".symtab",
            ".strtab",
            ".rela.text",
            ".dbg.debug",
            ".shstrtab",
        ]);
        let shstrtab_len: u64 = shstrtab.len() as u64;
        exec_file.add_section(shstrtab, e::init_strtabhdr(shstrtab_len), ".shstrtab");
    }
    fn get_sections(&self, name: &str) -> Vec<Vec<u8>> {
        self.objs
            .iter()
            .map(|elf| elf.get_section(name))
            .collect::<Vec<Vec<u8>>>()
    }
    fn condition_symbols(&self, exec_file: &mut ELF) {
        let strtabs: Vec<Vec<u8>> = self.get_sections(".strtab");
        let mut offsets: Vec<usize> = vec![0; self.objs.len()];
        for (idx, s) in strtabs.iter().enumerate() {
            if idx == 0 {
                offsets[idx] = 0;
            } else {
                offsets[idx] = s.len();
            }
        }
        let mut symtab: Vec<Symbol> = exec_file.get_symbols();
        let mut symap: HashMap<String, Symbol> = HashMap::new();
        let mut fnum = 0;
        for sym in symtab.iter_mut() {
            if sym.st_name != 0 {
                sym.st_name += offsets[fnum - 1] as u32;
            } else {
                fnum += 1;
                continue;
            }
            let name: String = strtabs[fnum - 1]
                [(sym.st_name as usize - offsets[fnum - 1]) as usize..]
                .to_vec()
                .iter()
                .take_while(|b| *b != &00)
                .map(|c| *c as char)
                .collect();
            if let Some(s) = symap.get(&name) {
                sym.st_value = s.st_value;
            } else {
                symap.insert(name, sym.clone());
            }
        }
        let symtab_number: usize = exec_file.get_section_number(".symtab");
        exec_file.sections[symtab_number] = e::symbols_to_vec(symtab);
    }
    fn conbine_vec(&self, mut vecvec: Vec<Vec<u8>>) -> Vec<u8> {
        let mut total: Vec<u8> = Vec::new();
        for vec in vecvec.iter_mut() {
            total.append(vec);
        }
        total
    }
    fn new(elf_files: Vec<ELF>) -> Self {
        Self { objs: elf_files }
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
    fn resolve_symbols(&mut self) {
        let symbols: Vec<e::Symbol> = self.get_symbols();
        let mut relas: Vec<e::Rela> = self.get_relas(".rela.text");
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
    fn padding(&mut self) {
        for _ in 0..PAGE_SIZE - e::Ehdr::size() as u64 - e::Phdr::size() as u64 {
            self.sections[0].push(0x00);
        }
    }
    fn conditioning_section_offset(&mut self) {
        let _ = self
            .shdrs
            .iter_mut()
            .map(|shdr| shdr.sh_offset = PAGE_SIZE - e::Ehdr::size() as u64 + shdr.sh_offset)
            .collect::<()>();
    }
}
