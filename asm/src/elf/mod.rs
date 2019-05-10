struct Elf64 {
    ehdr: Elf64_Header,
    phdrs: Vec<Elf64_PHeader>,
    shdrs: Vec<Elf64_SHeader>,
}

struct Elf64_Header {
    magicnumber: u32,
    class: u16,
    data: u16,
    version: u32,
    osabi: u16,
    abiversion: u16,
    //padding : u32,
    filetype: u16,
    march: u16,
    fileversion: u32,
    entrypoint: u64,
    phoff: u32,
    shoff: u32,
    flags: u32,
    size: u16,
    phsize: u16,
    phnum: u16,
    shsize: u16,
    shnum: u16,
    shstridx: u16,
}

struct Elf64_PHeader {}
struct Elf64_SHeader {}

impl Elf64_Header {
    pub fn new(
        param: (
            u32,
            u16,
            u16,
            u32,
            u16,
            u16,
            u16,
            u16,
            u32,
            u64,
            u32,
            u32,
            u32,
            u16,
            u16,
            u16,
            u16,
            u16,
            u16,
        ),
    ) -> Elf64_Header {
        Elf64_Header {
            magicnumber: param.0,
            class: param.1,
            data: param.2,
            version: param.3,
            osabi: param.4,
            abiversion: param.5,
            filetype: param.6,
            march: param.7,
            fileversion: param.8,
            entrypoint: param.9,
            phoff: param.10,
            shoff: param.11,
            flags: param.12,
            size: param.13,
            phsize: param.14,
            phnum: param.15,
            shsize: param.16,
            shnum: param.17,
            shstridx: param.18,
        }
    }
}
