use super::Optimizer;
use crate::compile::ir::tac::Operand;

use std::collections::BTreeSet;

impl Optimizer {
    pub fn reaching_definition(&mut self) {
        let mut reach_in: Vec<BTreeSet<Operand>> = vec![BTreeSet::new(); self.tacs.len()];
        let mut reach_out: Vec<BTreeSet<Operand>> = vec![BTreeSet::new(); self.tacs.len()];
        'outer: loop {
            let mut in_sets: Vec<BTreeSet<Operand>> = Vec::new();
            let mut out_sets: Vec<BTreeSet<Operand>> = Vec::new();
            for (idx, _t) in self.tacs.iter().enumerate() {
                in_sets.push(reach_in[idx].clone());
                out_sets.push(reach_out[idx].clone());
                for s in self.cfg.pred[idx].iter() {
                    reach_in[idx] = &reach_in[idx] | &reach_out[*s];
                }
                reach_out[idx] = &self.cfg.used[idx] | &(&reach_in[idx] - &self.cfg.def[idx]);
            }
            let mut chg_flg: bool = true;
            for idx in 0..reach_in.len() {
                if reach_in[idx] != in_sets[idx] {
                    chg_flg = false;
                }
                if reach_out[idx] != out_sets[idx] {
                    chg_flg = false;
                }
            }
            if chg_flg {
                break 'outer;
            }
        }
    }
}
