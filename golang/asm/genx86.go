package asm

import (
	"github.com/urfave/cli"
)

func GenObject(sbins [][]byte, c *cli.Context) *ELF64 {
	elf := &ELF64{}
	elf.Ehdr = genEhdr(uint16(len(sbins)), uint16(len(sbins))-1)
	for _, b := range sbins {
		elf.Sections = append(elf.Sections, NewSection(b))
	}
	return elf
}

func genEhdr(shnum uint16, shstrndx uint16) *ELF64_Ehdr {
	//Prototype
	return &ELF64_Ehdr{
		MagicNumber:         0x7f454c46,
		Class:               0x2,
		Data:                0x1,
		Version:             0x1,
		OSABI:               0x0,
		ABIVersion:          0x0,
		FileType:            0x1,
		MachineArchitecture: 0x3e,
		FileVersion:         0x1,
		EntryPoint:          0x0,
		Phoff:               0x0,
		Shoff:               0x208,
		Flags:               0x0,
		Size:                0x40,
		Phsize:              0x0,
		Phnum:               0x0,
		Shsize:              0x40,
		Shnum:               shnum,
		Shstr:               shstrndx,
	}
}

func genShdr(ty uint32, flag uint64, name uint32, size uint64) *ELF64_Shdr {
	return &ELF64_Shdr{

		Name:  name,
		Type:  ty,
		Flags: flag,
		Addr:  0x0,
		//Offset:    uint64,
		Size: size,
		//Link:      uint32,
		//Info:      uint32,
		//Alignment: uint64,
		//EntrySize: uint64,
	}
}
