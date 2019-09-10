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
    }
    fn gen_inst(&mut self, num: &usize) {
        let info: &Info = self.info_map.get(num).unwrap();
        match info.inst_name.as_str() {
            "mov" => {
                self.codes.push(0x48);
                self.codes.push(0xc7);
                self.codes.push(0xc0); // consider reg as rax
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
    generator.codes
}
