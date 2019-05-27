use super::super::lex::token;
use super::super::parse::node;
use std::collections::HashMap;
use token::TokenType;
pub struct Environment {
    pub sym_tables: HashMap<String, Symbol>,
}
impl Environment {
    pub fn new() -> Self {
        let sym_tables: HashMap<String, Symbol> = HashMap::new();
        Self {
            sym_tables: sym_tables,
        }
    }
    pub fn semantic(&mut self, nodes: Vec<node::Node>) {
        for n in nodes.iter() {
            match n.ty.clone() {
                node::NodeType::FUNC(func_name, _, _, nodes) => self.analyze_func(func_name, nodes),
                _ => (),
            }
        }
    }
    fn new_ident(&mut self, env_name: String, ident_name: String, type_name: TokenType) {
        self.sym_tables.insert(
            ident_name.clone(),
            Symbol::new_ident(ident_name.clone(), type_name),
        );
    }
    fn analyze_func(&mut self, func_name: String, nodes: Box<Vec<node::Node>>) {
        for n in nodes.iter() {
            match n.ty.clone() {
                node::NodeType::LETS(_, ident_name, type_name, _) => {
                    self.analyze_lets(func_name.clone(), ident_name.clone(), type_name.clone())
                }
                _ => (),
            }
        }
    }
    fn analyze_lets(&mut self, env_name: String, ident_name: String, type_name: TokenType) {
        self.new_ident(env_name, ident_name, type_name)
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
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum SymbolType {
    ID(String, TokenType),
    TYPE(String, TokenType),
}

pub fn walk(node: node::Node, syms: &mut Environment) {
    match node.ty {
        _ => println!(""),
    }
}
