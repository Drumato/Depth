use super::super::ir::tac::Operand;
use super::Optimizer;
use std::collections::HashSet;

impl Optimizer {
    pub fn liveness(&mut self) {
        let tacs = self.tacs.clone();
        let mut live_in: Vec<HashSet<Operand>> = vec![HashSet::new(); tacs.len()];
        let mut live_out: Vec<HashSet<Operand>> = vec![HashSet::new(); tacs.len()];
        'outer: loop {
            let mut in_sets: Vec<HashSet<Operand>> = Vec::new();
            let mut out_sets: Vec<HashSet<Operand>> = Vec::new();
            for (idx, _t) in tacs.iter().rev().enumerate() {
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
        self.live_in = live_in;
        self.live_out = live_out;
        for (idx, i) in self.live_in.iter().enumerate() {
            eprintln!(
                "\tlive_in[{}] {:?}  live_out[{}] {:?}",
                idx, i, idx, &self.live_out[idx]
            );
        }
    }
}
