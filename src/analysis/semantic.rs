extern crate drumatech;
use super::super::lex::token;
use super::super::parse::{error, node};
use drumatech::conv;
use std::collections::HashMap;
use token::TokenType;
pub struct Environment {
    pub sym_tables: HashMap<String, Symbol>,
    pub func_tables: HashMap<String, Vec<u64>>,
}
impl Environment {
    pub fn new() -> Self {
        let sym_tables: HashMap<String, Symbol> = HashMap::new();
        let func_tables: HashMap<String, Vec<u64>> = HashMap::new();
        Self {
            sym_tables: sym_tables,
            func_tables: func_tables,
        }
    }
    pub fn semantic(&mut self, nodes: Vec<node::Node>) {
        for n in nodes.iter() {
            match n.ty.clone() {
                node::NodeType::FUNC(func_name, _, _, nodes, _) => {
                    self.analyze_func(func_name, nodes)
                }
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
    fn new_stmt(&mut self, env_name: String, stmt_num: u64) {
        if !self.func_tables.contains_key(&env_name) {
            self.func_tables.insert(env_name.clone(), vec![stmt_num]);
        } else {
            if let Some(vector) = self.func_tables.get_mut(&env_name) {
                vector.push(stmt_num);
            }
        }
    }
    fn analyze_func(&mut self, func_name: String, nodes: Box<Vec<node::Node>>) {
        for n in nodes.iter() {
            self.new_stmt(func_name.clone(), n.id);
            match n.ty.clone() {
                node::NodeType::LETS(_, ident_name, type_name, n) => {
                    self.analyze_lets(func_name.clone(), ident_name.clone(), type_name.clone(), n)
                }
                _ => (),
            }
        }
    }
    fn analyze_lets(
        &mut self,
        env_name: String,
        ident_name: String,
        type_name: TokenType,
        n: Box<node::Node>,
    ) {
        let node: node::Node = conv::open_box(n);
        match node.ty {
            node::NodeType::BINOP(ty, lchild, rchild, _) => self.check_types(ty, lchild, rchild),
            _ => (),
        }
        self.new_ident(env_name, ident_name, type_name)
    }
    fn check_types(&mut self, ty: TokenType, lchild: Box<node::Node>, rchild: Box<node::Node>) {}
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
