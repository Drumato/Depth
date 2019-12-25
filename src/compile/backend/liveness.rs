use super::Optimizer;
use crate::compile::ir::tac::Operand;

use std::collections::BTreeSet;

impl Optimizer {
    pub fn liveness(&mut self) {
        let mut live_in: Vec<BTreeSet<Operand>> = vec![BTreeSet::new(); self.tacs.len()];
        let mut live_out: Vec<BTreeSet<Operand>> = vec![BTreeSet::new(); self.tacs.len()];
        'outer: loop {
            let mut in_sets: Vec<BTreeSet<Operand>> = Vec::new();
            let mut out_sets: Vec<BTreeSet<Operand>> = Vec::new();
            for (idx, _t) in self.tacs.iter().rev().enumerate() {
                in_sets.push(live_in[idx].clone());
                out_sets.push(live_out[idx].clone());
                for s in self.cfg.succ[idx].iter() {
                    live_out[idx] = &live_out[idx] | &live_in[*s];
                }
                live_in[idx] = &self.cfg.used[idx] | &(&live_out[idx] - &self.cfg.def[idx]);
            }
            let mut chg_flg: bool = true;
            for idx in 0..live_in.len() {
                if live_in[idx] != in_sets[idx] {
                    chg_flg = false;
                }
                if live_out[idx] != out_sets[idx] {
                    chg_flg = false;
                }
            }
            if chg_flg {
                break 'outer;
            }
        }
        for (op, range) in self.living.iter_mut() {
            for (idx, _t) in self.tacs.iter().enumerate() {
                if !live_in[idx].contains(op) && live_out[idx].contains(op) {
                    range.0 = idx;
                }
                if live_in[idx].contains(op) && !live_out[idx].contains(op) {
                    range.1 = idx;
                }
            }
        }
    }
}
