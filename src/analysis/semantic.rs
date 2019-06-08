extern crate drumatech;
use super::super::lex::token;
use super::super::parse::{error, node};
use drumatech::conv;
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
                node::NodeType::FUNC(func_name, _, _, nodes, _) => {
                    self.analyze_func(func_name, nodes)
                }
                _ => (),
            }
        }
    }
    fn new_ident(&mut self, env_name: String, ident_name: String, type_name: TokenType) {
        self.sym_tables.insert(
            ident_name.to_string(),
            Symbol::new_ident(ident_name.to_string(), type_name),
        );
    }
    fn analyze_func(&mut self, func_name: String, nodes: Vec<node::Node>) {
        for n in nodes.iter() {
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
        n: Vec<node::Node>,
    ) {
        let node: node::Node = n[0].clone();
        match node.ty {
            node::NodeType::BINOP(_, _, _, _) => self.analyze_binop(node),
            _ => (),
        }
        self.new_ident(env_name, ident_name, type_name)
    }
    fn checktype_binop(&mut self, ty: TokenType, lchild: Vec<node::Node>, rchild: Vec<node::Node>) {
        if ty == TokenType::TkPlus || ty == TokenType::TkStar {
            let lch: node::Node = self.walk(lchild[0].clone());
            let rch: node::Node = self.walk(rchild[0].clone());
            if !self.checklchild_valid_admul(&lch) {
                error::CompileError::TYPE(format!(
                    "operator '{}' doesn't implement for left-operand '{}'",
                    ty.string(),
                    lch.string(),
                ))
                .found();
            }
            if let node::NodeType::INT(val) = lch.ty.clone() {
                if !self.check_number(&rch) {
                    error::CompileError::TYPE(format!(
                        "operator '{}' doesn't implement for '{}' and '{}'",
                        ty.string(),
                        lch.string(),
                        rch.string(),
                    ))
                    .found();
                }
            } else if let node::NodeType::STRING(val) = lch.ty.clone() {
                if !self.check_string(&rch) {
                    error::CompileError::TYPE(format!(
                        "operator '{}' doesn't implement for '{}' and '{}'",
                        ty.string(),
                        lch.string(),
                        rch.string(),
                    ))
                    .found();
                }
            }
        } else if ty == TokenType::TkMinus || ty == TokenType::TkSlash {
            let lch: node::Node = self.walk(lchild[0].clone());
            let rch: node::Node = self.walk(rchild[0].clone());
            if !self.checklchild_valid_subdiv(&lch) {
                error::CompileError::TYPE(format!(
                    "operator '{}' doesn't implement for left-operand '{}'",
                    ty.string(),
                    lch.string(),
                ))
                .found();
            }
            if let node::NodeType::INT(val) = lch.ty.clone() {
                if !self.check_number(&rch) {
                    error::CompileError::TYPE(format!(
                        "operator '{}' doesn't implement for '{}' and '{}'",
                        ty.string(),
                        lch.string(),
                        rch.string(),
                    ))
                    .found();
                }
            } else if let node::NodeType::STRING(val) = lch.ty.clone() {
                if !self.check_string(&rch) {
                    error::CompileError::TYPE(format!(
                        "operator '{}' doesn't implement for '{}' and '{}'",
                        ty.string(),
                        lch.string(),
                        rch.string(),
                    ))
                    .found();
                }
            }
        }
    }
    fn walk(&mut self, n: node::Node) -> node::Node {
        if let node::NodeType::INT(val) = n.ty.clone() {
            return n;
        }
        if let node::NodeType::BINOP(ty, lchild, rchild, _) = n.ty.clone() {
            self.checktype_binop(ty, lchild, rchild);
        }
        n
    }
    fn analyze_binop(&mut self, node: node::Node) {
        self.walk(node);
    }
    fn checklchild_valid_admul(&mut self, lchild: &node::Node) -> bool {
        match &lchild.ty {
            node::NodeType::INT(t) | node::NodeType::UINT(t) | node::NodeType::STRING(t) => true,
            node::NodeType::ID(s) => true,
            _ => false,
        }
    }
    fn checklchild_valid_subdiv(&mut self, lchild: &node::Node) -> bool {
        match &lchild.ty {
            node::NodeType::INT(t) | node::NodeType::UINT(t) => true,
            node::NodeType::ID(s) => true,
            _ => false,
        }
    }
    fn check_number(&mut self, n: &node::Node) -> bool {
        match &n.ty {
            node::NodeType::INT(t) | node::NodeType::UINT(t) => true,
            _ => false,
        }
    }
    fn check_string(&mut self, n: &node::Node) -> bool {
        match &n.ty {
            node::NodeType::STRING(t) => true,
            _ => false,
        }
    }
    fn check_ident(&mut self, n: &node::Node) -> bool {
        match &n.ty {
            node::NodeType::ID(t) => true,
            _ => false,
        }
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
