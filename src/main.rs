extern crate yaml_rust;
#[macro_use]
extern crate clap;
use clap::App;

extern crate colored;
use colored::*;

mod compile;
use compile::frontend as f;
use compile::manager::manager::Manager;
mod object;
use object::elf;
mod assemble;
use assemble as a;
mod ce;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};

fn main() -> Result<(), Box<std::error::Error>> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    compile(&matches);
    if matches.is_present("stop-c") {
        std::process::exit(0);
    }
    let mut elf_file: elf::elf64::ELF = assemble(&matches);
    let file_name;
    if matches.is_present("stop-a") {
        file_name = "c.o";
    } else {
        file_name = "a.out";
        elf_file.linking();
    }
    let mut writer = BufWriter::new(File::create(file_name).unwrap());
    match writer.write_all(&elf_file.to_vec()) {
        Ok(_) => (),
        Err(e) => eprintln!("{}", e),
    }
    match writer.flush() {
        Ok(_) => (),
        Err(e) => eprintln!("{}", e),
    }
    Ok(())
}
fn compile(matches: &clap::ArgMatches) {
    if !matches.value_of("source").unwrap().contains(".dep") {
        return;
    }
    let tokens: Vec<f::token::token::Token> = lex_phase(&matches);
    let funcs: Vec<f::parse::node::Func> = parse_phase(&matches, tokens);
    let mut manager: Manager = Manager::new(funcs);
    manager.semantics();
    if matches.is_present("dump-symbol") {
        manager.dump_symbol();
    }
    manager.gen_irs();
    if matches.is_present("dump-hir") {
        manager.dump_hir();
    }
    genx64_phase(&matches, manager);
}
fn assemble(matches: &clap::ArgMatches) -> elf::elf64::ELF {
    if !matches.value_of("source").unwrap().contains(".s") {
        std::process::exit(1);
    }
    let tokens: Vec<a::lex::Token> = a::lex::lexing(read_file(matches.value_of("source").unwrap()));
    let (instructions, info_map, relas) = a::parse::parsing(tokens);
    if matches.is_present("dump-inst") {
        dump_inst(&instructions, &info_map);
    }
    let (code_map, mut relas) = a::gen::generate(instructions, info_map, relas);
    let shstrtab: Vec<u8> = elf::elf64::strtab(vec![
        ".text",
        ".symtab",
        ".strtab",
        ".relatext",
        ".shstrtab",
    ]);
    let strs: Vec<&str> = code_map
        .iter()
        .map(|(name, _)| name.as_str())
        .collect::<Vec<&str>>();
    let mut symbols: Vec<elf::elf64::Symbol> = Vec::new();
    symbols.push(elf::elf64::init_nullsym());
    let mut total_len: u64 = 0;
    let mut total_code: Vec<u8> = Vec::new();
    let mut name: u32 = 1;
    for (idx, (symbol_name, codes)) in code_map.iter().enumerate() {
        symbols.push(elf::elf64::init_sym(
            name,
            elf::elf64::STB_GLOBAL,
            codes.len() as u64,
            total_len,
        ));
        name += symbol_name.len() as u32 + 1;
        total_len += codes.len() as u64;
        for b in codes.iter() {
            total_code.push(*b);
        }
        if let Some(rela) = relas.get_mut(symbol_name) {
            rela.r_info = (((idx + 1) << 32) + 4) as u64;
        }
    }
    let mut elf_file = elf::elf64::ELF {
        ehdr: elf::elf64::init_ehdr(),
        sections: vec![],
        shdrs: vec![],
        phdrs: None,
    };
    let strtab: Vec<u8> = elf::elf64::strtab(strs);
    elf_file.add_section(vec![], elf::elf64::init_nullhdr());
    elf_file.add_section(total_code, elf::elf64::init_texthdr(total_len));
    let symbol_length = symbols.len();
    let symtab: Vec<u8> = elf::elf64::symbols_to_vec(symbols);
    elf_file.add_section(
        symtab,
        elf::elf64::init_symtabhdr(24 * symbol_length as u64),
    );
    let strtab_length = strtab.len() as u64;
    elf_file.add_section(strtab, elf::elf64::init_strtabhdr(strtab_length));
    let relas_length = relas.len() as u64;
    elf_file.add_section(
        elf::elf64::relas_to_vec(relas.values().collect::<Vec<&elf::elf64::Rela>>()),
        elf::elf64::init_relahdr(24 * relas_length),
    );
    let shstrtab_length = shstrtab.len() as u64;
    elf_file.add_section(shstrtab, elf::elf64::init_strtabhdr(shstrtab_length));
    elf_file.condition();
    elf_file
}

fn lex_phase(matches: &clap::ArgMatches) -> Vec<f::token::token::Token> {
    let filecontent: String = read_file(matches.value_of("source").unwrap());
    let tokens: Vec<f::token::token::Token> = f::lex::lexing::lexing(filecontent);
    if matches.is_present("dump-token") {
        eprintln!("{}", "--------dumptoken--------".blue().bold());
        for t in tokens.iter() {
            eprintln!("{}", t.string().green().bold());
        }
    }
    tokens
}

fn parse_phase(
    matches: &clap::ArgMatches,
    tokens: Vec<f::token::token::Token>,
) -> Vec<f::parse::node::Func> {
    let funcs: Vec<f::parse::node::Func> = f::parse::parser::parsing(tokens);
    if matches.is_present("dump-ast") {
        f::parse::node::dump_ast(&funcs);
    }
    funcs
}
fn genx64_phase(matches: &clap::ArgMatches, manager: Manager) {
    if matches.is_present("intel") {
        println!(".intel_syntax noprefix");
        println!(".global main");
    }
    manager.genx64();
}

fn read_file(s: &str) -> String {
    use std::fs;
    use std::path::Path;
    let filepath: &Path = Path::new(s);
    if filepath.exists() && filepath.is_file() {
        return fs::read_to_string(s).unwrap();
    }
    if filepath.is_dir() {
        eprintln!("{} is directory.", filepath.to_str().unwrap());
    }
    s.to_string()
}
fn dump_inst(
    instructions: &std::collections::HashMap<
        std::string::String,
        std::vec::Vec<assemble::parse::Inst>,
    >,
    info_map: &std::collections::HashMap<usize, assemble::parse::Info>,
) {
    for (symbol, v) in instructions.iter() {
        eprintln!("{}'s instructions", symbol.bold().green());
        for inst in v.iter() {
            let num: &usize = match inst {
                a::parse::Inst::BINARG(num)
                | a::parse::Inst::UNARG(num)
                | a::parse::Inst::NOARG(num) => num,
            };
            let info: &a::parse::Info = info_map.get(num).unwrap();
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
