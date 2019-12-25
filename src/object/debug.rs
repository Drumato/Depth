extern crate cli_table;
use cli_table::{Cell, Row};

use crate::compile::frontend;
use crate::object::elf::elf64::ELF;
use frontend::parse::node::Func;
use frontend::sema::semantics::Type;

pub const LIMIT_DOCUMENTS: usize = 320;

pub fn build_debug_information(elf_file: &ELF, functions: Vec<Func>) -> Vec<u8> {
    let mut d_arraysize: u8 = 0;
    let mut debugs: Vec<u8> = Vec::new();

    for f in functions {
        let argument_number = f.args.len() as u8;
        let d_name = find_debug_name(elf_file, f.name.clone());

        let mut count_pointer = count_dup_pointer(&f.return_type);
        if let Type::ARRAY(_, n) = f.return_type {
            d_arraysize = n as u8;
            count_pointer = count_pointer | DBG_ARRAY;
        }
        let debug_symbol = DebugSymbol {
            d_type: count_pointer,
            d_argnumber: argument_number,
            d_name: d_name,
            d_arraysize: d_arraysize,
            d_argtype: build_arg_types(f),
        };
        debugs.append(&mut debug_symbol.to_vec());
    }
    debugs
}

pub fn build_documents(functions: Vec<Func>) -> Vec<u8> {
    let mut binary: Vec<u8> = Vec::new();
    for f in functions {
        let mut documents: Vec<u8>;
        if let Some(contents) = f.document {
            documents = contents.as_bytes().to_vec();
            if LIMIT_DOCUMENTS <= contents.len() {
                eprintln!(
                    "documentation's length stripped because its more than limits(320 bytes)."
                );
                documents = "no documents".to_string().as_bytes().to_vec();
            } else {
            }
        } else {
            documents = "no documents".to_string().as_bytes().to_vec();
        }
        // padding
        for _ in 0..(LIMIT_DOCUMENTS - documents.len()) {
            documents.push(0x00);
        }
        binary.append(&mut documents);
    }
    binary
}

fn find_debug_name(elf_file: &ELF, func_name: String) -> u8 {
    let symtab_number = elf_file.get_section_number(".symtab");
    let string_table_number = elf_file.shdrs[symtab_number].sh_link as usize;
    let strtab = elf_file.sections[string_table_number].clone();
    let name_count = strtab
        .iter()
        .filter(|num| *num == &0x00)
        .map(|b| *b)
        .collect::<Vec<u8>>()
        .len()
        - 1;
    let mut d_name = 1;
    for (idx, bb) in strtab
        .to_vec()
        .splitn(name_count, |num| *num == 0x00)
        .enumerate()
    {
        if idx == 0 {
            continue;
        }
        let b: Vec<u8> = bb
            .iter()
            .take_while(|num| *num != &0x00)
            .map(|b| *b)
            .collect::<Vec<u8>>();
        if func_name == String::from_utf8(b.to_vec()).unwrap() {
            return d_name;
        }
        d_name += b.len() as u8 + 1;
    }
    0
}

fn count_dup_pointer(t: &Type) -> u8 {
    match t {
        Type::INTEGER => 0,
        Type::POINTER(ptr_to) => count_dup_pointer(ptr_to) + 1,
        Type::ALIAS(alias) => count_dup_pointer(alias),
        _ => 0,
    }
}

fn build_arg_types(func: Func) -> u32 {
    let mut arg_types: u32 = 0;
    for (i, arg) in func.args.iter().enumerate() {
        let argument_name = arg.name().unwrap();
        if let Some(arg_symbol) = func.env.sym_table.get(&argument_name) {
            if let Ok(arg_type) = &arg_symbol.ty {
                arg_types |= (count_dup_pointer(&arg_type) as u32) << (i * 8);
            }
        }
    }
    arg_types
}

const DBG_ARRAY: u8 = 0b10000000;
#[repr(C)]
pub struct DebugSymbol {
    pub d_type: u8,
    pub d_argnumber: u8,
    pub d_name: u8,
    pub d_arraysize: u8,
    pub d_argtype: u32,
}
impl DebugSymbol {
    pub fn new_unsafe(binary: Vec<u8>) -> DebugSymbol {
        unsafe { std::ptr::read(binary.as_ptr() as *const DebugSymbol) }
    }
    pub fn size() -> usize {
        8
    }
    pub fn to_vec(&self) -> Vec<u8> {
        let mut bb = vec![self.d_type, self.d_argnumber, self.d_name, self.d_arraysize];
        for b in self.d_argtype.to_le_bytes().to_vec() {
            bb.push(b);
        }
        bb
    }
    fn get_type(&self) -> String {
        let mut type_string = Self::type_to_string(self.d_type);
        if self.d_type & DBG_ARRAY != 0 {
            type_string = format!("Array<{}, {}>", type_string, self.d_arraysize);
        }
        type_string
    }
    fn type_to_string(b: u8) -> String {
        let mut type_string = "i64".to_string();
        for _ in 0..(b & 0x7f) {
            type_string = format!("Pointer<{}>", type_string);
        }
        type_string
    }
    pub fn get_name(&self, elf_file: &ELF) -> String {
        let symtab_number = elf_file.get_section_number(".symtab");
        let string_table_number = elf_file.shdrs[symtab_number].sh_link as usize;
        let strtab = elf_file.sections[string_table_number].clone();
        let mut debug_name = ELF::collect_name(strtab[self.d_name as usize..].to_vec());
        let length = debug_name.len();
        if length == 0 {
            debug_name = "(NULL)".to_string();
        }
        debug_name
    }
    fn get_arg_types(&self) -> String {
        if self.d_argnumber == 0 {
            return "void".to_string();
        }
        let mut type_string = String::from("(");
        for (i, b) in self.d_argtype.to_le_bytes().to_vec().iter().enumerate() {
            if i as u8 == self.d_argnumber {
                break;
            }

            // last argument
            if i as u8 == self.d_argnumber - 1 {
                type_string += &(format!("{})", Self::type_to_string(*b)).as_str());
            } else {
                type_string += &(format!("{}, ", Self::type_to_string(*b)).as_str());
            }
        }
        type_string
    }
    pub fn to_stdout(&self, elf_file: &ELF) -> Row {
        let mut cells: Vec<Cell> = Vec::new();
        ELF::add_cell(&mut cells, &self.get_type());
        ELF::add_cell(&mut cells, &format!("0x{:x}", self.d_argnumber));
        ELF::add_cell(&mut cells, &self.get_name(elf_file));
        ELF::add_cell(&mut cells, &self.get_arg_types());
        Row::new(cells)
    }
}
