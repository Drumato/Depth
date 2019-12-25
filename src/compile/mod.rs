pub mod backend;
pub mod frontend;
pub mod ir;

extern crate clap;
extern crate colored;
use colored::*;

use crate::util;
use frontend::frontmanager::frontmanager::FrontManager;
use ir::llvm;
use ir::tac::Tac;

pub fn compile(
    file_name: String,
    matches: &clap::ArgMatches,
) -> (String, Vec<frontend::parse::node::Func>) {
    if !file_name.contains(".dep") {
        return (util::read_file(&file_name), vec![]);
    }

    /* tokenize */
    let tokens: Vec<frontend::token::token::Token> = lex_phase(file_name.to_string(), &matches);

    /* parse */
    let funcs: Vec<frontend::parse::node::Func> = parse_phase(&matches, tokens);
    let mut front_manager: FrontManager = FrontManager::new(funcs);

    /* semantic-analyze */
    front_manager.semantics();

    /* constant-fold with ast */
    front_manager.constant_folding();

    /* emit-llvm path */
    if matches.is_present("emit-llvm") {
        llvm::emit_llvm(file_name, front_manager);
        std::process::exit(0);
    }

    /* escape functions for debug section*/
    let functions = front_manager.functions.clone();

    /* generate three-address-code from ast */
    front_manager.gen_tacs();
    let tacs: Vec<Tac> = front_manager.tacs;

    /* backend */
    let mut optimizer: backend::Optimizer = backend::Optimizer::new(tacs);

    /* build the control-flow-graph */
    optimizer.build_cfg();
    if matches.is_present("dump-cfg") {
        optimizer.dump_cfg();
    }

    /* TODO: not implemented yet */
    if matches.is_present("Opt1") {
        optimizer.build_cfg_for_reaching();
        optimizer.reaching_definition();
        optimizer.available_expression();
    }

    /* append the information for liveness */
    optimizer.build_cfg_for_liveness();

    /* liveness-analysis */
    optimizer.liveness();
    if matches.is_present("dump-liveness") {
        optimizer.dump_liveness();
    }
    /* linear register allocation */
    optimizer.regalloc();
    if matches.is_present("dump-tac") {
        eprintln!("{}", "--------dump-tac--------".blue().bold());
        for (i, tac) in optimizer.tacs.iter().enumerate() {
            eprintln!("{}: {}", i, tac.string());
        }
    }

    /* codegen */
    (backend::codegen::genx64(optimizer.tacs), functions)
}

fn lex_phase(file_name: String, matches: &clap::ArgMatches) -> Vec<frontend::token::token::Token> {
    let filecontent: String = util::read_file(&file_name);

    /* lex */
    let tokens: Vec<frontend::token::token::Token> = frontend::lex::lexing::lexing(filecontent);

    /* render tokens to stderr */
    if matches.is_present("dump-token") {
        eprintln!("{}", "--------dumptoken--------".blue().bold());
        for t in tokens.iter() {
            eprintln!("{}", t.string().green().bold());
        }
    }

    tokens
}

fn parse_phase(
    matches: &clap::ArgMatches,
    tokens: Vec<frontend::token::token::Token>,
) -> Vec<frontend::parse::node::Func> {
    /* parse */
    let funcs: Vec<frontend::parse::node::Func> = frontend::parse::parser::parsing(tokens);

    /* render ast by string to stderr */
    if matches.is_present("dump-ast") {
        frontend::parse::node::dump_ast(&funcs);
    }
    funcs
}
