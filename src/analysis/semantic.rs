use super::super::lex::token;
use super::super::parse::node;
use std::collections::HashMap;
use token::TokenType;
pub struct Environment {
    sym_tables: HashMap<String, HashMap<String, Symbol>>,
}
impl Environment {
    pub fn new(map: HashMap<String, HashMap<String, Symbol>>) -> Self {
        Self { sym_tables: map }
    }
    /*   pub fn merge<'a>(
        m1: &HashMap<String, HashMap<String, Symbol>>,
        m2: &HashMap<String, HashMap<String, Symbol>>,
    ) -> Self {
        let mut new_map: HashMap<String, HashMap<String, Symbol>> = HashMap::new();
        for (k, v) in m1.iter() {
            new_map.insert(k.clone(), v.clone());
        }
        for (k, v) in m2.iter() {
            new_map.insert(k.clone(), v.clone());
        }
        Self {
            sym_tables: new_map,
        }
    }
    */
    pub fn new_symbol(&mut self, namespace: String, map: HashMap<String, Symbol>) {
        self.sym_tables.insert(namespace, map);
    }
}

pub struct Symbol {
    ty: SymbolType,
}

impl Symbol {
    pub fn new(ty: SymbolType) -> Self {
        Self { ty: ty }
    }
    pub fn new_ident(name: String, ty: TokenType) -> Self {
        Symbol::new(SymbolType::ID(name, ty))
    }
    pub fn new_type(name: String, ty: TokenType) -> Self {
        Symbol::new(SymbolType::TYPE(name, ty))
    }
}

pub enum SymbolType {
    ID(String, TokenType),
    TYPE(String, TokenType),
}

pub fn walk(node: node::Node, syms: &mut Environment) {
    match node.ty {
        _ => println!(""),
    }
}

pub fn semantic(nodes: &mut Vec<node::Node>) -> Environment {
    let sym_tables: HashMap<String, HashMap<String, Symbol>> = HashMap::new();
    let mut env: Environment = Environment::new(sym_tables);
    for n in nodes {
        walk(n.clone(), &mut env);
    }
    env
}
