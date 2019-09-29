use super::super::ir::tac::Operand;
use super::Optimizer;
use std::collections::{BTreeMap, HashSet};

static AVAILABLE_REG: usize = 9; // rax, rdx, rcx, rdi, rsi, r8, r9, r10, r11

impl Optimizer {
    pub fn regalloc(&mut self) {
        let mut reg_maps: Vec<HashSet<Operand>> = vec![HashSet::new(); self.tacs.len()];
        let mut graph: Vec<Vec<bool>> = vec![vec![false; self.living.len()]; self.living.len()];
        let mut vars: BTreeMap<Operand, usize> = BTreeMap::new();
        for (idx, (op, _range)) in self.living.iter().enumerate() {
            vars.insert(op.clone(), idx);
        }
        for (idx, t) in self.tacs.iter_mut().enumerate() {
            for (op, range) in self.living.iter() {
                if range.0 <= idx && idx <= range.1 {
                    reg_maps[idx].insert(op.clone());
                }
            }
            for op in reg_maps[idx].iter() {
                for op2 in reg_maps[idx].iter() {
                    if op == op2 {
                        continue;
                    }
                    graph[*vars.get(op).unwrap()][*vars.get(op2).unwrap()] = true;
                }
            }
        }
        /*
        println!("graph {{\n");
        for (g, edges) in graph.iter() {
            for edge in edges.iter() {
                println!("\t{} -- {};\n", g.string(), edge.string());
            }
        }
        println!("}} \n");
        */
    }
}
