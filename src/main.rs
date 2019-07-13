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
mod manager;
use manager::manager::Manager;
mod binary;
use binary::bytes::Bin;
mod elf;
use elf::ehdr::Ehdr;
use elf::section;
mod asm;

fn main() -> Result<(), Box<std::error::Error>> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches(); //フラグ･オプション引数等の利用はこのオブジェクトから
    let tokens: Vec<token::Token> = lex_phase(&matches);
    let mut manager: Manager = parse_phase(&matches, tokens);
    /* semantic phase */
    manager.env.semantic(&manager.nodes);
    if matches.is_present("dump-symbol") {
        println!("{}", "--------symbol_table--------".green().bold());
        println!("{}", "variables".green().bold());
        for (sym_name, symbol) in manager.env.var_table.iter() {
            println!("name:{}\tsym:{}", sym_name, symbol.string());
        }
    }
    /* generate-ir phase */
    manager.gen_ir(&matches);
    if matches.is_present("dump-ir") {
        println!("{}", "--------IR--------".green().bold());
        for i in &manager.irs {
            i.dump();
        }
    }
    /* generate-code phase */
    manager.gen_code(&matches);
    if matches.is_present("stop-s") {
        return Ok(());
    }
    //let bin: Bin = Bin::read_file("c.o");
    //let _ehdr: Ehdr = Ehdr::new(bin.b.into_inner());
    //ehdr.out();
    let atokens: Vec<asm::parse::AToken> = asm_lex_phase(&matches);
    let anodes: Vec<asm::parse::ANode> = asm_parse_phase(&matches, atokens);
    let text: Vec<u8> = asm::gen::generate(anodes);
    let mut bin: Bin = Bin::new((vec![], true));
    let shstrndx: Vec<u8> = section::build_shstrndx(vec![
        ".text".as_bytes().to_vec(),
        ".shstrndx".as_bytes().to_vec(),
    ]);
    let ehdr: Ehdr = gen_ehdr(0x40 + text.len() + shstrndx.len());
    bin.write(&ehdr.bin());
    bin.write(&text);
    bin.write(&shstrndx);
    bin.flush("c.o");
    Ok(())
}

fn lex_phase(matches: &clap::ArgMatches) -> Vec<token::Token> {
    let filecontent: String = drumatech::fileu::content_or_raw(matches.value_of("source").unwrap());
    if matches.is_present("dump-source") {
        println!("{}", "--------source--------".green().bold());
        let out = std::io::stdout(); //バッファリング
        let mut out = BufWriter::new(out.lock());
        writeln!(out, "{}", &filecontent).unwrap();
    }
    let tokens: Vec<token::Token> = lexing::lex_phase(filecontent); //字句解析結果のトークン列
    if matches.is_present("dump-token") {
        println!("{}", "--------tokens--------".green().bold());
        let out = std::io::stdout();
        let mut out = BufWriter::new(out.lock());
        for t in &tokens {
            writeln!(out, "{}", t.dump()).unwrap(); //トークン列ダンプ(デバッグ用)
        }
    }
    tokens
}

fn parse_phase(matches: &clap::ArgMatches, tokens: Vec<token::Token>) -> Manager {
    let nodes: Vec<node::Node> = parse::parser::parse(tokens);
    let manager: Manager = Manager {
        nodes: nodes,
        env: semantic::Environment::new(),
        irs: Vec::new(),
        nreg: 1,
        offset: 0,
        nlabel: 0,
    };
    if matches.is_present("dump-ast") {
        println!("{}", "--------AST--------".green().bold());
        let out = std::io::stdout();
        let mut out = BufWriter::new(out.lock());
        for n in &manager.nodes {
            writeln!(out, "{}", n.ty.dump()).unwrap();
        }
    };
    manager
}

fn asm_lex_phase(matches: &clap::ArgMatches) -> Vec<asm::parse::AToken> {
    let filecontent: String = drumatech::fileu::content_or_raw("c.s");
    if matches.is_present("dump-source") {
        println!("{}", "--------asmsource--------".green().bold());
        let out = std::io::stdout(); //バッファリング
        let mut out = BufWriter::new(out.lock());
        writeln!(out, "{}", &filecontent).unwrap();
    }
    let tokens: Vec<asm::parse::AToken> = asm::parse::lex_phase(filecontent); //字句解析結果のトークン列
    if matches.is_present("dump-asmtoken") {
        println!("{}", "--------asmtokens--------".green().bold());
        let out = std::io::stdout();
        let mut out = BufWriter::new(out.lock());
        for t in &tokens {
            writeln!(out, "{}", t.dump()).unwrap(); //トークン列ダンプ(デバッグ用)
        }
    }
    tokens
}

fn asm_parse_phase(
    matches: &clap::ArgMatches,
    tokens: Vec<asm::parse::AToken>,
) -> Vec<asm::parse::ANode> {
    let anodes: Vec<asm::parse::ANode> = asm::parse::parse(tokens);
    if matches.is_present("dump-asmast") {
        println!("{}", "--------asmAST--------".green().bold());
        let out = std::io::stdout();
        let mut out = BufWriter::new(out.lock());
        for n in &anodes {
            writeln!(out, "{}", n.ty.dump()).unwrap();
        }
    }
    anodes
}

fn gen_ehdr(shoff: usize) -> Ehdr {
    let mut bin: Vec<u8> = Vec::new();
    let mut e_ident: Vec<u8> = vec![
        0x7f, 0x45, 0x4c, 0x46, 0x02, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00,
    ];
    let mut e_type: Vec<u8> = vec![0x01, 0x00]; //ET_REL
    let mut e_machine: Vec<u8> = vec![0x3e, 0x00]; //amdx86-64
    let mut e_version: Vec<u8> = vec![0x01, 0x00, 0x00, 0x00]; //EV_CURRENT
    let mut e_entry: Vec<u8> = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    let mut e_phoff: Vec<u8> = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    let mut e_shoff: Vec<u8> = shoff.to_le_bytes().to_vec();
    let mut e_flags: Vec<u8> = vec![0x00, 0x00, 0x00, 0x00];
    let mut e_ehsize: Vec<u8> = vec![0x40, 0x00]; //64bytes
    let mut e_phentsize: Vec<u8> = vec![0x00, 0x00];
    let mut e_phnum: Vec<u8> = vec![0x00, 0x00];
    let mut e_shentsize: Vec<u8> = vec![0x40, 0x00]; //64bytes
    let mut e_shnum: Vec<u8> = vec![0x02, 0x00];
    let mut e_shstrndx: Vec<u8> = vec![0x01, 0x00];
    bin.append(&mut e_ident);
    bin.append(&mut e_type);
    bin.append(&mut e_machine);
    bin.append(&mut e_version);
    bin.append(&mut e_entry);
    bin.append(&mut e_phoff);
    bin.append(&mut e_shoff);
    bin.append(&mut e_flags);
    bin.append(&mut e_ehsize);
    bin.append(&mut e_phentsize);
    bin.append(&mut e_phnum);
    bin.append(&mut e_shentsize);
    bin.append(&mut e_shnum);
    bin.append(&mut e_shstrndx);
    Ehdr::new(bin)
}
