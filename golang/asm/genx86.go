package asm

import (
	"github.com/urfave/cli"
)

func GenObject(sbins [][]byte, c *cli.Context) *ELF64 {
	elf := &ELF64{}
	elf.Ehdr = genEhdr(3, 2)
	for _, b := range sbins {
		elf.Sections = append(elf.Sections, NewSection(b))
	}
	return elf
}

func genEhdr(shnum int, shstrndx int) *Elf64_Ehdr {
	//Prototype
	return &Elf64_Ehdr{
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
		Shnum:               uint16(shnum),
		Shstr:               uint16(shstrndx),
	}
}
