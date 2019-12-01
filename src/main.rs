extern crate yaml_rust;
#[macro_use]
extern crate clap;
use clap::App;

extern crate colored;
use colored::*;

mod compile;
use compile::backend as b;
use compile::frontend as f;
use compile::ir::llvm;
use compile::ir::tac::Tac;
use f::frontmanager::frontmanager as fm;
mod object;
use object::elf::elf64::ELF;
mod assemble;
use assemble as a;
mod ce;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::os::unix::fs::OpenOptionsExt;
mod link;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    /* compile phase */
    let file_name = matches.value_of("source").unwrap();
    let assembly: String = compile(file_name.to_string(), &matches)
        + "_start:\n  call main\n  mov rdi, rax\n  mov rax, 60\n  syscall\n";

    /* if stop-c given so output the assembly-code to file. */
    if matches.is_present("stop-c") {
        let output_path: String = file_name.split(".").collect::<Vec<&str>>()[0].to_string() + ".s";
        let mut file = File::create(output_path)?;
        file.write_all(assembly.as_bytes())?;
        std::process::exit(0);
    }

    /* assembly phase */
    let elf_binary: ELF = if !file_name.contains(".o") {
        assemble(assembly.to_string(), &matches)
    } else {
        // read the object file then construct ELF struct.
        ELF::read_elf(&file_name.to_string())
    };

    /* if stop-a given so output the object-file. */
    if matches.is_present("stop-a") {
        let output_path = file_name.split(".").collect::<Vec<&str>>()[0].to_string() + ".o";
        output_file_with_binary(output_path, elf_binary);
    } else {
        // link the object-file then we generate ET_EXEC from it.
        let exec_file: ELF = link::linker::Linker::linking(vec![elf_binary]);
        output_file_with_binary("a.out".to_string(), exec_file);
    }
    Ok(())
}
fn compile(file_name: String, matches: &clap::ArgMatches) -> String {
    if !file_name.contains(".dep") {
        return read_file(&file_name);
    }
    /* tokenize */
    let tokens: Vec<f::token::token::Token> = lex_phase(file_name.to_string(), &matches);

    /* parse */
    let funcs: Vec<f::parse::node::Func> = parse_phase(&matches, tokens);
    let mut front_manager: fm::FrontManager = fm::FrontManager::new(funcs);

    /* semantic-analyze */
    front_manager.semantics();

    if matches.is_present("dump-symbol") {
        front_manager.dump_symbol();
    }

    /* constant-fold with ast */
    front_manager.constant_folding();

    /* emit-llvm path */
    if matches.is_present("emit-llvm") {
        llvm::emit_llvm(file_name, front_manager);
        std::process::exit(0);
    }

    /* generate three-address-code from ast */
    front_manager.gen_tacs();
    let tacs: Vec<Tac> = front_manager.tacs;

    /* backend */
    let mut optimizer: b::Optimizer = b::Optimizer::new(tacs);

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
    b::codegen::genx64(optimizer.tacs)
}

fn assemble(assembler_code: String, matches: &clap::ArgMatches) -> ELF {
    use object::elf::elf64;

    /* tokenize */
    let tokens: Vec<a::lex::Token> = a::lex::lexing(assembler_code);

    /* parse */
    let (instructions, info_map, relas) = a::parse::parsing(tokens);
    if matches.is_present("dump-inst") {
        dump_inst(&instructions, &info_map);
    }

    let (code_map, mut relas) = a::gen::generate(instructions, info_map, relas);

    let shstrtab = build_shstrtab();

    /* build symbol-names from map */
    let symbol_names = code_map
        .iter()
        .map(|(name, _)| name.as_str())
        .collect::<Vec<&str>>();

    /* initialize with null symbol. */
    let mut symbols: Vec<elf64::Symbol> = vec![elf64::init_nullsym()];

    let mut total_len: u64 = 0;
    let mut total_code: Vec<u8> = Vec::with_capacity(2048);
    let mut name: u32 = 1;
    for (idx, (symbol_name, codes)) in code_map.iter().enumerate() {
        if codes.len() != 0 {
            symbols.push(elf64::init_sym(
                name,
                elf64::STB_GLOBAL,
                codes.len() as u64,
                total_len,
            ));
        } else {
            symbols.push(elf64::init_refsym(name, elf64::STB_GLOBAL));
        }
        name += symbol_name.len() as u32 + 1;
        total_len += codes.len() as u64;
        for b in codes.iter() {
            total_code.push(*b);
        }
        if let Some(rela) = relas.get_mut(symbol_name) {
            rela.r_info = (((idx + 1) << 32) + 1) as u64;
        }
    }
    let mut elf_file = ELF::init();
    let strtab: Vec<u8> = elf64::strtab(symbol_names);
    elf_file.add_section(vec![], elf64::init_nullhdr(), "null");
    elf_file.add_section(total_code, elf64::init_texthdr(total_len), ".text");
    let symbol_length = symbols.len();
    let symtab: Vec<u8> = elf64::symbols_to_vec(symbols);
    elf_file.add_section(
        symtab,
        elf64::init_symtabhdr(elf64::Symbol::size() as u64 * symbol_length as u64),
        ".symtab",
    );
    let strtab_length = strtab.len() as u64;
    elf_file.add_section(strtab, elf64::init_strtabhdr(strtab_length), ".strtab");
    let relas_length = relas.len() as u64;
    elf_file.add_section(
        elf64::relas_to_vec(relas.values().collect::<Vec<&elf64::Rela>>()),
        elf64::init_relahdr(elf64::Rela::size() as u64 * relas_length),
        ".rela.text",
    );
    let shstrtab_length = shstrtab.len() as u64;
    elf_file.add_section(
        shstrtab,
        elf64::init_strtabhdr(shstrtab_length),
        ".shstrtab",
    );
    elf_file.condition();
    elf_file
}

fn lex_phase(file_name: String, matches: &clap::ArgMatches) -> Vec<f::token::token::Token> {
    let filecontent: String = read_file(&file_name);
    let tokens: Vec<f::token::token::Token> = f::lex::lexing::lexing(filecontent);
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
    tokens: Vec<f::token::token::Token>,
) -> Vec<f::parse::node::Func> {
    let funcs: Vec<f::parse::node::Func> = f::parse::parser::parsing(tokens);
    if matches.is_present("dump-ast") {
        f::parse::node::dump_ast(&funcs);
    }
    funcs
}

fn read_file(s: &str) -> String {
    if s.to_string().contains(".o") {
        return s.to_string();
    }
    use std::fs;
    use std::path::Path;
    let filepath: &Path = Path::new(s);
    if filepath.exists() && filepath.is_file() {
        return fs::read_to_string(s).unwrap();
    }
    if filepath.is_dir() {
        eprintln!("{} is directory.", filepath.to_str().unwrap());
    } else {
        eprintln!("{} not found", filepath.to_str().unwrap());
        std::process::exit(1);
    }
    "".to_string()
}
fn dump_inst(
    instructions: &std::collections::BTreeMap<
        std::string::String,
        std::vec::Vec<assemble::parse::Inst>,
    >,
    info_map: &std::collections::BTreeMap<usize, assemble::parse::Info>,
) {
    for (symbol, v) in instructions.iter() {
        eprintln!("{}'s instructions", symbol.bold().green());
        for inst in v.iter() {
            let num: &usize = match inst {
                a::parse::Inst::BINARG(num)
                | a::parse::Inst::UNARG(num)
                | a::parse::Inst::NOARG(num) => num,
                a::parse::Inst::LABEL(num, _) => num,
            };
            if let Some(info) = info_map.get(num) {
                let lop_string: String = match &info.lop {
                    Some(l) => l.string(),
                    None => "".to_string(),
                };
                let rop_string: String = match &info.rop {
                    Some(r) => r.string(),
                    None => "".to_string(),
                };
                eprintln!(
                    "  instname->'{}' lop->'{}' rop->'{}' ",
                    info.inst_name.bold().blue(),
                    lop_string.green(),
                    rop_string.green(),
                );
            }
        }
    }
}

fn output_file_with_binary(output_path: String, binary: ELF) {
    let file = std::fs::OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .mode(0o755)
        .open(output_path)
        .unwrap();
    let mut writer = BufWriter::new(file);
    match writer.write_all(&binary.to_vec()) {
        Ok(_) => (),
        Err(e) => eprintln!("{}", e),
    }
    match writer.flush() {
        Ok(_) => (),
        Err(e) => eprintln!("{}", e),
    }
}

fn build_shstrtab() -> Vec<u8> {
    use object::elf::elf64;
    elf64::strtab(vec![
        ".text",
        ".symtab",
        ".strtab",
        ".rela.text",
        ".shstrtab",
    ])
}
