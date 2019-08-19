use super::super::parse::node::Node;
use super::manager::Manager;

type TypeSize = usize;
type Pointer = Option<Box<Type>>;
#[derive(Clone)]
pub enum Type {
    INTEGER(i128, TypeSize, Pointer), // INTEGER(INTTYPE) in future
}

impl Manager {
    pub fn semantics(nodes: &mut Vec<Node>) {
        for n in nodes {
            match n {
                Node::BINOP(ref mut t, blhs, blrs, oty) => {}
                _ => (),
            }
        }
    }
}
