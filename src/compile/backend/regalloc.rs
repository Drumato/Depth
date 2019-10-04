use super::super::ir::tac::{Operand, Tac};
use super::Optimizer;
use std::collections::BTreeMap;

static AVAILABLE_X64: usize = 9;
impl Optimizer {
    pub fn regalloc(&mut self) {
        use std::iter::FromIterator;
        let mut living_list = Vec::from_iter(self.living.clone());
        let mut living_list_copy = living_list.clone();
        let mut reg_map: BTreeMap<String, usize> = BTreeMap::new();
        let mut active_list: Vec<(Operand, (usize, usize))> = Vec::new();
        let mut registers: Vec<Option<usize>> = (0..9).map(|num| Some(num)).collect();
        living_list.sort_by(|&(_, r1), &(_, r2)| r1.0.cmp(&r2.0));
        for (var, range) in living_list.iter_mut() {
            let mut num: usize = 100;
            if let Some(n) = registers.iter().find(|x| x.is_some()) {
                num = n.unwrap();
            }
            match var {
                Operand::REG(ref mut _virt, ref mut phys, ref mut _oind) => {
                    if num < AVAILABLE_X64 {
                        *phys = num;
                        reg_map.insert(var.string(), num);
                        registers[num] = None;
                        for (op, r) in living_list_copy.iter_mut() {
                            *op = var.clone();
                            if range.0 <= r.0 && r.1 <= range.1 || range.0 <= r.0 && r.0 <= range.1
                            {
                                active_list.push((op.clone(), *range));
                            }
                        }
                        self.sort_active(&mut active_list);
                    } else {
                        eprintln!("spill occured (not implemented)");
                    }
                }
                _ => (),
            }
            let remove_list: Vec<usize> = active_list
                .iter()
                .filter(|(_op, r)| r.1 < range.0)
                .enumerate()
                .map(|(idx, _)| idx)
                .collect();
            for i in remove_list.iter() {
                let mut return_reg: usize = 0;
                if *i < active_list.len() {
                    if let Operand::REG(_, phys, _oind) = active_list[*i].0 {
                        return_reg = phys;
                    }
                    active_list.remove(*i);
                    registers[return_reg] = Some(return_reg);
                }
            }
        }
        self.living = living_list
            .into_iter()
            .collect::<BTreeMap<Operand, (usize, usize)>>();
        /* allocating reg into tac*/
        let mut tacs = self.tacs.clone();
        for t in tacs.iter_mut() {
            match t {
                Tac::EX(lv, _, lop, rop) => {
                    let op2 = lop.clone();
                    if let Operand::REG(ref mut _virt, ref mut phys, ref mut _oind) = lop {
                        *phys = *reg_map.get(&op2.string()).unwrap();
                    }
                    let op2 = rop.clone();
                    if let Operand::REG(ref mut _virt, ref mut phys, ref mut _oind) = rop {
                        *phys = *reg_map.get(&op2.string()).unwrap();
                    }
                    let op2 = lv.clone();
                    if let Operand::REG(ref mut _virt, ref mut phys, ref mut _oind) = lv {
                        *phys = *reg_map.get(&op2.string()).unwrap();
                    }
                }
                Tac::UNEX(lv, _, op) => {
                    let op2 = op.clone();
                    if let Operand::REG(ref mut _virt, ref mut phys, ref mut _oind) = op {
                        *phys = *reg_map.get(&op2.string()).unwrap();
                    }
                    let op2 = lv.clone();
                    if let Operand::REG(ref mut _virt, ref mut phys, ref mut _oind) = lv {
                        *phys = *reg_map.get(&op2.string()).unwrap();
                    }
                }
                Tac::RET(op) => {
                    let op2 = op.clone();
                    if let Operand::REG(ref mut _virt, ref mut phys, ref mut _oind) = op {
                        *phys = *reg_map.get(&op2.string()).unwrap();
                    }
                }
                Tac::PARAM(_, op) => {
                    let op2 = op.clone();
                    if let Operand::REG(ref mut _virt, ref mut phys, ref mut _oind) = op {
                        *phys = *reg_map.get(&op2.string()).unwrap();
                    }
                }
                Tac::LET(lv, op) => {
                    let op2 = op.clone();
                    if let Operand::REG(ref mut _virt, ref mut phys, ref mut _oind) = op {
                        *phys = *reg_map.get(&op2.string()).unwrap();
                    }
                    let op2 = lv.clone();
                    if let Operand::REG(ref mut _virt, ref mut phys, ref mut _oind) = lv {
                        *phys = *reg_map.get(&op2.string()).unwrap();
                    }
                }
                Tac::IFF(op, _label) => {
                    let op2 = op.clone();
                    if let Operand::REG(ref mut _virt, ref mut phys, ref mut _oind) = op {
                        *phys = *reg_map.get(&op2.string()).unwrap();
                    }
                }
                _ => (),
            }
        }
        self.tacs = tacs;
    }
    fn sort_active(&self, active_list: &mut Vec<(Operand, (usize, usize))>) {
        active_list.sort_by(|&(_, r1), &(_, r2)| r1.1.cmp(&r2.1));
    }
}
