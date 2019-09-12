use super::parse::{Info, Inst, Operand};
use std::collections::HashMap;
struct Generator {
    insts: Vec<Inst>,
    info_map: HashMap<usize, Info>,
    codes: Vec<u8>,
}
impl Generator {
    fn gen(&mut self) {
        let insts: Vec<Inst> = self.insts.to_vec();
        for inst in insts.iter() {
            match inst {
                &Inst::BINARG(num) | &Inst::UNARG(num) | &Inst::NOARG(num) => {
                    self.gen_inst(&num);
                }
            }
        }
    }
    fn gen_inst(&mut self, num: &usize) {
        let info: &Info = self.info_map.get(&num).unwrap();
        match info.inst_name.as_str() {
            "add" => {
                self.codes.push(self.set_rexprefix(&info.lop, &info.rop));
                let modrm: u8 = self.set_modrm(&info.lop, &info.rop); // mod field of ModR/M
                if let Some(Operand::IMM(_value)) = info.rop {
                } else {
                    self.codes.push(0x01);
                }
                self.codes.push(modrm);
            }
            "cmp" => {
                self.codes.push(self.set_rexprefix(&info.lop, &info.rop));
                if let Some(Operand::IMM(value)) = info.rop {
                    self.codes.push(0x81);
                    self.codes.push(self.set_modrm(&info.rop, &info.lop));
                    self.gen_immediate(value);
                } else {
                    self.codes.push(0x3b);
                    self.codes.push(self.set_modrm(&info.rop, &info.lop));
                }
            }
            "cqo" => {
                self.codes.push(0x48);
                self.codes.push(0x99);
            }
            "idiv" => {
                self.codes.push(0x49);
                self.codes.push(0xf7);
                let mut modrm: u8 = 0xf8;
                if let Some(reg) = &info.lop {
                    modrm |= reg.reg_number();
                }
                self.codes.push(modrm);
            }
            "imul" => {
                self.codes.push(0x49);
                self.codes.push(0xf7);
                let mut modrm: u8 = 0xe8;
                if let Some(reg) = &info.lop {
                    modrm |= reg.reg_number();
                }
                self.codes.push(modrm);
            }
            "mov" => {
                self.codes.push(self.set_rexprefix(&info.lop, &info.rop));
                let modrm: u8 = self.set_modrm(&info.lop, &info.rop); // mod field of ModR/M
                if let Some(Operand::IMM(value)) = info.rop {
                    self.codes.push(0xc7); // mov reg, immediate
                    self.codes.push(modrm);
                    self.gen_immediate(value);
                } else {
                    self.codes.push(0x89); // mov reg, reg
                    self.codes.push(modrm);
                }
            }
            "push" => {
                let mut opcode: u8 = 0x50;
                if let Some(reg) = &info.lop {
                    if let Operand::REG(name) = reg {
                        match name.as_str() {
                            "r8" | "r9" | "r10" | "r11" | "r12" | "r13" | "r14" | "r15" => {
                                self.codes.push(0x41);
                            }
                            _ => (),
                        }
                    }
                    opcode |= reg.reg_number();
                }
                self.codes.push(opcode);
            }
            "pop" => {
                let mut opcode: u8 = 0x58;
                if let Some(reg) = &info.lop {
                    if let Operand::REG(name) = reg {
                        match name.as_str() {
                            "r8" | "r9" | "r10" | "r11" | "r12" | "r13" | "r14" | "r15" => {
                                self.codes.push(0x41);
                            }
                            _ => (),
                        }
                    }
                    opcode |= reg.reg_number();
                }
                self.codes.push(opcode);
            }
            "ret" => {
                self.codes.push(0xc3);
            }
            "setl" => {
                self.codes.push(0x0f);
                self.codes.push(0x9c);
                self.codes.push(self.set_modrm(&info.lop, &info.rop));
            }
            "sub" => {
                self.codes.push(self.set_rexprefix(&info.lop, &info.rop));
                let modrm: u8 = self.set_modrm(&info.lop, &info.rop); // mod field of ModR/M
                if let Some(Operand::IMM(_value)) = info.rop {
                } else {
                    self.codes.push(0x29);
                }
                self.codes.push(modrm);
            }
            "syscall" => {
                self.codes.push(0x0f);
                self.codes.push(0x05);
            }
            _ => (),
        }
    }
    fn gen_immediate(&mut self, value: i128) {
        match value {
            n if n <= 255 => {
                self.codes.push(value as u8);
                self.codes.push(0x00);
                self.codes.push(0x00);
                self.codes.push(0x00);
            }
            n if n <= 65535 => {
                self.codes.push((value >> 8) as u8);
                self.codes.push(value as u8);
                self.codes.push(0x00);
                self.codes.push(0x00);
            }
            n if n <= 4294967295 => {
                self.codes.push((value >> 16) as u8);
                self.codes.push((value >> 8) as u8);
                self.codes.push(value as u8);
                self.codes.push(0x00);
            }
            _ => {
                self.codes.push((value >> 24) as u8);
                self.codes.push((value >> 16) as u8);
                self.codes.push((value >> 8) as u8);
                self.codes.push(value as u8);
            }
        }
    }
    fn set_rexprefix(&self, lop: &Option<Operand>, rop: &Option<Operand>) -> u8 {
        // 0100 | REX.w	REX.r REX.x REX.b
        let mut rexprefix: u8 = 0x40;
        if let Some(Operand::REG(name)) = lop {
            if name.starts_with("r") {
                rexprefix |= 0x08;
            }
            match name.as_str() {
                "r8" | "r9" | "r10" | "r11" | "r12" | "r13" | "r14" | "r15" => {
                    rexprefix |= 0x01;
                }
                _ => (),
            }
        }
        if let Some(Operand::REG(name)) = rop {
            if name.starts_with("r") {
                rexprefix |= 0x08;
            }
            match name.as_str() {
                "r8" | "r9" | "r10" | "r11" | "r12" | "r13" | "r14" | "r15" => {
                    rexprefix |= 0x04;
                }
                _ => (),
            }
        }
        rexprefix
    }
    fn set_modrm(&self, lop: &Option<Operand>, rop: &Option<Operand>) -> u8 {
        // mod(2 bits) | reg(3 bits) | r/m(3 bits)
        let mut modrm: u8 = 0xc0; // the mod filed of modr/m
        if let Some(reg) = lop {
            modrm |= reg.reg_number();
        }
        match rop {
            Some(Operand::IMM(_)) => (),
            Some(reg) => {
                modrm |= reg.reg_number() << 3;
            }
            None => (),
        }
        modrm
    }
}
pub fn generate(
    inst_map: HashMap<String, Vec<Inst>>,
    info_map: HashMap<usize, Info>,
) -> HashMap<String, Vec<u8>> {
    let mut generator: Generator = Generator {
        insts: Vec::new(),
        info_map: info_map,
        codes: Vec::new(),
    };
    let mut symbol_map: HashMap<String, Vec<u8>> = HashMap::new();
    for (symbol, insts) in inst_map.iter() {
        generator.insts = insts.to_vec();
        generator.gen();
        let md = generator.codes.len() % 4;
        for _ in 0..(4 - md) {
            generator.codes.push(0x00);
        }
        symbol_map.insert(symbol.to_string(), generator.codes.to_vec());
        generator.codes = Vec::new();
    }
    symbol_map
}
