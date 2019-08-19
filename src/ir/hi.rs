use super::super::manager::manager::Manager;
use super::super::parse::node;
pub enum HIR {
    LOAD(usize, i128),
    ADD(usize, usize),
    SUB(usize, usize),
    MUL(usize, usize),
    DIV(usize, usize),
    MOD(usize, usize),
    LSHIFT(usize, usize),
    RSHIFT(usize, usize),
    LT(usize, usize),
    LTEQ(usize, usize),
    GT(usize, usize),
    GTEQ(usize, usize),
    EQ(usize, usize),
    NTEQ(usize, usize),
    NEGATIVE(usize),
    RETURN(usize),
}

impl HIR {
    pub fn string(&self) -> String {
        match self {
            HIR::LOAD(reg, val) => format!("load {} to {}", val, reg),
            HIR::ADD(lr, rr) => format!("{} plus {}", lr, rr),
            HIR::SUB(lr, rr) => format!("{} minus {}", lr, rr),
            HIR::MUL(lr, rr) => format!("{} multiply {}", lr, rr),
            HIR::DIV(lr, rr) => format!("{} divided by {}", lr, rr),
            HIR::MOD(lr, rr) => format!("reminder of '{} divided by {}'", lr, rr),
            HIR::LSHIFT(lr, rr) => format!("lshift {} {} times", lr, rr),
            HIR::RSHIFT(lr, rr) => format!("rshift {} {} times", lr, rr),
            HIR::LT(lr, rr) => format!("{} less than {}", lr, rr),
            HIR::LTEQ(lr, rr) => format!("{} less than or equal {}", lr, rr),
            HIR::GT(lr, rr) => format!("{} greater than {}", lr, rr),
            HIR::GTEQ(lr, rr) => format!("{} greater than or equal {}", lr, rr),
            HIR::EQ(lr, rr) => format!("{} equal {}", lr, rr),
            HIR::NTEQ(lr, rr) => format!("{} not equal {}", lr, rr),
            HIR::NEGATIVE(reg) => format!("negative {} ", reg),
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
        manager.gen_stmt(n);
    }
    manager
}
