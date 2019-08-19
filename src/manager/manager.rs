use super::super::ir::hi::HIR;
use super::super::parse::node::Func;
pub struct Manager {
    pub functions: Vec<Func>,
    pub hirs: Vec<HIR>,
    pub regnum: usize,
}
