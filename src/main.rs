extern crate yaml_rust;
#[macro_use]
extern crate clap;
use clap::App;

extern crate colored;
use colored::*;
extern crate cli_table;
use cli_table::{Row, Table};

mod compile;
use compile::backend as b;
use compile::frontend as f;
use compile::ir::llvm;
use compile::ir::tac::Tac;
use f::frontmanager::frontmanager as fm;
mod object;
use object::debug::DebugSymbol;
use object::elf::elf64::{Dyn, Rela, Symbol, ELF};
mod assemble;
use assemble as a;
mod ce;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::os::unix::fs::OpenOptionsExt;
mod link;
mod load;
use load::elf::ELFLoader;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    /* switch the process each OS */
    return if cfg!(target_os = "linux") {
        linux_main(&matches)
    } else if cfg!(target_os = "mac_os") {
        panic!("not implemented on MacOS");
    } else if cfg!(target_os = "windows") {
        panic!("not implemented on Windows");
    } else {
        Ok(())
    };
}

fn linux_main(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    if let Err(r) = std::env::var("DEPTH_ROOT") {
        panic!("{} -> DEPTH_ROOT", r);
    };
    return if matches.is_present("readelf") {
        analyze_elf(matches)
    } else if matches.is_present("dump-symbol") {
        panic!("not implemented --dump-symbol option");
    } else {
        linux_generate_binary_main(matches)
    };
}

fn analyze_elf(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let file_name = matches.value_of("source").unwrap().to_string();
    if file_name.contains(".dep") {
        return Ok(());
    }
    let elf_file = ELF::read_elf(&file_name);
    // -h option
    if matches.is_present("all") || matches.is_present("header") {
        elf_file.ehdr.print_to_stdout();
    }

    // -S option
    if matches.is_present("all") || matches.is_present("section") {
        let shnum = elf_file.ehdr.e_shnum;
        let shoff = elf_file.ehdr.e_shoff;
        println!(
            "There are {} section headers, starting at 0x{:x}",
            shnum, shoff
        );
        print_shdrs_stdout(&elf_file)?;
        println!("Key to Flags:");
        println!("  W (write), A (alloc), X (execute), M (merge), S (strings), I (info),");
        println!("  L (link order), O (extra OS processing required), G (group), T (TLS),");
        println!("  C (compressed), x (unknown), o (OS specific), E (exclude),");
        println!("  l (large), p (processor specific)");
    }

    // -l option
    if matches.is_present("all") || matches.is_present("segment") {
        let phnum = elf_file.ehdr.e_phnum;
        let phoff = elf_file.ehdr.e_phoff;
        if phnum != 0 {
            println!("\n\n{}", "Program Headers:".bold().green());
            println!(
                "There are {} program headers, starting at 0x{:x}",
                phnum, phoff
            );
            print_phdrs_stdout(&elf_file)?;
        } else {
            println!("There are no program header table");
        }
    }

    // -s option
    if matches.is_present("all") || matches.is_present("symbol") {
        if elf_file.check_whether_given_section_is_exist(".symtab") {
            print_symbols(&elf_file, ".symtab")?;
        } else {
            println!("There are no .symtab section");
        }
        if elf_file.check_whether_given_section_is_exist(".dynsym") {
            print_symbols(&elf_file, ".dynsym")?;
        } else {
            println!("There are no .dynsym section");
        }
    }

    // -r option
    if matches.is_present("all") || matches.is_present("relocation") {
        print_relocations(&elf_file)?;
    }

    // -d option
    if matches.is_present("all") || matches.is_present("dynamic") {
        print_dynamics(&elf_file)?;
    }

    // --debug option
    if matches.is_present("all") || matches.is_present("debug") {
        print_debugs(&elf_file)?;
    }
    Ok(())
}

fn linux_generate_binary_main(
    matches: &clap::ArgMatches,
) -> Result<(), Box<dyn std::error::Error>> {
    /* compile phase */
    let startup_routine =
        read_file(&(std::env::var("DEPTH_ROOT").unwrap() + "/lib/start_up_linux.s").as_str());
    let file_name = matches.value_of("source").unwrap();
    let (mut assembly, debug_funcs) = compile(file_name.to_string(), &matches);
    assembly += &startup_routine;

    /* if 'stop-c' given so output the assembly-code to file. */
    if matches.is_present("stop-c") {
        let output_path: String = file_name.split(".").collect::<Vec<&str>>()[0].to_string() + ".s";
        let mut file = File::create(output_path)?;
        file.write_all(assembly.as_bytes())?;
        std::process::exit(0);
    }

    /* assembly phase */
    let elf_binary: ELF = if !file_name.contains(".o") {
        assemble(assembly.to_string(), &matches, debug_funcs)
    } else {
        // read the object file then construct ELF struct.
        ELF::read_elf(&file_name.to_string())
    };

    /* if 'stop-a' given so output the object-file. */
    if matches.is_present("stop-a") {
        let output_path = file_name.split(".").collect::<Vec<&str>>()[0].to_string() + ".o";
        output_file_with_binary(output_path, elf_binary);
    } else {
        /* link the object-file  */
        let exec_file: ELF = link::linker::Linker::linking(elf_binary);

        /* if 'run' given then the loader load the binary and execute machine code. */
        if matches.is_present("run") {
            let return_value = ELFLoader::load(exec_file);
            std::process::exit(return_value);
        } else {
            /* then we generate a.out from ET_EXEC. */
            output_file_with_binary("a.out".to_string(), exec_file);
        }
    }

    Ok(())
}
fn compile(file_name: String, matches: &clap::ArgMatches) -> (String, Vec<f::parse::node::Func>) {
    if !file_name.contains(".dep") {
        return (read_file(&file_name), vec![]);
    }

    /* tokenize */
    let tokens: Vec<f::token::token::Token> = lex_phase(file_name.to_string(), &matches);

    /* parse */
    let funcs: Vec<f::parse::node::Func> = parse_phase(&matches, tokens);
    let mut front_manager: fm::FrontManager = fm::FrontManager::new(funcs);

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
    (b::codegen::genx64(optimizer.tacs), functions)
}

fn assemble(
    assembler_code: String,
    matches: &clap::ArgMatches,
    debug_funcs: Vec<f::parse::node::Func>,
) -> ELF {
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

    let total_len: u64 = sum_all_code_length(&code_map);
    let mut total_code: Vec<u8> = Vec::with_capacity(2048);

    /* initialize string-index with null byte. */
    let mut name: u32 = 1;
    for (idx, (symbol_name, codes)) in code_map.iter().enumerate() {
        if codes.len() != 0 {
            symbols.push(elf64::init_sym(
                name,
                elf64::STB_GLOBAL,
                codes.len() as u64,
                total_code.len() as u64,
            ));
        } else {
            symbols.push(elf64::init_refsym(name, elf64::STB_GLOBAL));
        }

        /* progress the index with null byte. */
        name += symbol_name.len() as u32 + 1;

        /* merge binaries into one vector. */
        for b in codes.iter() {
            total_code.push(*b);
        }
        if let Some(rela) = relas.get_mut(symbol_name) {
            rela.r_info = (((idx + 1) << 32) + 1) as u64;
        }
    }

    let mut elf_file = ELF::init();

    /* add all-sections. */
    let strtab: Vec<u8> = elf64::strtab(symbol_names);
    elf_file.add_section(vec![], elf64::init_nullhdr(), "null");

    /* .text */
    elf_file.add_section(total_code, elf64::init_texthdr(total_len), ".text");

    /* .symtab */
    let symbol_length = symbols.len();
    let symtab: Vec<u8> = elf64::symbols_to_vec(symbols);
    let symtab_size = elf64::Symbol::size() as u64 * symbol_length as u64;
    elf_file.add_section(symtab, elf64::init_symtabhdr(symtab_size), ".symtab");

    /* .strtab */
    let strtab_length = strtab.len() as u64;
    elf_file.add_section(strtab, elf64::init_strtabhdr(strtab_length), ".strtab");

    /* .rela.text */
    let relas_length = relas.len() as u64;
    let relas_tab = elf64::relas_to_vec(relas.values().collect::<Vec<&elf64::Rela>>());
    let relas_size = elf64::Rela::size() as u64 * relas_length;
    elf_file.add_section(relas_tab, elf64::init_relahdr(relas_size), ".rela.text");

    /* .dbg.depth */
    let debug_section_binary = object::debug::build_debug_information(&elf_file, debug_funcs);
    let debug_length = debug_section_binary.len();
    elf_file.add_section(
        debug_section_binary,
        elf64::init_debughdr(debug_length as u64),
        ".dbg.depth",
    );

    /* .shstrtab */
    let shstrtab_length = shstrtab.len() as u64;
    let shstrtab_hdr = elf64::init_strtabhdr(shstrtab_length);
    elf_file.add_section(shstrtab, shstrtab_hdr, ".shstrtab");

    elf_file.condition();
    elf_file
}

fn lex_phase(file_name: String, matches: &clap::ArgMatches) -> Vec<f::token::token::Token> {
    let filecontent: String = read_file(&file_name);

    /* lex */
    let tokens: Vec<f::token::token::Token> = f::lex::lexing::lexing(filecontent);

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
    tokens: Vec<f::token::token::Token>,
) -> Vec<f::parse::node::Func> {
    /* parse */
    let funcs: Vec<f::parse::node::Func> = f::parse::parser::parsing(tokens);

    /* render ast by string to stderr */
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

fn sum_all_code_length(
    code_map: &std::collections::BTreeMap<std::string::String, std::vec::Vec<u8>>,
) -> u64 {
    code_map
        .iter()
        .map(|(_, codes)| codes.len() as u64)
        .sum::<u64>()
}
fn print_shdrs_stdout(elf_file: &ELF) -> Result<(), Box<dyn std::error::Error>> {
    let mut rows: Vec<Row> = elf_file
        .shdrs
        .iter()
        .map(|shdr| shdr.to_stdout(elf_file))
        .collect::<Vec<Row>>();
    rows.insert(0, ELF::section_header_columns());

    let table = Table::new(rows, Default::default());
    table.print_stdout()?;
    Ok(())
}

fn print_phdrs_stdout(elf_file: &ELF) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(derefered_phdrs) = &elf_file.phdrs {
        let mut rows: Vec<Row> = derefered_phdrs
            .iter()
            .map(|phdr| phdr.to_stdout())
            .collect::<Vec<Row>>();
        rows.insert(0, ELF::program_header_columns());

        let table = Table::new(rows, Default::default());
        table.print_stdout()?;
    }
    Ok(())
}
fn print_symbols(elf_file: &ELF, section_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let symbol_size = Symbol::size() as usize;
    let mut rows: Vec<Row> = Vec::new();
    rows.push(ELF::symbol_table_columns());

    let symtab_number = elf_file.get_section_number(section_name);
    let symtab = elf_file.sections[symtab_number].clone();
    let symtab_shdr = elf_file.shdrs[symtab_number].clone();
    let symbol_number = symtab_shdr.sh_size as usize / symbol_size;
    println!(
        "\n\nSymbol table '{}' contains {} entries:",
        section_name, symbol_number
    );

    for i in 0..symbol_number as usize {
        let symbol_binary = symtab[i * symbol_size..(i + 1) * symbol_size].to_vec();
        let symbol = Symbol::new_unsafe(symbol_binary);
        rows.push(symbol.to_stdout(elf_file, symtab_shdr.sh_link));
    }

    let table = Table::new(rows, Default::default());
    table.print_stdout()?;

    Ok(())
}

fn print_relocations(elf_file: &ELF) -> Result<(), Box<dyn std::error::Error>> {
    let shstrtab = elf_file.sections[elf_file.ehdr.e_shstrndx as usize].clone();

    for (i, shdr) in elf_file.shdrs.iter().enumerate() {
        let mut rows: Vec<Row> = Vec::new();
        rows.push(ELF::relocation_table_columns());

        if shdr.sh_type as u64 == object::elf::elf64::SHT_RELA {
            let relocation_number = shdr.sh_size as usize / Rela::size();
            let section_name = ELF::collect_name(shstrtab[shdr.sh_name as usize..].to_vec());

            println!(
                "\n\nRelocation section '{}' at offset 0x{:x} contains {} entry:",
                section_name, shdr.sh_offset, relocation_number,
            );

            let relatab = elf_file.sections[i].clone();
            for i in 0..relocation_number as usize {
                let rela_binary = relatab[i * Rela::size()..(i + 1) * Rela::size()].to_vec();
                let rela = Rela::new_unsafe(rela_binary);

                // for print symbol name
                let symtab_header_link = shdr.sh_link as usize;
                rows.push(rela.to_stdout(elf_file, symtab_header_link));
            }
            let table = Table::new(rows, Default::default());
            table.print_stdout()?;
        }
    }

    Ok(())
}
fn print_dynamics(elf_file: &ELF) -> Result<(), Box<dyn std::error::Error>> {
    for (i, shdr) in elf_file.shdrs.iter().enumerate() {
        let mut rows: Vec<Row> = Vec::new();
        rows.push(ELF::dynamic_table_columns());

        if shdr.sh_type as u64 == object::elf::elf64::SHT_DYNAMIC {
            let dynamics_number = shdr.sh_size as usize / Dyn::size();
            println!(
                "\n\nDynamic section at offset 0x{:x} contains {} entry:",
                shdr.sh_offset, dynamics_number,
            );

            let dyntab = elf_file.sections[i].clone();
            for i in 0..dynamics_number as usize {
                let dyn_binary = dyntab[i * Dyn::size()..(i + 1) * Dyn::size()].to_vec();
                let dyn_sym = Dyn::new_unsafe(dyn_binary);

                // for print symbol name
                let symtab_header_link = shdr.sh_link as usize;
                rows.push(dyn_sym.to_stdout(elf_file, symtab_header_link));
            }
            let table = Table::new(rows, Default::default());
            table.print_stdout()?;
        }
    }
    Ok(())
}
fn print_debugs(elf_file: &ELF) -> Result<(), Box<dyn std::error::Error>> {
    let debug_number = 5;
    let debug_table = elf_file.sections[debug_number].clone();
    let debug_shdr = elf_file.shdrs[debug_number].clone();
    let debug_count = debug_shdr.sh_size as usize / DebugSymbol::size();

    println!(
        "\n\nDebug table '.dbg.depth' contains {} entries:",
        debug_count
    );

    let mut rows: Vec<Row> = Vec::new();
    rows.push(ELF::debug_table_columns());

    for i in 0..debug_count as usize {
        let debug_binary =
            debug_table[i * DebugSymbol::size()..(i + 1) * DebugSymbol::size()].to_vec();
        let debug_symbol = DebugSymbol::new_unsafe(debug_binary);
        rows.push(debug_symbol.to_stdout(elf_file));
    }

    let table = Table::new(rows, Default::default());
    table.print_stdout()?;

    Ok(())
}
