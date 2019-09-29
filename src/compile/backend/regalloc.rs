use super::super::ir::tac::Operand;
use super::Optimizer;
use std::collections::{BTreeMap, HashSet};

static AVAILABLE_REG: usize = 9; // rax, rdx, rcx, rdi, rsi, r8, r9, r10, r11

impl Optimizer {
    pub fn regalloc(&mut self) {
        let mut reg_maps: Vec<HashSet<Operand>> = vec![HashSet::new(); self.tacs.len()];
        let mut graph: BTreeMap<Operand, Vec<Operand>> = BTreeMap::new();
        for (idx, t) in self.tacs.iter_mut().enumerate() {
            for (op, range) in self.living.iter() {
                if range.0 <= idx && idx <= range.1 {
                    reg_maps[idx].insert(op.clone());
                }
            }
            for op in reg_maps[idx].iter() {
                graph.insert(op.clone(), Vec::new());
            }
            for op in reg_maps[idx].iter() {
                for op2 in reg_maps[idx].iter() {
                    if op == op2 {
                        continue;
                    }
                    if let Some(edges) = graph.get_mut(op) {
                        edges.push(op2.clone());
                    }
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
