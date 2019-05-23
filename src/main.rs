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

mod parse;
use parse::{node, parser};

struct Manager {
    nodes: Vec<node::Node>,
}

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
    Manager { nodes }
}

fn get_yaml() -> Result<Vec<yaml_rust::Yaml>, yaml_rust::ScanError> {
    let mut f = File::open("elf.yaml").expect("file not found");
    let mut fstr = String::new();
    f.read_to_string(&mut fstr)
        .expect("something went wront reading the file");
    YamlLoader::load_from_str(&fstr)
}

/*
#[macro_use]
extern crate lazy_static;
mod syntax {
    use super::parse::lr::SyntaxDefinition;
    use super::token;
    lazy_static! {
        pub static ref SYNTAXDEFINITIONS: [SyntaxDefinition; 8] = [SyntaxDefinition {
            /* E -> E + T */
            ltoken: token::Token::new((
                token::TokenType::TkExp,
                String::from("EXP"),
                token::TokenVal::InVal
            )),
            pattern: vec![
                token::Token::new((
                    token::TokenType::TkExp,
                    String::from("EXP"),
                    token::TokenVal::InVal
                )),
                token::Token::new((
                    token::TokenType::TkPlus,
                    String::from("+"),
                    token::TokenVal::InVal
                )),
                token::Token::new((
                    token::TokenType::TkTerm,
                    String::new(),
                    token::TokenVal::InVal
                )),
            ],
        },
        SyntaxDefinition{
            /* E -> E - T */
            ltoken: token::Token::new((
                token::TokenType::TkExp,
                String::from("EXP"),
                token::TokenVal::InVal
            )),
            pattern: vec![
                token::Token::new((
                    token::TokenType::TkExp,
                    String::from("EXP"),
                    token::TokenVal::InVal
                )),
                token::Token::new((
                    token::TokenType::TkMinus,
                    String::from("-"),
                    token::TokenVal::InVal
                )),
                token::Token::new((
                    token::TokenType::TkTerm,
                    String::new(),
                    token::TokenVal::InVal
                )),
            ],

        },
        SyntaxDefinition{
            /* E -> T */
            ltoken: token::Token::new((
                token::TokenType::TkExp,
                String::from("EXP"),
                token::TokenVal::InVal
            )),
            pattern: vec![
                token::Token::new((
                token::TokenType::TkTerm,
                    String::new(),
                    token::TokenVal::InVal
                )),
            ],
        },
        SyntaxDefinition{
            /* T -> A */
            ltoken: token::Token::new((
                token::TokenType::TkTerm,
                String::from("TERM"),
                token::TokenVal::InVal
            )),
            pattern: vec![
                token::Token::new((
                token::TokenType::TkAtom,
                    String::new(),
                    token::TokenVal::InVal,
                )),
            ],
        },
        SyntaxDefinition{
            /* T -> T * Num */
            ltoken: token::Token::new((
                token::TokenType::TkTerm,
                String::from("TERM"),
                token::TokenVal::InVal
            )),
            pattern: vec![
             token::Token::new((
                token::TokenType::TkTerm,
                String::from("TERM"),
                token::TokenVal::InVal
            )),
            token::Token::new((token::TokenType::TkStar,String::from("*"),token::TokenVal::InVal)),
                token::Token::new((
                token::TokenType::TkAtom,
                    String::new(),
                    token::TokenVal::InVal,
                )),
            ],
        },
        SyntaxDefinition{
            /* T -> T / Num */
            ltoken: token::Token::new((
                token::TokenType::TkTerm,
                String::from("TERM"),
                token::TokenVal::InVal
            )),
            pattern: vec![
             token::Token::new((
                token::TokenType::TkTerm,
                String::from("TERM"),
                token::TokenVal::InVal
            )),
            token::Token::new((token::TokenType::TkSlash,String::from("/"),token::TokenVal::InVal)),
                token::Token::new((
                token::TokenType::TkAtom,
                    String::new(),
                    token::TokenVal::InVal,
                )),
            ],
        },
        SyntaxDefinition{
            /* A -> Num */
            ltoken: token::Token::new((
                token::TokenType::TkAtom,
                String::from("ATOM"),
                token::TokenVal::InVal
            )),
            pattern: vec![
                token::Token::new((
                token::TokenType::TkIntlit,
                    String::new(),
                    token::TokenVal::IntVal(0),
                )),
            ],
        },
        SyntaxDefinition{
            /* A -> ( E )  */
            ltoken: token::Token::new((
                token::TokenType::TkAtom,
                String::from("ATOM"),
                token::TokenVal::InVal
            )),
            pattern: vec![
                token::Token::new((token::TokenType::TkLparen,String::from("("),token::TokenVal::InVal)),
                token::Token::new((
                token::TokenType::TkExp,
                    String::new(),
                    token::TokenVal::InVal,
                )),
                token::Token::new((token::TokenType::TkRparen,String::from(")"),token::TokenVal::InVal)),
            ],
        },
        ];
    }
}
*/
