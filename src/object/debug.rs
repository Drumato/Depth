extern crate cli_table;
use super::super::compile::frontend::parse::node::Func;
use super::super::compile::frontend::sema::semantics::Type;
use super::elf::elf64::ELF;
use cli_table::{Cell, Row};

pub fn build_debug_information(elf_file: &ELF, functions: Vec<Func>) -> Vec<u8> {
    let mut d_arraysize: u8 = 0;
    let mut debugs: Vec<u8> = Vec::new();

    for f in functions {
        let argument_number = f.args.len() as u8;
        let d_name = find_debug_name(elf_file, f.name);

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
            d_argtype: 0,
        };
        debugs.append(&mut debug_symbol.to_vec());
    }
    debugs
}

fn find_debug_name(elf_file: &ELF, func_name: String) -> u8 {
    if let Some(idx) = elf_file.names.get(&func_name) {
        *idx as u8
    } else {
        0
    }
}

fn count_dup_pointer(t: &Type) -> u8 {
    match t {
        Type::INTEGER => 0,
        Type::POINTER(ptr_to) => count_dup_pointer(ptr_to) + 1,
        Type::ALIAS(alias) => count_dup_pointer(alias),
        _ => 0,
    }
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
        let mut type_string = "i64".to_string();
        for _ in 0..(self.d_type & 0x7f) {
            type_string = format!("Pointer<{}>", type_string);
        }
        if self.d_type & DBG_ARRAY != 0 {
            type_string = format!("Array<{}, {}>", type_string, self.d_arraysize);
        }
        type_string
    }
    fn get_name(&self, elf_file: &ELF) -> String {
        let strtab = elf_file.sections[elf_file.ehdr.e_shstrndx as usize].clone();
        let mut debug_name = ELF::collect_name(strtab[self.d_name as usize..].to_vec());
        let length = debug_name.len();
        if length == 0 {
            debug_name = "(NULL)".to_string();
        }
        debug_name
    }
    //fn get_arg_types(&self) -> String {}
    pub fn to_stdout(&self, elf_file: &ELF) -> Row {
        let mut cells: Vec<Cell> = Vec::new();
        ELF::add_cell(&mut cells, &self.get_type());
        ELF::add_cell(&mut cells, &format!("0x{:x}", self.d_argnumber));
        ELF::add_cell(&mut cells, &self.get_name(elf_file));
        ELF::add_cell(&mut cells, &"TODO".to_string());
        //ELF::add_cell(&mut cells, &self.get_arg_types());
        Row::new(cells)
    }
}
