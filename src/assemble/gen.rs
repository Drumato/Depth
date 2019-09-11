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
                &Inst::BINARG(num) | &Inst::NOARG(num) => {
                    self.gen_inst(&num);
                }
            }
        }
        /*
        0000000000000000 <main>:
          0:   55                      push   rbp
          1:   48 89 e5                mov    rbp,rsp
          4:   b8 00 00 00 00          mov    eax,0x0
          9:   5d                      pop    rbp
          a:   c3                      ret
               */
    }
    fn gen_inst(&mut self, num: &usize) {
        let info: &Info = self.info_map.get(num).unwrap();
        match info.inst_name.as_str() {
            "push" => {}
            "mov" => {
                self.codes.push(0x48);
                let modrm: u8 = self.set_modrm(&info.lop, &info.rop); // mod field of ModR/M
                if let Some(Operand::IMM(_)) = info.rop {
                    self.codes.push(0xc7); // mov reg, immediate
                } else {
                    self.codes.push(0x89); // mov reg, reg
                }
                self.codes.push(modrm);
                if let Some(Operand::IMM(value)) = info.rop {
                    self.gen_immediate(value);
                }
            }
            "ret" => {
                self.codes.push(0xc3);
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
    fn set_modrm(&self, lop: &Option<Operand>, rop: &Option<Operand>) -> u8 {
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
pub fn generate(inst_map: HashMap<String, Vec<Inst>>, info_map: HashMap<usize, Info>) -> Vec<u8> {
    let mut generator: Generator = Generator {
        insts: Vec::new(),
        info_map: info_map,
        codes: Vec::new(),
    };
    for (_symbol, insts) in inst_map.iter() {
        generator.insts = insts.to_vec();
        generator.gen();
    }
    let md = generator.codes.len() % 4;
    for _ in 0..(4 - md) {
        generator.codes.push(0x00);
    }
    generator.codes
}
