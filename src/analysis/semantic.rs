use super::super::parse::node;
pub struct Environment {
    sym_tables: HashMap<String, HashMap<String, Symbol>>,
}
pub struct Symbol {
    name: String,
    ty: TokenType, //change another struct in future
}

pub fn walk(mut nodes: Vec<node::Node>) -> Environment {
    match n.ty {
        _ => println!("invalid"),
    }
}

pub fn semantic(nodes: Vec<node::Node>) -> Environment {}
