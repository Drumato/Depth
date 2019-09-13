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
    if matches.is_present("stop-s") {
        std::process::exit(0);
    }
    assemble(&matches);
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
fn assemble(matches: &clap::ArgMatches) {
    if !matches.value_of("source").unwrap().contains(".s") {
        return;
    }
    let tokens: Vec<a::lex::Token> = a::lex::lexing(read_file(matches.value_of("source").unwrap()));
    let (instructions, info_map) = a::parse::parsing(tokens);
    if matches.is_present("dump-inst") {
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
                    "  {} {} {}",
                    info.inst_name.bold().blue(),
                    lop_string,
                    rop_string
                );
            }
        }
    }
    let code_map: HashMap<String, Vec<u8>> = a::gen::generate(instructions, info_map);
    let shstrtab: Vec<u8> = elf::elf64::strtab(vec![".text", ".symtab", ".strtab", ".shstrtab"]);
    let mut strs: Vec<&str> = Vec::new();
    let mut symbols: Vec<elf::elf64::Symbol> = Vec::new();
    let mut total_len: u64 = 0;
    let mut total_code: Vec<u8> = Vec::new();
    let mut name: u32 = 1;
    for (symbol_name, codes) in code_map.iter() {
        strs.push(symbol_name.as_str());
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
    }
    let strtab: Vec<u8> = elf::elf64::strtab(strs);
    let symbol_number = symbols.len();
    let symtab: Vec<u8> = elf::elf64::symbols_to_vec(symbols);
    let main_hdr = elf::elf64::init_mainhdr(total_len);
    let symtab_hdr = elf::elf64::init_symtabhdr(24 * symbol_number as u64);
    let strtab_hdr = elf::elf64::init_strtabhdr(strtab.len() as u64);
    let shstrtab_hdr = elf::elf64::init_shstrtabhdr(shstrtab.len() as u64);
    let ehdr: elf::elf64::Ehdr = elf::elf64::init_ehdr();
    let mut writer = BufWriter::new(File::create("c.o").unwrap());
    let mut elf_file = elf::elf64::ELF {
        ehdr: ehdr,
        sections: vec![total_code, symtab, strtab, shstrtab],
        shdrs: vec![
            elf::elf64::init_nullhdr(),
            main_hdr,
            symtab_hdr,
            strtab_hdr,
            shstrtab_hdr,
        ],
        phdrs: None,
    };
    elf_file.condition();
    match writer.write_all(&elf_file.to_vec()) {
        Ok(_) => (),
        Err(e) => eprintln!("{}", e),
    }
    match writer.flush() {
        Ok(_) => (),
        Err(e) => eprintln!("{}", e),
    }
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
