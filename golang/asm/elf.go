package asm

import (
	"bytes"
	"encoding/binary"

	"github.com/sirupsen/logrus"
)

type ELF64 struct {
	Ehdr     *ELF64_Ehdr
	Sections []*Section
	//Segments []*Segment
	Phdrs []*ELF64_Phdr
	Shdrs []*ELF64_Shdr
}

func (e *ELF64) Dump() []byte {
	var buf bytes.Buffer
	if _, err := buf.Write(e.Ehdr.Dump()); err != nil {
		logrus.Errorf("Error found: %+v", err)
	}
	/*
		for _, phdr := range e.Phdrs {
			if _, err := buf.Write(phdr.Dump()); err != nil {
				logrus.Errorf("Error found: %+v", err)
			}
		}
	*/
	for _, section := range e.Sections {
		if _, err := buf.Write(section.Binary); err != nil {
			logrus.Errorf("Error found: %+v", err)
		}
	}
	/*
		for _, shdr := range e.Shdrs {
			if _, err := buf.Write(shdr.Dump()); err != nil {
				logrus.Errorf("Error found: %+v", err)
			}
		}
	*/
	return buf.Bytes()

}

/* 32-bit ELF base types. */
type ELF32_Addr uint32
type ELF32_Half uint16
type ELF32_Off uint32
type ELF32_Sword int32
type ELF32_Word uint32

/* 64-bit ELF base types. */
type ELF64_Addr uint64
type ELF64_Half uint16
type ELF64_SHalf int16
type ELF64_Off uint64
type ELF64_Sword int32
type ELF64_Word uint32
type ELF64_Xword uint64
type ELF64_Sxword int64
type ELF64_Section uint16

type ELF64_Ehdr struct {
	MagicNumber         uint32
	Class               uint16
	Data                uint16
	Version             uint32
	OSABI               uint16
	ABIVersion          uint16
	Padding             uint32
	FileType            uint16
	MachineArchitecture uint16
	FileVersion         uint32
	EntryPoint          uint64
	Phoff               uint64
	Shoff               uint64
	Flags               uint32
	Size                uint16
	Phsize              uint16
	Phnum               uint16
	Shsize              uint16
	Shnum               uint16
	Shstr               uint16
}

func (e *ELF64_Ehdr) Dump() []byte {
	if e.Check() {
		buf := make([]byte, 64)
		binary.BigEndian.PutUint32(buf[0:], e.MagicNumber)
		binary.LittleEndian.PutUint16(buf[4:], e.Class)
		binary.LittleEndian.PutUint16(buf[5:], e.Data)
		binary.LittleEndian.PutUint32(buf[6:], e.Version)
		binary.LittleEndian.PutUint16(buf[7:], e.OSABI)
		binary.LittleEndian.PutUint16(buf[8:], e.ABIVersion)
		binary.LittleEndian.PutUint32(buf[9:], e.Padding)
		binary.LittleEndian.PutUint16(buf[16:], e.FileType)
		binary.LittleEndian.PutUint16(buf[18:], e.MachineArchitecture)
		binary.LittleEndian.PutUint32(buf[20:], e.FileVersion)
		binary.LittleEndian.PutUint64(buf[24:], e.EntryPoint)
		binary.LittleEndian.PutUint64(buf[32:], e.Phoff)
		binary.LittleEndian.PutUint64(buf[40:], e.Shoff)
		binary.LittleEndian.PutUint32(buf[48:], e.Flags)
		binary.LittleEndian.PutUint16(buf[52:], e.Size)
		binary.LittleEndian.PutUint16(buf[54:], e.Phsize)
		binary.LittleEndian.PutUint16(buf[56:], e.Phnum)
		binary.LittleEndian.PutUint16(buf[58:], e.Shsize)
		binary.LittleEndian.PutUint16(buf[60:], e.Shnum)
		binary.LittleEndian.PutUint16(buf[62:], e.Shstr)
		return buf
	}
	logrus.Errorf("cannot dump object-file")
	return nil
}

func (e *ELF64_Ehdr) Check() bool {
	if e.MagicNumber != 0x7f454c46 {
		logrus.Errorf("Invalid MagicNumber: it's not an elf format file")
		return false
	}
	if e.Class != 0x2 {
		logrus.Errorf("Invalid Class: it's not 64-bit file")
		return false
	}
	if e.Data != 0x1 {
		logrus.Errorf("Invalid Endian: x86 architecture using only little-endian")
		return false
	}
	return true
}

/* Section */

type Section struct {
	Size   uint64
	Binary []byte
}

func NewSection(b []byte) *Section {
	return &Section{Binary: b, Size: uint64(len(b))}

}

type ELF64_Shdr struct {
	Name      uint32
	Type      uint32
	Flags     uint64
	Addr      uint64
	Offset    uint64
	Size      uint64
	Link      uint32
	Info      uint32
	Alignment uint64
	EntrySize uint64
}

func (s *ELF64_Shdr) Dump() []byte {
	buf := make([]byte, s.Size)
	binary.LittleEndian.PutUint32(buf[0:], s.Name)
	binary.LittleEndian.PutUint32(buf[4:], s.Type)
	binary.LittleEndian.PutUint64(buf[8:], s.Flags)
	binary.LittleEndian.PutUint64(buf[16:], s.Addr)
	binary.LittleEndian.PutUint64(buf[24:], s.Offset)
	binary.LittleEndian.PutUint64(buf[32:], s.Size)
	binary.LittleEndian.PutUint32(buf[40:], s.Link)
	binary.LittleEndian.PutUint32(buf[44:], s.Info)
	binary.LittleEndian.PutUint64(buf[48:], s.Alignment)
	binary.LittleEndian.PutUint64(buf[56:], s.EntrySize)
	return buf
}

type ELF64_Sym struct {
	Name  uint32
	Info  uint16 //unsigned char
	Other uint16 //unsigned char
	Shndx uint16
	Value uint64
	Size  uint64
}

func NewInfo(bind uint16, ty uint16) uint16 {
	return ((bind << 4) + (ty & 0xf))
}

func (s *ELF64_Sym) Dump() []byte {
	buf := make([]byte, s.Size)
	binary.LittleEndian.PutUint32(buf[0:], s.Name)
	binary.LittleEndian.PutUint16(buf[4:], s.Info)
	binary.LittleEndian.PutUint16(buf[6:], s.Other)
	binary.LittleEndian.PutUint16(buf[8:], s.Shndx)
	binary.LittleEndian.PutUint64(buf[10:], s.Value)
	binary.LittleEndian.PutUint64(buf[18:], s.Size)
	return buf
}

const (
	/* st_other */
	STV_DEFAULT   = 0 /* Default symbol visibility rules */
	STV_INTERNAL  = 1 /* Processor specific hidden class */
	STV_HIDDEN    = 2 /* Sym unavailable in other modules */
	STV_PROTECTED = 3 /* Not preemptible, not exported */

	/* st_bind */
	STB_LOCAL  = 0
	STB_GLOBAL = 1
	STB_WEAR   = 2
	STB_LOOS   = 10
	STB_HIOS   = 12
	STB_LOPROC = 13
	STB_HIPROC = 15

	/* st_type */
	STT_NOTYPE        = 0  /* Symbol type is unspecified */
	STT_OBJECT        = 1  /* Symbol is a data object */
	STT_FUNC          = 2  /* Symbol is a code object */
	STT_SECTION       = 3  /* Symbol associated with a section */
	STT_FILE          = 4  /* Symbol's name is file name */
	STT_COMMON        = 5  /* Symbol is a common data object */
	STT_TLS           = 6  /* Symbol is thread-local data object*/
	STT_NUM           = 7  /* Number of defined types.  */
	STT_LOOS          = 10 /* Start of OS-specific */
	STT_GNU_IFUNC     = 10 /* Symbol is indirect code object */
	STT_HIOS          = 12 /* End of OS-specific */
	STT_LOPROC        = 13 /* Start of processor-specific */
	STT_HIPROC        = 15 /* End of processor-specific */
	SHT_NULL          = 0
	SHT_PROGBITS      = 1
	SHT_SYMTAB        = 2
	SHT_STRTAB        = 3
	SHT_RELA          = 4
	SHT_HASH          = 5
	SHT_DYNAMIC       = 6
	SHT_NOTE          = 7
	SHT_NOBITS        = 8
	SHT_REL           = 9
	SHT_SHLIB         = 10
	SHT_DYNSYM        = 11
	SHT_INIT_ARRAY    = 14
	SHT_FINI_ARRAY    = 15
	SHT_PREINIT_ARRAY = 16
	SHT_GROUP         = 17
	SHT_SYNTAB_SHNDX  = 18
	SHT_NUM           = 19
	SHT_GNU_HASH      = 0x6ffffff6
	SHT_GNU_VERNEED   = 0x6ffffffe
	SHT_GNU_VERSYM    = 0x6fffffff
	SHT_LOPROC        = 0x70000000
	SHT_HIPROC        = 0x7fffffff
	SHT_LOUSER        = 0x80000000
	SHT_HIUSER        = 0xffffffff

	/* special section indexes */
	SHN_UNDEF     = 0
	SHN_LORESERVE = 0xff00
	SHN_LOPROC    = 0xff00
	SHN_HIPROC    = 0xff1f
	SHN_LIVEPATCH = 0xff20
	SHN_ABS       = 0xfff1
	SHN_COMMON    = 0xfff2
	SHN_HIRESERVE = 0xffff

	/* sh_flags */
	SHF_WRITE            = (1 << 0)   /* Writable */
	SHF_ALLOC            = (1 << 1)   /* Occupies memory during execution */
	SHF_EXECINSTR        = (1 << 2)   /* Executable */
	SHF_MERGE            = (1 << 4)   /* Might be merged */
	SHF_STRINGS          = (1 << 5)   /* Contains nul-terminated strings */
	SHF_INFO_LINK        = (1 << 6)   /* `sh_info' contains SHT index */
	SHF_LINK_ORDER       = (1 << 7)   /* Preserve order after combining */
	SHF_OS_NONCONFORMING = (1 << 8)   /* Non-standard OS specific handling required */
	SHF_GROUP            = (1 << 9)   /* Section is member of a group.  */
	SHF_TLS              = (1 << 10)  /* Section hold thread-local data.  */
	SHF_COMPRESSED       = (1 << 11)  /* Section with compressed data. */
	SHF_MASKOS           = 0x0ff00000 /* OS-specific.  */
	SHF_MASKPROC         = 0xf0000000 /* Processor-specific */
	SHF_ORDERED          = (1 << 30)  /* Special ordering requirement  (Solaris).  */
	SHF_EXCLUDE          = (1 << 31)  /* Section is excluded unless  referenced or allocated (Solaris).*/
)

/* Segment */

const (
	PT_NULL         = 0          /* Program header table entry unused */
	PT_LOAD         = 1          /* Loadable program segment */
	PT_DYNAMIC      = 2          /* Dynamic linking information */
	PT_INTERP       = 3          /* Program interpreter */
	PT_NOTE         = 4          /* Auxiliary information */
	PT_SHLIB        = 5          /* Reserved */
	PT_PHDR         = 6          /* Entry for header table itself */
	PT_TLS          = 7          /* Thread-local storage segment */
	PT_NUM          = 8          /* Number of defined types */
	PT_LOOS         = 0x60000000 /* Start of OS-specific */
	PT_GNU_EH_FRAME = 0x6474e550 /* GCC .eh_frame_hdr segment */
	PT_GNU_STACK    = 0x6474e551 /* Indicates stack executability */
	PT_GNU_RELRO    = 0x6474e552 /* Read-only after relocation */
	PT_LOSUNW       = 0x6ffffffa
	PT_SUNWBSS      = 0x6ffffffa /* Sun Specific segment */
	PT_SUNWSTACK    = 0x6ffffffb /* Stack segment */
	PT_HISUNW       = 0x6fffffff
	PT_HIOS         = 0x6fffffff /* End of OS-specific */
	PT_LOPROC       = 0x70000000 /* Start of processor-specific */
	PT_HIPROC       = 0x7fffffff /* End of processor-specific */
	PF_R            = 0x4
	PF_W            = 0x2
	PF_X            = 0x1
)

type ELF64_Phdr struct {
	Type         ELF64_Word
	Flags        ELF64_Word
	Offset       ELF64_Off
	VirtualAddr  ELF64_Addr
	PhysicalAddr ELF64_Addr
	SegmentSize  ELF64_Xword
	MemorySize   ELF64_Xword
	Alignment    ELF64_Xword
}
