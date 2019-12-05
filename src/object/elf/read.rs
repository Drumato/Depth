extern crate colored;
use super::elf64;

impl elf64::ELF {
    pub fn read_elf(file_path: &String) -> Self {
        let mut elf_file = Self::init();
        let binary: Vec<u8> = read_binary(file_path);
        elf_file.ehdr = elf64::Ehdr::new_unsafe(binary[0..63].to_vec());

        /* setup section header table */
        let shdrs: Vec<elf64::Shdr> = Self::build_shdrs(
            &elf_file.ehdr,
            binary[elf_file.ehdr.e_shoff as usize..].to_vec(),
        );
        let length: usize = shdrs.len();
        for idx in 0..length {
            let sh_name: String = Self::collect_name(
                binary[shdrs[elf_file.ehdr.e_shstrndx as usize].sh_offset as usize
                    + shdrs[idx].sh_name as usize..]
                    .to_vec(),
            );
            let offset = shdrs[idx].sh_offset as usize;
            let size = shdrs[idx].sh_size as usize;
            elf_file.add_section(
                binary[offset..offset + size].to_vec(),
                shdrs[idx].clone(),
                &sh_name,
            );
        }

        /* setup program header table */
        let phdrs: Vec<elf64::Phdr> = Self::build_phdrs(
            &elf_file.ehdr,
            binary[elf_file.ehdr.e_phoff as usize..].to_vec(),
        );
        elf_file.phdrs = Some(phdrs);
        elf_file.condition();
        return elf_file;
    }
    fn build_phdrs(ehdr: &elf64::Ehdr, binary: Vec<u8>) -> Vec<elf64::Phdr> {
        let mut phdrs: Vec<elf64::Phdr> = Vec::new();
        for i in 0..ehdr.e_phnum {
            phdrs.push(elf64::Phdr::new_unsafe(
                binary[(i * ehdr.e_phentsize) as usize..].to_vec(),
            ));
        }
        phdrs
    }
    fn build_shdrs(ehdr: &elf64::Ehdr, binary: Vec<u8>) -> Vec<elf64::Shdr> {
        let mut shdrs: Vec<elf64::Shdr> = Vec::new();
        for i in 0..ehdr.e_shnum {
            shdrs.push(elf64::Shdr::new_unsafe(
                binary[(i * ehdr.e_shentsize) as usize..].to_vec(),
            ));
        }
        shdrs
    }
    fn collect_name(binary: Vec<u8>) -> String {
        binary
            .iter()
            .take_while(|b| *b != &0x00)
            .map(|b| *b as char)
            .collect::<String>()
    }
    //fn build_sections()
}

fn read_binary(file_path: &String) -> Vec<u8> {
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;
    let filepath: &Path = Path::new(file_path);

    if filepath.exists() && filepath.is_file() {
        let mut file = File::open(file_path).unwrap();
        let mut buf = Vec::new();
        let _ = file.read_to_end(&mut buf).unwrap();
        return buf;
    }
    Vec::new()
}
