extern crate libc;
use libc::c_void;

use super::super::object::elf::elf64::ELF;
pub struct ELFLoader {}

const PAGE_SIZE: libc::size_t = 4096;

impl ELFLoader {
    pub fn load(elf_file: ELF) -> i32 {
        let (program, page) = Self::setup_page_with_using_mmap();
        let binary = elf_file.to_vec();
        if let Some(unwrapped_phdrs) = elf_file.phdrs {
            let offset = unwrapped_phdrs[0].p_offset as usize;
            let segment_size = unwrapped_phdrs[0].p_filesz as usize;
            let load_segment = binary[offset..offset + segment_size].to_vec();
            let pointer_to_segment = load_segment.as_ptr();
            unsafe {
                program.copy_from_nonoverlapping(pointer_to_segment, segment_size as usize);
                let f: fn() -> i32 = ::std::mem::transmute(page);
                f()
            }
        } else {
            0
        }
    }

    fn setup_page_with_using_mmap() -> (*mut u8, *mut c_void) {
        unsafe {
            let program: *mut u8;
            let start: *mut c_void = 0x400000 as *mut c_void;
            let page: *mut c_void = libc::mmap(
                start,
                PAGE_SIZE,
                libc::PROT_EXEC | libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_ANONYMOUS | libc::MAP_PRIVATE,
                0,
                0,
            );
            program = ::std::mem::transmute(page);
            return (program, page);
        }
    }
}
