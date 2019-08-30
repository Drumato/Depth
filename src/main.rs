extern crate yaml_rust;
#[macro_use]
extern crate clap;
use clap::App;

extern crate colored;
use colored::*;

use std::collections::HashMap;
mod compile;
use compile::frontend::lex;
use compile::frontend::parse::node;
use compile::frontend::parse::parser;
use compile::frontend::token::token as tok;
use compile::manager::manager::Manager;
mod object;
use object::elf;

fn main() -> Result<(), Box<std::error::Error>> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    if matches.is_present("elf") {
        match parse_elf(matches.value_of("source").unwrap()) {
            Ok(()) => (),
            Err(e) => eprintln!("{}", e),
        }
        std::process::exit(0);
    }
    let tokens: Vec<tok::Token> = lex_phase(&matches);
    let funcs: Vec<node::Func> = parse_phase(&matches, tokens);
    let mut manager: Manager = Manager {
        functions: funcs,
        hirs: Vec::new(),
        regnum: 0,
        labelnum: 0,
        stack_offset: 0,
        var_table: HashMap::new(),
    };
    manager.semantics();
    if matches.is_present("dump-symbol") {
        eprintln!("{}", "--------symbol_table--------".green().bold());
        for (_, symbol) in manager.var_table.iter() {
            eprintln!("{}", symbol.string());
        }
    }
    manager.gen_irs();
    if matches.is_present("dump-hir") {
        eprintln!("{}", "--------dumphir--------".blue().bold());
        for ir in manager.hirs.iter() {
            eprintln!("{}", ir.string().green().bold());
        }
    }
    genx64_phase(&matches, manager);
    Ok(())
}

fn lex_phase(matches: &clap::ArgMatches) -> Vec<tok::Token> {
    let filecontent: String = read_file(matches.value_of("source").unwrap());
    let tokens: Vec<tok::Token> = lex::lexing::lexing(filecontent);
    if matches.is_present("dump-token") {
        eprintln!("{}", "--------dumptoken--------".blue().bold());
        for t in tokens.iter() {
            eprintln!("{}", t.string().green().bold());
        }
    }
    tokens
}

fn parse_phase(matches: &clap::ArgMatches, tokens: Vec<tok::Token>) -> Vec<node::Func> {
    let funcs: Vec<node::Func> = parser::parsing(tokens);
    if matches.is_present("dump-ast") {
        eprintln!("{}", "--------dumpast--------".blue().bold());
        for f in funcs.iter() {
            eprintln!("{}'s stmts:", f.name);
            for n in f.stmts.iter() {
                eprintln!("{}", n.string().green().bold());
            }
        }
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
fn parse_elf(s: &str) -> Result<(), Box<std::error::Error>> {
    use std::fs::File;
    use std::io::Read;
    let mut file: File = File::open(s)?;
    let mut content = Vec::new();
    file.read_to_end(&mut content)?;
    elf::elf64::dump_elf_in_detail(content);
    Ok(())
}
