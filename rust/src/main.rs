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
mod binary;
use binary::bytes;

extern crate colored;
use colored::*;

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
    if matches.is_present("dump-token") {
        println!("{}", "--------tokens--------".green().bold());
        /* sample of buffering
         * let out = stdout();
         * let mut out = BufWriter::new(out.lock());
         * for t in &tokens{
         *   writeln!(out,t.dump()).unwrap();
         * }
         */
    }
    //let t = token::Token::new((token::TokenType::TkIntlit, "30".to_string(), 30));
    //println!("{}", t.dump());
    let x: Vec<u8> = vec![
        //sample
        0x7f, 0x45, 0x4c, 0x46, 0x02, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0000,
        0x01, 0x00, 0x3e, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x02, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x00, 0x0b,
        0x00, 0x0a, 0x00,
    ];
    let b = bytes::Binaryen::new((x, false));
    b.flush("sample.o")?;
    Ok(())
}

fn get_yaml() -> Result<Vec<yaml_rust::Yaml>, yaml_rust::ScanError> {
    let mut f = File::open("elf.yaml").expect("file not found");
    let mut fstr = String::new();
    f.read_to_string(&mut fstr)
        .expect("something went wront reading the file");
    YamlLoader::load_from_str(&fstr)
}
