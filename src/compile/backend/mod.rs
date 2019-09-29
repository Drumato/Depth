use super::ir::tac::{Operand, Tac};
use std::collections::HashSet;
use std::fs::File;
use std::io::Write;
pub mod data_flow;
pub mod liveness;

pub struct Optimizer {
    pub tacs: Vec<Tac>,
    pub cfg: ControlFlowGraph,
    pub live_in: Vec<HashSet<Operand>>,
    pub live_out: Vec<HashSet<Operand>>,
}

impl Optimizer {
    pub fn new(tac_vec: Vec<Tac>) -> Self {
        let len: usize = tac_vec.len();
        Self {
            tacs: tac_vec,
            cfg: ControlFlowGraph::new(len),
            live_in: Vec::new(),
            live_out: Vec::new(),
        }
    }
    pub fn dump_cfg(&self) {
        let mut out: String = String::new();
        out += "digraph { \n";
        for (idx, t) in self.tacs.iter().enumerate() {
            out += &(format!("\t{}[label=\"{}\",shape=\"box\"];\n", idx, t.string()).as_str());
        }
        for idx in 0..self.tacs.len() {
            for pred in self.cfg.pred[idx].iter() {
                out += &(format!("\t{} -> {};\n", pred, idx).as_str());
            }
        }
        out += "}";
        let file_name: String = "cfg.dot".to_string();
        let mut file = File::create(file_name).unwrap();
        file.write_all(out.as_bytes()).unwrap();
    }
}

pub struct ControlFlowGraph {
    succ: Vec<HashSet<usize>>,
    pred: Vec<HashSet<usize>>,
    used: Vec<HashSet<Operand>>,
    def: Vec<HashSet<Operand>>,
}
impl ControlFlowGraph {
    fn new(len: usize) -> Self {
        Self {
            succ: vec![HashSet::new(); len],
            pred: vec![HashSet::new(); len],
            used: vec![HashSet::new(); len],
            def: vec![HashSet::new(); len],
        }
    }
}
