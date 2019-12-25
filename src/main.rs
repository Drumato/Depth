extern crate yaml_rust;
#[macro_use]
extern crate clap;
use clap::App;

use std::fs::File;
use std::io::Write;

mod compile;
mod object;
use object::elf::analyze;
use object::elf::elf64::ELF;
use object::elf::security;
mod assemble;
mod ce;
mod link;
mod load;
use load::elf::ELFLoader;
mod util;

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
        analyze::analyze_elf(matches)
    } else if matches.is_present("checksec") {
        security::check_security(matches)
    } else if matches.is_present("dump-symbol") {
        panic!("not implemented --dump-symbol option");
    } else {
        linux_generate_binary_main(matches)
    };
}

fn linux_generate_binary_main(
    matches: &clap::ArgMatches,
) -> Result<(), Box<dyn std::error::Error>> {
    /* compile phase */
    let startup_routine =
        util::read_file(&(std::env::var("DEPTH_ROOT").unwrap() + "/lib/start_up_linux.s").as_str());
    let file_name = matches.value_of("source").unwrap();
    let (mut assembly, debug_funcs) = compile::compile(file_name.to_string(), &matches);
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
        assemble::assemble(assembly.to_string(), &matches, debug_funcs)
    } else {
        // read the object file then construct ELF struct.
        ELF::read_elf(&file_name.to_string())
    };

    /* if 'stop-a' given so output the object-file. */
    if matches.is_present("stop-a") {
        let output_path = file_name.split(".").collect::<Vec<&str>>()[0].to_string() + ".o";
        util::output_file_with_binary(output_path, elf_binary);
    } else {
        /* link the object-file  */
        let exec_file: ELF = link::linker::Linker::linking(elf_binary);

        /* if 'run' given then the loader load the binary and execute machine code. */
        if matches.is_present("run") {
            let return_value = ELFLoader::load(exec_file);
            std::process::exit(return_value);
        } else {
            /* then we generate a.out from ET_EXEC. */
            util::output_file_with_binary("a.out".to_string(), exec_file);
        }
    }

    Ok(())
}
