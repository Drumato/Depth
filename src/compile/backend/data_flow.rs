use super::super::ir::tac::{Operand, Tac};
use super::Optimizer;
use std::collections::HashMap;
impl Optimizer {
    pub fn build_cfg(&mut self) {
        let label_map: HashMap<String, usize> = self.build_labelmap();
        let tacs = self.tacs.clone();
        for (n, t) in tacs.iter().enumerate() {
            match t {
                Tac::EX(lv, _, lop, rop) => {
                    self.cfg.def[n].insert(lv.clone());
                    if self.check_use_value(&lop) {
                        self.cfg.used[n].insert(lop.clone());
                    }
                    if self.check_use_value(&rop) {
                        self.cfg.used[n].insert(rop.clone());
                    }
                    self.add_pred(n, n - 1);
                    self.add_succ(n, n + 1);
                }
                Tac::LET(lv, op) => {
                    self.cfg.def[n].insert(lv.clone());
                    if self.check_use_value(&op) {
                        self.cfg.used[n].insert(op.clone());
                    }
                    self.add_pred(n, n - 1);
                    self.add_succ(n, n + 1);
                }
                Tac::RET(op) => {
                    if self.check_use_value(&op) {
                        self.cfg.used[n].insert(op.clone());
                    }
                    self.add_pred(n, n - 1);
                    self.add_succ(n, n + 1);
                }
                Tac::GOTO(label) => {
                    self.add_pred(n, n - 1);
                    if let Some(goto) = label_map.get(label) {
                        self.add_succ(n, *goto);
                        self.add_pred(*goto, n);
                    }
                }
                Tac::IFF(op, label) => {
                    self.add_pred(n, n - 1);
                    if self.check_use_value(&op) {
                        self.cfg.used[n].insert(op.clone());
                    }
                    self.add_succ(n, n + 1);
                    if let Some(goto) = label_map.get(label) {
                        self.add_succ(n, *goto);
                        self.add_pred(*goto, n);
                    }
                }
                Tac::LABEL(_) => {
                    if n != 0 && !self.check_goto(n - 1) {
                        self.add_pred(n, n - 1);
                    }
                    self.add_succ(n, n + 1);
                }
                _ => (),
            }
        }
    }
    fn build_labelmap(&self) -> HashMap<String, usize> {
        let mut map: HashMap<String, usize> = HashMap::new();
        for (idx, t) in self.tacs.iter().enumerate() {
            if let Tac::LABEL(name) = t {
                map.insert(name.to_string(), idx);
            }
        }
        map
    }
    fn check_use_value(&self, op: &Operand) -> bool {
        match op {
            Operand::REG(_, _) => true,
            Operand::ID(_) => true,
            _ => false,
        }
    }
    fn check_goto(&self, n: usize) -> bool {
        match self.tacs[n] {
            Tac::IFF(_, _) => true,
            Tac::GOTO(_) => true,
            _ => false,
        }
    }
    fn add_succ(&mut self, n: usize, edge: usize) {
        if edge < self.tacs.len() {
            self.cfg.succ[n].insert(edge);
        }
    }
    fn add_pred(&mut self, n: usize, edge: usize) {
        self.cfg.pred[n].insert(edge);
    }
}
