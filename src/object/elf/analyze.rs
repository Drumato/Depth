extern crate colored;
use colored::*;
extern crate clap;
extern crate cli_table;
use cli_table::{Cell, Row, Table};

use super::super::debug;
use super::elf64;
use debug::DebugSymbol;
use elf64::{Dyn, Rela, Symbol, ELF};

pub fn analyze_elf(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
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
            "\n\nThere are {} section headers, starting at 0x{:x}",
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
        let debug_names: Vec<String> = print_debugs(&elf_file)?;
        print_documents(&elf_file, debug_names)?;
    }
    Ok(())
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

        if shdr.sh_type as u64 == elf64::SHT_RELA {
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

        if shdr.sh_type as u64 == elf64::SHT_DYNAMIC {
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
fn print_debugs(elf_file: &ELF) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let debug_number = elf_file.get_section_number(".dbg.depth");
    let debug_table = elf_file.sections[debug_number].clone();
    let debug_shdr = elf_file.shdrs[debug_number].clone();
    let debug_count = debug_shdr.sh_size as usize / DebugSymbol::size();

    println!(
        "\n\nDebug table '.dbg.depth' contains {} entries:",
        debug_count
    );

    let mut rows: Vec<Row> = Vec::new();
    rows.push(ELF::debug_table_columns());

    let mut debug_names: Vec<String> = Vec::new();
    for i in 0..debug_count as usize {
        let debug_binary =
            debug_table[i * DebugSymbol::size()..(i + 1) * DebugSymbol::size()].to_vec();
        let debug_symbol = DebugSymbol::new_unsafe(debug_binary);
        debug_names.push(debug_symbol.get_name(elf_file));
        rows.push(debug_symbol.to_stdout(elf_file));
    }

    let table = Table::new(rows, Default::default());
    table.print_stdout()?;

    Ok(debug_names)
}

fn print_documents(
    elf_file: &ELF,
    debug_names: Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let document_number = elf_file.get_section_number(".documents");
    let document_binary = elf_file.sections[document_number].clone();

    let mut rows: Vec<Row> = Vec::new();
    rows.push(ELF::documents_columns());
    for (i, name) in debug_names.iter().enumerate() {
        let mut cells: Vec<Cell> = Vec::new();
        let documents =
            document_binary[i * debug::LIMIT_DOCUMENTS..(i + 1) * debug::LIMIT_DOCUMENTS].to_vec();
        ELF::add_cell(&mut cells, name);
        ELF::add_cell(&mut cells, &String::from_utf8(documents).unwrap());
        rows.push(Row::new(cells));
    }

    let table = Table::new(rows, Default::default());
    table.print_stdout()?;
    Ok(())
}
