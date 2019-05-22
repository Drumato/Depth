extern crate yaml_rust;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter, Write};
use yaml_rust::{YamlEmitter, YamlLoader};

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

fn main() -> Result<(), Box<std::error::Error>> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let cfg = &get_yaml()?[0];
    /*
    let mut out_str = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut out_str);
        emitter.dump(cfg).unwrap(); // dump the YAML object to a String
    }
    println!("{}", out_str);
    */
    let tokens: Vec<token::Token> = lex_phase(matches);
    Ok(())
}

fn lex_phase(matches: clap::ArgMatches) -> Vec<token::Token> {
    let mut tokens: Vec<token::Token> = Vec::new();
    let filecontent: String = drumatech::fileu::content_or_raw(matches.value_of("source").unwrap());
    let mut lexer = lexing::Lexer::new(filecontent).unwrap();
    if matches.is_present("dump-source") {
        println!("{}", "--------source--------".green().bold());
        let out = std::io::stdout();
        let mut out = BufWriter::new(out.lock());
        writeln!(out, "{}", lexer.input).unwrap();
    }
    loop {
        let t: token::Token = lexer.next_token();
        tokens.push(t);
        if lexer.ch == 0 {
            break;
        }
    }
    if matches.is_present("dump-token") {
        println!("{}", "--------tokens--------".green().bold());
        let out = std::io::stdout();
        let mut out = BufWriter::new(out.lock());
        for t in &tokens {
            writeln!(out, "{}", t.dump()).unwrap();
        }
    }
    tokens
}

fn get_yaml() -> Result<Vec<yaml_rust::Yaml>, yaml_rust::ScanError> {
    let mut f = File::open("elf.yaml").expect("file not found");
    let mut fstr = String::new();
    f.read_to_string(&mut fstr)
        .expect("something went wront reading the file");
    YamlLoader::load_from_str(&fstr)
}
