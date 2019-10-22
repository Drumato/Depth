use super::super::ir::tac::{Operand, Tac};
use super::Optimizer;
use std::collections::{HashMap, HashSet};
impl Optimizer {
    pub fn build_cfg(&mut self) {
        let label_map: HashMap<String, usize> = self.build_labelmap();
        let tacs = self.tacs.clone();
        for (n, t) in tacs.iter().enumerate() {
            match t {
                Tac::EX(_lv, _, _lop, _rop) => {
                    self.add_pred(n, n - 1);
                    self.add_succ(n, n + 1);
                }
                Tac::UNEX(_lv, _, _op) => {
                    self.add_pred(n, n - 1);
                    self.add_succ(n, n + 1);
                }
                Tac::PUSHARG(_, _) => {
                    self.add_pred(n, n - 1);
                    self.add_succ(n, n + 1);
                }
                Tac::PARAM(_, _op) => {
                    self.add_pred(n, n - 1);
                    self.add_succ(n, n + 1);
                }
                Tac::LET(_lv, _op) => {
                    self.add_pred(n, n - 1);
                    self.add_succ(n, n + 1);
                }
                Tac::RET(_op) => {
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
                Tac::IFF(_op, label) => {
                    self.add_pred(n, n - 1);
                    self.add_succ(n, n + 1);
                    if let Some(goto) = label_map.get(label) {
                        self.add_succ(n, *goto);
                        self.add_pred(*goto, n);
                    }
                }
                Tac::FUNCNAME(_) => {
                    if n != 0 && !self.check_goto(n - 1) {
                        self.add_pred(n, n - 1);
                    }
                    self.add_succ(n, n + 1);
                }
                Tac::PROLOGUE(_) => {
                    if n != 0 && !self.check_goto(n - 1) {
                        self.add_pred(n, n - 1);
                    }
                    self.add_succ(n, n + 1);
                }
                Tac::LABEL(_) => {
                    if n != 0 && !self.check_goto(n - 1) {
                        self.add_pred(n, n - 1);
                    }
                    self.add_succ(n, n + 1);
                }
            }
        }
    }
    pub fn build_cfg_for_liveness(&mut self) {
        self.cfg.used = vec![HashSet::new(); self.tacs.len()];
        self.cfg.def = vec![HashSet::new(); self.tacs.len()];
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
                    self.living.insert(lv.clone(), (0, 0));
                }
                Tac::UNEX(lv, _, op) => {
                    self.cfg.def[n].insert(lv.clone());
                    if self.check_use_value(&op) {
                        self.cfg.used[n].insert(op.clone());
                    }
                    self.living.insert(lv.clone(), (0, 0));
                }
                Tac::PUSHARG(_, _) => {}
                Tac::PARAM(_, op) => {
                    if self.check_use_value(&op) {
                        self.cfg.used[n].insert(op.clone());
                    }
                }
                Tac::LET(lv, op) => {
                    self.cfg.def[n].insert(lv.clone());
                    if self.check_use_value(&op) {
                        self.cfg.used[n].insert(op.clone());
                    }
                    self.living.insert(lv.clone(), (0, 0));
                }
                Tac::RET(op) => {
                    if self.check_use_value(&op) {
                        self.cfg.used[n].insert(op.clone());
                    }
                }
                Tac::GOTO(_label) => {}
                Tac::IFF(op, _label) => {
                    if self.check_use_value(&op) {
                        self.cfg.used[n].insert(op.clone());
                    }
                }
                Tac::FUNCNAME(_) => {}
                Tac::PROLOGUE(_) => {}
                Tac::LABEL(_) => {}
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
            Operand::REG(_, _, _, _) => true,
            Operand::ID(_, _, _, _) => true,
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
