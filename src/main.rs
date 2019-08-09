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
mod asm;

extern crate genelf;
use elf::{Ehdr, Shdr, Symbol, ELF};
use genelf::elf::elf;
use std::fs::File;

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
    //let mut elf_file: ELF = gen_elf_phase(&matches);
    //let mut file = File::create("c.o")?;
    //file.write_all(&elf_file.bin())?;
    //file.flush()?;
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
fn gen_elf_phase(matches: &clap::ArgMatches) -> ELF {
    let mut elf_file: ELF = ELF::init();

    let strtab: Vec<&str> = vec!["main"];
    let shstrtab: Vec<&str> = vec![".text", ".strtab", ".symtab", ".shstrtab"];
    let atokens: Vec<asm::parse::AToken> = asm_lex_phase(&matches);
    let anodes: Vec<asm::parse::ANode> = asm_parse_phase(&matches, atokens);
    let text: Vec<u8> = asm::gen::generate(anodes);
    let main_sym: Symbol = Symbol::gen_main(text.len() as u64);
    elf_file.ehdr = gen_ehdr(0x40 + text.len() + 6 + 17 + main_sym.len());
    let text_shdr = gen_shdr(
        //.text
        1,
        elf::SHT_PROGBITS,
        text.len() as u64,
        elf::SHF_ALLOC | elf::SHF_EXECUTE,
        0x40,
    );
    text_shdr.out();
    elf_file.append_shdr(gen_nullhdr());
    elf_file.append_shdr(text_shdr);
    let strtab_shdr = gen_shdr(7, elf::SHT_STRTAB, 6, 0, (0x40 + text.len()) as u64);
    strtab_shdr.out();
    elf_file.append_shdr(strtab_shdr); //.strtab
    let symtab_shdr = gen_shdr(
        //.symtab
        15,
        elf::SHT_SYMTAB,
        main_sym.len() as u64,
        0,
        (0x40 + text.len() + 6) as u64,
    );
    elf_file.append_shdr(symtab_shdr);
    elf_file.append_shdr(gen_shdr(
        //.shstrtab
        23,
        elf::SHT_STRTAB,
        33,
        0,
        (0x40 + text.len() + 6 + main_sym.len()) as u64,
    ));
    elf_file.set_text(text);
    elf_file.set_strtab(strtab);
    elf_file.set_symtab(vec![main_sym]);
    elf_file.set_shstrtab(shstrtab);
    elf_file
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
    let mut e_shnum: Vec<u8> = vec![0x00, 0x00];
    let mut e_shstrndx: Vec<u8> = vec![0x00, 0x00];
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

fn gen_nullhdr() -> Shdr {
    Shdr::new(vec![
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ])
}
fn gen_shdr(ndx: u32, shtype: u32, size: u64, flags: u64, offset: u64) -> Shdr {
    let mut bin: Vec<u8> = Vec::new();
    let mut sh_name: Vec<u8> = ndx.to_le_bytes().to_vec();
    let mut sh_type: Vec<u8> = shtype.to_le_bytes().to_vec();
    let mut sh_size: Vec<u8> = size.to_le_bytes().to_vec();
    let mut sh_flags: Vec<u8> = flags.to_le_bytes().to_vec();
    let mut sh_info: Vec<u8> = vec![0x00, 0x00, 0x00, 0x00];
    let mut sh_link: Vec<u8> = vec![0x00, 0x00, 0x00, 0x00];
    let mut sh_addralign: Vec<u8> = vec![0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    let mut sh_entsize: Vec<u8> = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    let mut sh_offset: Vec<u8> = offset.to_le_bytes().to_vec();
    let mut sh_addr: Vec<u8> = vec![0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    bin.append(&mut sh_name);
    bin.append(&mut sh_type);
    bin.append(&mut sh_flags);
    bin.append(&mut sh_addr);
    bin.append(&mut sh_offset);
    bin.append(&mut sh_size);
    bin.append(&mut sh_link);
    bin.append(&mut sh_info);
    bin.append(&mut sh_addralign);
    bin.append(&mut sh_entsize);
    Shdr::new(bin)
}
