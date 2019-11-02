use super::super::ir::tac::{Operand, Tac};
use super::Optimizer;
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
        let tacs = self.tacs.clone();
        for (idx, t) in tacs.iter().enumerate() {
            match t {
                Tac::EX(_lv, _op, lch, rch) => {
                    if let Some(_def_idx) = self.check_duplicate(lch.clone(), reach_in[idx].clone())
                    {
                    }
                    if let Some(_def_idx) = self.check_duplicate(rch.clone(), reach_in[idx].clone())
                    {
                    }
                }
                Tac::UNEX(_lv, _op, ch) => {
                    if let Some(_def_idx) = self.check_duplicate(ch.clone(), reach_in[idx].clone())
                    {
                    }
                }
                Tac::LET(_lv, ch) => {
                    if let Some(_def_idx) = self.check_duplicate(ch.clone(), reach_in[idx].clone())
                    {
                    }
                }
                _ => (),
            }
        }
        self.tacs = tacs;
    }
    fn check_duplicate(&mut self, _lv: Operand, reach_in: BTreeSet<Operand>) -> Option<usize> {
        let ret_idx: Option<usize> = None;
        for _idx in reach_in.iter() {}
        return ret_idx;
    }
}
