use super::super::parse::node::{Func, Node};
use super::manager::Manager;

type TypeSize = usize;
type Pointer = Option<Box<Type>>;
#[derive(Clone)]
pub enum Type {
    INTEGER(i128, TypeSize, Pointer), // INTEGER(INTTYPE) in future
    UNKNOWN,                          // INTEGER(INTTYPE) in future
}

impl Manager {
    pub fn semantics(&mut self) {
        let func_num: usize = self.functions.len();
        let mut idx: usize = 0;
        loop {
            if idx == func_num {
                break;
            }
            let f: Func = self.functions[idx].clone();
            for n in f.stmts {
                self.walk(n);
            }
            idx += 1;
        }
    }
    fn walk(&mut self, n: Node) -> Type {
        match n {
            //Node::UNARY(op,binner,otype),
            Node::NUMBER(ty) => ty,
            //Node::RETURN(bstmt),
            //Node::IF(bcond,bstmt),
            _ => Type::UNKNOWN,
        }
    }
}
