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
use analysis::{ir, semantic};

//ファイル単位で存在させる(予定の)構造体
pub struct Manager {
    nodes: Vec<node::Node>,
    irs: Vec<ir::IR>,
    env: semantic::Environment,
}

impl Manager {
    fn gen_ir(mut self, matches: &clap::ArgMatches) {
        self.irs = ir::generate_ir(self.nodes);
    }
}

fn main() -> Result<(), Box<std::error::Error>> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches(); //フラグ･オプション引数等の利用はこのオブジェクトから
                                                      /*
                                                      let mut out_str = String::new();
                                                      {
                                                          let mut emitter = YamlEmitter::new(&mut out_str);
                                                          emitter.dump(cfg).unwrap(); // dump the YAML object to a String
                                                      }
                                                      println!("{}", out_str);
                                                      */
    let tokens: Vec<token::Token> = lex_phase(&matches);
    let mut manager: Manager = parse_phase(&matches, tokens);
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
