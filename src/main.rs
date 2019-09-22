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
mod link;

fn main() -> Result<(), Box<std::error::Error>> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let assembler_code: String = compile(&matches);
    if matches.is_present("stop-c") {
        let mut file = File::create(
            matches
                .value_of("source")
                .unwrap()
                .split(".")
                .collect::<Vec<&str>>()[0]
                .to_string()
                + ".s",
        )?;
        file.write_all(assembler_code.as_bytes())?;
        std::process::exit(0);
    }
    let mut elf_file: elf::elf64::ELF = assemble(&matches, assembler_code);
    let file_name;
    if matches.is_present("stop-a") {
        file_name = matches
            .value_of("source")
            .unwrap()
            .split(".")
            .collect::<Vec<&str>>()[0]
            .to_string()
            + ".o";
    } else {
        file_name = "a.out".to_string();
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
fn compile(matches: &clap::ArgMatches) -> String {
    if !matches.value_of("source").unwrap().contains(".dep") {
        return read_file(matches.value_of("source").unwrap());
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
    genx64_phase(&matches, manager)
}
fn assemble(matches: &clap::ArgMatches, mut assembler_code: String) -> elf::elf64::ELF {
    if !matches.is_present("stop-a") {
        assembler_code += "_start:\n  call main\n  mov rdi, rax\n  mov rax, 60\n  syscall\n";
    }
    if matches.value_of("source").unwrap().contains(".o") {
        //return read_file(matches.value_of("source").unwrap());
    }
    let tokens: Vec<a::lex::Token> = a::lex::lexing(assembler_code);
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
    let mut symbols: Vec<elf::elf64::Symbol> = Vec::with_capacity(100);
    symbols.push(elf::elf64::init_nullsym());
    let mut total_len: u64 = 0;
    let mut total_code: Vec<u8> = Vec::with_capacity(2048);
    let mut name: u32 = 1;
    for (idx, (symbol_name, codes)) in code_map.iter().enumerate() {
        if codes.len() != 0 {
            symbols.push(elf::elf64::init_sym(
                name,
                elf::elf64::STB_GLOBAL,
                codes.len() as u64,
                total_len,
            ));
        } else {
            symbols.push(elf::elf64::init_refsym(name, elf::elf64::STB_GLOBAL));
        }
        name += symbol_name.len() as u32 + 1;
        total_len += codes.len() as u64;
        for b in codes.iter() {
            total_code.push(*b);
        }
        if let Some(rela) = relas.get_mut(symbol_name) {
            rela.r_info = (((idx + 1) << 32) + 1) as u64;
        }
    }
    let mut elf_file = elf::elf64::ELF {
        ehdr: elf::elf64::init_ehdr(),
        sections: vec![],
        shdrs: vec![],
        phdrs: None,
        names: HashMap::new(),
    };
    let strtab: Vec<u8> = elf::elf64::strtab(strs);
    elf_file.add_section(vec![], elf::elf64::init_nullhdr(), "null");
    elf_file.add_section(total_code, elf::elf64::init_texthdr(total_len), ".text");
    let symbol_length = symbols.len();
    let symtab: Vec<u8> = elf::elf64::symbols_to_vec(symbols);
    elf_file.add_section(
        symtab,
        elf::elf64::init_symtabhdr(24 * symbol_length as u64),
        ".symtab",
    );
    let strtab_length = strtab.len() as u64;
    elf_file.add_section(strtab, elf::elf64::init_strtabhdr(strtab_length), ".strtab");
    let relas_length = relas.len() as u64;
    elf_file.add_section(
        elf::elf64::relas_to_vec(relas.values().collect::<Vec<&elf::elf64::Rela>>()),
        elf::elf64::init_relahdr(24 * relas_length),
        ".relatext",
    );
    let shstrtab_length = shstrtab.len() as u64;
    elf_file.add_section(
        shstrtab,
        elf::elf64::init_strtabhdr(shstrtab_length),
        ".shstrtab",
    );
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
fn genx64_phase(matches: &clap::ArgMatches, mut manager: Manager) -> String {
    let mut assembler_code: String = manager.genx64();
    if matches.is_present("intel") {
        assembler_code = ".global main\n".to_string() + assembler_code.as_str();
        assembler_code = ".intel_syntax noprefix\n".to_string() + assembler_code.as_str();
    }
    assembler_code
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
    instructions: &std::collections::BTreeMap<
        std::string::String,
        std::vec::Vec<assemble::parse::Inst>,
    >,
    info_map: &std::collections::BTreeMap<usize, assemble::parse::Info>,
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
