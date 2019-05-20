struct Elf64 {
    ehdr: Elf64Header,
    sections: Vec<Elf64Section>,
    segments: Vec<Elf64Segment>,
    phdrs: Vec<Elf64PHeader>,
    shdrs: Vec<Elf64SHeader>,
}

pub struct Elf64Header {
    pub magicnumber: u128,
    pub filetype: u16,
    pub march: u16,
    pub fileversion: u32,
    pub entrypoint: u64,
    pub phoff: u32,
    pub shoff: u32,
    pub flags: u32,
    pub size: u16,
    pub phsize: u16,
    pub phnum: u16,
    pub shsize: u16,
    pub shnum: u16,
    pub shstridx: u16,
}

struct Elf64PHeader {}
struct Elf64SHeader {}
struct Elf64Section {}
struct Elf64Segment {}

impl Elf64Header {
    pub fn new(
        param: (
            u128,
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
    ) -> Elf64Header {
        Elf64Header {
            magicnumber: param.0,
            filetype: param.1,
            march: param.2,
            fileversion: param.3,
            entrypoint: param.4,
            phoff: param.5,
            shoff: param.6,
            flags: param.7,
            size: param.8,
            phsize: param.9,
            phnum: param.10,
            shsize: param.11,
            shnum: param.12,
            shstridx: param.13,
        }
    }
    pub fn hex_dump(&self, ln: bool) -> String {
        if ln {
            return format!(
            "{:x}\n{:x}\n{:x}\n{:x}\n{:x}\n{:x}\n{:x}\n{:x}\n{:x}\n{:x}\n{:x}\n{:x}\n{:x}\n{:x}",
            self.magicnumber,
            self.filetype,
            self.march,
            self.fileversion,
            self.entrypoint,
            self.phoff,
            self.shoff,
            self.flags,
            self.size,
            self.phsize,
            self.phnum,
            self.shsize,
            self.shnum,
            self.shstridx
        );
        }
        format!(
            "{:x}{:x}{:x}{:x}{:x}{:x}{:x}{:x}{:x}{:x}{:x}{:x}{:x}{:x}",
            self.magicnumber,
            self.filetype,
            self.march,
            self.fileversion,
            self.entrypoint,
            self.phoff,
            self.shoff,
            self.flags,
            self.size,
            self.phsize,
            self.phnum,
            self.shsize,
            self.shnum,
            self.shstridx
        )
    }
    pub fn binary_dump(&self, ln: bool) -> String {
        if ln {
            return format!(
                "{:b}\n{:b}\n{:b}\n{:b}\n{:b}\n{:b}\n{:b}\n{:b}\n{:b}\n{:b}\n{:b}\n{:b}\n{:b}\n{:b}",
                self.magicnumber,
                self.filetype,
                self.march,
                self.fileversion,
                self.entrypoint,
                self.phoff,
                self.shoff,
                self.flags,
                self.size,
                self.phsize,
                self.phnum,
                self.shsize,
                self.shnum,
                self.shstridx
            );
        }
        format!(
            "{:<04b}{:b}{:b}{:b}{:b}{:b}{:b}{:b}{:b}{:b}{:b}{:b}{:b}{:b}",
            self.magicnumber,
            self.filetype,
            self.march,
            self.fileversion,
            self.entrypoint,
            self.phoff,
            self.shoff,
            self.flags,
            self.size,
            self.phsize,
            self.phnum,
            self.shsize,
            self.shnum,
            self.shstridx
        )
    }
}
