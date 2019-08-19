extern crate yaml_rust;
#[macro_use]
extern crate clap;
use clap::App;
extern crate drumatech;

extern crate colored;
use colored::*;

mod token;
use token::token as tok;
mod lex;
mod parse;
use parse::{node, parser};
mod ir;
use ir::hi;
mod manager;
use manager::manager::Manager;

fn main() -> Result<(), Box<std::error::Error>> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let tokens: Vec<tok::Token> = lex_phase(&matches);
    let mut funcs: Vec<node::Func> = parse_phase(&matches, tokens);
    //Manager::semantics(&mut nodes);
    let manager: Manager = genir_phase(&matches, funcs);
    genx64_phase(&matches, manager);

    Ok(())
}

fn lex_phase(matches: &clap::ArgMatches) -> Vec<tok::Token> {
    let filecontent: String = drumatech::fileu::content_or_raw(matches.value_of("source").unwrap());
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
fn genir_phase(matches: &clap::ArgMatches, funcs: Vec<node::Func>) -> Manager {
    let mut manager: Manager = Manager {
        functions: funcs,
        hirs: Vec::new(),
        regnum: 0,
    };
    manager.gen_irs();
    if matches.is_present("dump-hir") {
        eprintln!("{}", "--------dumphir--------".blue().bold());
        for ir in manager.hirs.iter() {
            eprintln!("{}", ir.string().green().bold());
        }
    }
    manager
}
fn genx64_phase(matches: &clap::ArgMatches, manager: Manager) {
    if matches.is_present("intel") {
        println!(".intel_syntax noprefix");
        println!(".global main");
    }
    manager.genx64();
}
