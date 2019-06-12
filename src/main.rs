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
    //let lexer: lexing::Lexer = lex_phase(&matches);
    let mut manager: Manager = parse_phase(&matches);
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
        for (sym_name, symbol) in manager.env.sym_tables.iter() {
            println!("name:{}\tsym:{}", sym_name, symbol.string());
        }
    }
    Ok(())
}

fn parse_phase(matches: &clap::ArgMatches) -> Manager {
    let filecontent: String = drumatech::fileu::content_or_raw(matches.value_of("source").unwrap());
    let mut lexer = lexing::Lexer::new(filecontent).unwrap();
    if matches.is_present("dump-source") {
        println!("{}", "--------source--------".green().bold());
        let out = std::io::stdout();
        let mut out = BufWriter::new(out.lock());
        writeln!(out, "{}", lexer.input).unwrap();
    }
    if matches.is_present("dump-token") {
        let mut tokens: Vec<token::Token> = Vec::new();
        loop {
            let t: token::Token = lexer.next_token();
            tokens.push(t);
            if lexer.ch == 0 {
                break;
            }
        }
        println!("{}", "--------tokens--------".green().bold());
        let out = std::io::stdout();
        let mut out = BufWriter::new(out.lock());
        for t in tokens {
            writeln!(out, "{}", t.dump()).unwrap();
        }
        lexer = lexing::Lexer::new(drumatech::fileu::content_or_raw(
            matches.value_of("source").unwrap(),
        ))
        .unwrap();
    }
    let nodes: Vec<node::Node> = parse::parser::parse(lexer);
    Manager {
        nodes: nodes,
        env: semantic::Environment::new(),
    }
}
