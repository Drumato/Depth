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

fn main() -> Result<(), Box<std::error::Error>> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let _tokens: Vec<tok::Token> = lex_phase(&matches);
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
