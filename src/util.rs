use crate::object::elf::elf64::ELF;
use std::io::{BufWriter, Write};
use std::os::unix::fs::OpenOptionsExt;

pub fn read_file(s: &str) -> String {
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

pub fn output_file_with_binary(output_path: String, binary: ELF) {
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
