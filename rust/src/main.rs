extern crate yaml_rust;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Write};
use yaml_rust::{YamlEmitter, YamlLoader};

#[macro_use]
extern crate clap;
use clap::App;
mod lex;
use lex::token;

fn main() -> Result<(), Box<std::error::Error>> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let cfg = &get_yaml()?[0];
    let mut out_str = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut out_str);
        emitter.dump(cfg).unwrap(); // dump the YAML object to a String
    }
    println!("{}", out_str);
    println!("\n\n");
    let t: lex::token::Token =
        lex::token::new_token((lex::token::TokenType::TkIllegal, "$$$".to_string()));
    println!("{}", t.dump());
    Ok(())
}

fn get_yaml() -> Result<Vec<yaml_rust::Yaml>, yaml_rust::ScanError> {
    let mut f = File::open("elf.yaml").expect("file not found");
    let mut fstr = String::new();
    f.read_to_string(&mut fstr)
        .expect("something went wront reading the file");
    YamlLoader::load_from_str(&fstr)
}
