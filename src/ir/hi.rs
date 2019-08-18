use super::super::manager::manager::Manager;
use super::super::parse::node;
pub enum HIR {
    LOAD(usize, i128),
    ADD(usize, usize),
    SUB(usize, usize),
    RETURN(usize),
}

impl HIR {
    pub fn string(&self) -> String {
        match self {
            HIR::LOAD(reg, val) => format!("load {} to {}", val, reg),
            HIR::ADD(lr, rr) => format!("add {} and {}", lr, rr),
            HIR::SUB(lr, rr) => format!("sub {} and {}", lr, rr),
            HIR::RETURN(reg) => format!("return {}", reg),
        }
    }
}

pub fn gen_hir(nodes: Vec<node::Node>) -> Manager {
    let mut manager: Manager = Manager {
        hirs: Vec::new(),
        regnum: 0,
    };
    for n in nodes {
        manager.gen_expr(n);
    }
    let return_reg: usize = manager.regnum - 1;
    manager.hirs.push(HIR::RETURN(return_reg));
    manager
}
