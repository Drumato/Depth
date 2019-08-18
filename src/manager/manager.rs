use super::super::parse::node::Func;
pub struct Manager {
    pub functions: Vec<Func>,
}

impl Manager {
    pub fn new(functions: Vec<Func>) -> Manager {
        Manager {
            functions: functions,
        }
    }
}
