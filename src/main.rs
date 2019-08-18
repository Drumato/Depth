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
    let nodes: Vec<node::Node> = parse_phase(&matches, tokens);
    let manager: Manager = genir_phase(&matches, nodes);

    if matches.is_present("intel") {
        println!(".intel_syntax noprefix");
        println!(".global main");
    }
    println!("main:");
    manager.genx64();

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

fn parse_phase(matches: &clap::ArgMatches, tokens: Vec<tok::Token>) -> Vec<node::Node> {
    let nodes: Vec<node::Node> = parser::parsing(tokens);
    if matches.is_present("dump-ast") {
        eprintln!("{}", "--------dumpast--------".blue().bold());
        for n in nodes.iter() {
            eprintln!("{}", n.string().green().bold());
        }
    }
    nodes
}
fn genir_phase(matches: &clap::ArgMatches, nodes: Vec<node::Node>) -> Manager {
    let manager: Manager = hi::gen_hir(nodes);
    if matches.is_present("dump-hir") {
        eprintln!("{}", "--------dumphir--------".blue().bold());
        for ir in manager.hirs.iter() {
            eprintln!("{}", ir.string().green().bold());
        }
    }
    manager
}
