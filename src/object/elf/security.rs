extern crate clap;
extern crate colored;
use colored::*;

use super::elf64;
use elf64::{Symbol, ELF, PIE, RELRO};

pub fn check_security(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let file_name = matches.value_of("source").unwrap().to_string();
    if file_name.contains(".dep") {
        return Ok(());
    }
    let elf_file = ELF::read_elf(&file_name);
    println!("\n\n++++++++++++++++ checksec +++++++++++++++");

    // check PF_X in GNU_STACK
    println!("{}", "Execstack:".bold().blue());
    if elf_file.check_execstack() {
        println!(
            "\t{}",
            "This binary may be able to exploit by execution code on stack"
                .bold()
                .red()
        );
    } else {
        println!(
            "\t{}",
            "This binary's NX-bit is on so we prevent execution code on stack."
                .bold()
                .green()
        );
    }

    // check stack__chk in symbol table.
    println!("{}", "Canary:".bold().blue());
    let symbol_size = Symbol::size() as usize;

    let symtab_number = elf_file.get_section_number(".symtab");
    let symtab = elf_file.sections[symtab_number].clone();
    let symtab_shdr = elf_file.shdrs[symtab_number].clone();
    let symbol_number = symtab_shdr.sh_size as usize / symbol_size;

    let mut canary_flag = false;
    for i in 0..symbol_number as usize {
        let symbol_binary = symtab[i * symbol_size..(i + 1) * symbol_size].to_vec();
        let symbol = Symbol::new_unsafe(symbol_binary);
        let symbol_name = symbol.get_name(&elf_file, symtab_shdr.sh_link as u32);
        for chk in vec![
            "__stack_chk_fail".to_string(),
            "__stack_smach_handler".to_string(),
        ]
        .iter()
        {
            if symbol_name.contains(chk) {
                canary_flag = true;
            }
        }
    }
    if canary_flag {
        println!("\t{}", "Found".bold().green());
    } else {
        println!("\t{}", "Not Found".bold().red());
    }

    // Relocation Read-Only
    println!("{}", "RELRO:".bold().blue());
    match elf_file.check_relro() {
        RELRO::ENABLE => println!("\t{}", "Full RELRO".bold().green()),
        RELRO::PARTIAL => println!("\t{}", "Partial RELRO".bold().red()),
        RELRO::DISABLE => println!("\t{}", "No RELRO".bold().red()),
    }

    // PIE
    println!("{}", "PIE:".bold().blue());
    match elf_file.check_pie() {
        PIE::ENABLE => println!("\t{}", "Enabled".bold().green()),
        PIE::DSO => println!("\t{}", "DSO".bold().green()),
        PIE::DISABLE => println!("\t{}", "Disabled".bold().red()),
    }

    // DT_RPATH
    println!("{}", "DT_RPATH:".bold().blue());
    if elf_file.check_rpath() {
        println!("\t{}", "Enabled".bold().red());
    } else {
        println!("\t{}", "Disabled".bold().green());
    }

    // DT_RUNPATH
    println!("{}", "DT_RUNPATH:".bold().blue());
    if elf_file.check_runpath() {
        println!("\t{}", "Enabled".bold().red());
    } else {
        println!("\t{}", "Disabled".bold().green());
    }
    Ok(())
}
