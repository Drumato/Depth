extern crate yaml_rust;
use std::io::{BufWriter, Write};

#[macro_use]
extern crate clap;
use clap::App;
mod lex;
use lex::lexing;
use lex::token;
//mod binary;
//use binary::bytes;
extern crate drumatech;

extern crate colored;
use colored::*;

mod parse;
use parse::node;
mod analysis;
use analysis::semantic;

pub struct Manager {
    nodes: Vec<node::Node>,
    env: semantic::Environment,
}

fn main() -> Result<(), Box<std::error::Error>> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    /*
    let mut out_str = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut out_str);
        emitter.dump(cfg).unwrap(); // dump the YAML object to a String
    }
    println!("{}", out_str);
    */
    let filecontent: String = drumatech::fileu::content_or_raw(matches.value_of("source").unwrap());
    if matches.is_present("dump-source") {
        println!("{}", "--------source--------".green().bold());
        let out = std::io::stdout();
        let mut out = BufWriter::new(out.lock());
        writeln!(out, "{}", &filecontent).unwrap();
    }
    let mut tokens: Vec<token::Token> = lexing::lex_phase(filecontent);
    if matches.is_present("dump-token") {
        println!("{}", "--------tokens--------".green().bold());
        let out = std::io::stdout();
        let mut out = BufWriter::new(out.lock());
        for t in &tokens {
            writeln!(out, "{}", t.dump()).unwrap();
        }
    }
    let mut manager: Manager = parse_phase(&matches, tokens);
    if matches.is_present("dump-ast") {
        println!("{}", "--------AST--------".green().bold());
        let out = std::io::stdout();
        let mut out = BufWriter::new(out.lock());
        for n in &manager.nodes {
            writeln!(out, "{}", n.ty.dump()).unwrap();
        }
    }
    manager.env.semantic(manager.nodes);
    if matches.is_present("dump-symbol") {
        println!("{}", "--------symbol_tables--------".green().bold());
        println!("{}", "variables".green().bold());
        for (sym_name, symbol) in manager.env.var_tables.iter() {
            println!("name:{}\tsym:{}", sym_name, symbol.string());
        }
    }
    Ok(())
}

fn parse_phase(matches: &clap::ArgMatches, tokens: Vec<token::Token>) -> Manager {
    let nodes: Vec<node::Node> = parse::parser::parse(tokens);
    Manager {
        nodes: nodes,
        env: semantic::Environment::new(),
    }
}
