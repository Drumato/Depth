pub mod gen;
pub mod lex;
pub mod parse;

extern crate clap;
extern crate colored;
use colored::*;

use crate::compile;
use crate::object;
use object::elf::elf64::ELF;

pub fn assemble(
    assembler_code: String,
    matches: &clap::ArgMatches,
    debug_funcs: Vec<compile::frontend::parse::node::Func>,
) -> ELF {
    use object::elf::elf64;

    /* tokenize */
    let tokens: Vec<lex::Token> = lex::lexing(assembler_code);

    /* parse */
    let (instructions, info_map, relas) = parse::parsing(tokens);
    if matches.is_present("dump-inst") {
        dump_inst(&instructions, &info_map);
    }

    let (code_map, mut relas) = gen::generate(instructions, info_map, relas);

    let shstrtab = build_shstrtab();

    /* build symbol-names from map */
    let symbol_names = code_map
        .iter()
        .map(|(name, _)| name.as_str())
        .collect::<Vec<&str>>();

    /* initialize with null symbol. */
    let mut symbols: Vec<elf64::Symbol> = vec![elf64::init_nullsym()];

    let total_len: u64 = sum_all_code_length(&code_map);
    let mut total_code: Vec<u8> = Vec::with_capacity(2048);

    /* initialize string-index with null byte. */
    let mut name: u32 = 1;
    for (idx, (symbol_name, codes)) in code_map.iter().enumerate() {
        if codes.len() != 0 {
            symbols.push(elf64::init_sym(
                name,
                elf64::STB_GLOBAL,
                codes.len() as u64,
                total_code.len() as u64,
            ));
        } else {
            symbols.push(elf64::init_refsym(name, elf64::STB_GLOBAL));
        }

        /* progress the index with null byte. */
        name += symbol_name.len() as u32 + 1;

        /* merge binaries into one vector. */
        for b in codes.iter() {
            total_code.push(*b);
        }
        if let Some(rela) = relas.get_mut(symbol_name) {
            rela.r_info = (((idx + 1) << 32) + 1) as u64;
        }
    }

    let mut elf_file = ELF::init();

    /* add all-sections. */
    let strtab: Vec<u8> = elf64::strtab(symbol_names);
    elf_file.add_section(vec![], elf64::init_nullhdr(), "null");

    /* .text */
    elf_file.add_section(total_code, elf64::init_texthdr(total_len), ".text");

    /* .symtab */
    let symbol_length = symbols.len();
    let symtab: Vec<u8> = elf64::symbols_to_vec(symbols);
    let symtab_size = elf64::Symbol::size() as u64 * symbol_length as u64;
    elf_file.add_section(symtab, elf64::init_symtabhdr(symtab_size), ".symtab");

    /* .strtab */
    let strtab_length = strtab.len() as u64;
    elf_file.add_section(strtab, elf64::init_strtabhdr(strtab_length), ".strtab");

    /* .rela.text */
    let relas_length = relas.len() as u64;
    let relas_tab = elf64::relas_to_vec(relas.values().collect::<Vec<&elf64::Rela>>());
    let relas_size = elf64::Rela::size() as u64 * relas_length;
    elf_file.add_section(relas_tab, elf64::init_relahdr(relas_size), ".rela.text");

    /* .dbg.depth */
    let debug_section_binary =
        object::debug::build_debug_information(&elf_file, debug_funcs.clone());
    let debug_length = debug_section_binary.len();
    elf_file.add_section(
        debug_section_binary,
        elf64::init_debughdr(debug_length as u64),
        ".dbg.depth",
    );

    /* .documents */
    let documents_binary = object::debug::build_documents(debug_funcs);
    let documents_length = documents_binary.len();
    elf_file.add_section(
        documents_binary,
        elf64::init_documenthdr(documents_length as u64),
        ".documents",
    );
    /* .shstrtab */
    let shstrtab_length = shstrtab.len() as u64;
    let shstrtab_hdr = elf64::init_strtabhdr(shstrtab_length);
    elf_file.add_section(shstrtab, shstrtab_hdr, ".shstrtab");

    elf_file.condition();
    elf_file
}

fn dump_inst(
    instructions: &std::collections::BTreeMap<std::string::String, std::vec::Vec<parse::Inst>>,
    info_map: &std::collections::BTreeMap<usize, parse::Info>,
) {
    for (symbol, v) in instructions.iter() {
        eprintln!("{}'s instructions", symbol.bold().green());
        for inst in v.iter() {
            let num: &usize = match inst {
                parse::Inst::BINARG(num) | parse::Inst::UNARG(num) | parse::Inst::NOARG(num) => num,
                parse::Inst::LABEL(num, _) => num,
            };
            if let Some(info) = info_map.get(num) {
                let lop_string: String = match &info.lop {
                    Some(l) => l.string(),
                    None => "".to_string(),
                };
                let rop_string: String = match &info.rop {
                    Some(r) => r.string(),
                    None => "".to_string(),
                };
                eprintln!(
                    "  instname->'{}' lop->'{}' rop->'{}' ",
                    info.inst_name.bold().blue(),
                    lop_string.green(),
                    rop_string.green(),
                );
            }
        }
    }
}

fn build_shstrtab() -> Vec<u8> {
    use object::elf::elf64;
    elf64::strtab(vec![
        ".text",
        ".symtab",
        ".strtab",
        ".rela.text",
        ".dbg.depth",
        ".documents",
        ".shstrtab",
    ])
}

fn sum_all_code_length(
    code_map: &std::collections::BTreeMap<std::string::String, std::vec::Vec<u8>>,
) -> u64 {
    code_map
        .iter()
        .map(|(_, codes)| codes.len() as u64)
        .sum::<u64>()
}
