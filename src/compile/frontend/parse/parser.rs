use super::super::super::super::ce::types::Error;
use super::super::super::manager::manager::{Env, Symbol};
use super::super::super::manager::semantics::{IntType, Type};
use super::super::token::token::Token;
use super::node::{Func, Node};
use std::collections::HashMap;
struct Parser {
    tokens: Vec<Token>,
    funcs: Vec<Func>,
    cur_env: Env,
}
static mut CUR: usize = 0;
static mut NEXT: usize = 1;
pub fn parsing(tokens: Vec<Token>) -> Vec<Func> {
    let mut parser: Parser = Parser::new(tokens);
    parser.toplevel();
    parser.funcs
}
impl Parser {
    fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens: tokens,
            funcs: Vec::new(),
            cur_env: Env::new(),
        }
    }
    fn toplevel(&mut self) {
        let global: Env = Env::new();
        while let Token::FUNC = self.cur_token() {
            let mut func: Func = self.func();
            func.env.prev = Some(Box::new(global.clone()));
            self.funcs.push(func);
        }
    }
    fn func(&mut self) -> Func {
        let mut f: Func = Func {
            name: String::new(),
            stmts: Vec::new(),
            args: Vec::new(),
            env: Env::new(),
        };
        self.cur_env = f.env.clone();
        self.next_token();
        let ident_name: String = self.consume_ident();
        f.name = ident_name.to_string();
        self.consume(&Token::LPAREN);
        loop {
            if self.consume(&Token::RPAREN) {
                break;
            }
            f.args.push(self.defarg());
            if !self.consume(&Token::COMMA) {
                self.consume(&Token::RPAREN);
                break;
            }
        }
        self.consume(&Token::LBRACE);
        let mut t: Token = self.get_token();
        while let Some(_) = Token::start_stmt(&t) {
            let stmt: Node = self.stmt();
            f.stmts.push(stmt);
            self.consume(&Token::RBRACE);
            t = self.get_token();
        }
        f.env = self.cur_env.clone();
        f
    }
    fn stmt(&mut self) -> Node {
        if let Token::RETURN = self.cur_token() {
            return self.parse_return();
        }
        if let Token::IF = self.cur_token() {
            return self.parse_if();
        }
        if let Token::LBRACE = self.cur_token() {
            return self.parse_block();
        }
        if let Token::LET = self.cur_token() {
            return self.parse_let();
        }
        self.expr()
    }
    fn expr(&mut self) -> Node {
        self.equal()
    }
    fn parse_return(&mut self) -> Node {
        if self.consume(&Token::RETURN) {
            return Node::RETURN(Box::new(self.expr()));
        }
        Error::PARSE.found(&format!(
            "unexpected {} while parsing return-stmt",
            self.cur_token().string()
        ));
        Node::INVALID
    }
    fn parse_if(&mut self) -> Node {
        if self.consume(&Token::IF) {
            let cond: Node = self.expr();
            let stmt: Node = self.stmt();
            if !self.consume(&Token::ELSE) {
                return Node::IF(Box::new(cond), Box::new(stmt), None);
            }
            return Node::IF(Box::new(cond), Box::new(stmt), Some(Box::new(self.stmt())));
        }
        Error::PARSE.found(&format!(
            "unexpected {} while parsing if-stmt",
            self.cur_token().string()
        ));
        Node::INVALID
    }
    fn parse_block(&mut self) -> Node {
        self.compound_stmt()
    }
    fn parse_let(&mut self) -> Node {
        self.next_token();
        let ident_name: String = self.consume_ident();
        if !self.consume(&Token::COLON) {
            Error::PARSE.found(&format!(
                "expected colon before declaring type but got {}",
                self.cur_token().string()
            ));
        }
        let typename: Token = self.consume_typename();
        if !self.consume(&Token::ASSIGN) {
            Error::PARSE.found(&format!(
                "expected assign after declaring type but got {}",
                self.cur_token().string()
            ));
        }
        let mut expr: Node = self.expr();
        if let Node::ARRAYLIT(ref mut v) = expr {
            for elem in v.iter_mut() {
                *elem = (Some(ident_name.clone()), elem.1.clone());
            }
        }
        self.cur_env
            .table
            .insert(ident_name.clone(), Symbol::new(0, typename.clone()));
        Node::LET(ident_name, typename, Box::new(expr))
    }
    fn compound_stmt(&mut self) -> Node {
        self.next_token();
        let mut stmts: Vec<Box<Node>> = Vec::new();
        let mut t: Token = self.get_token();
        while let Some(_) = Token::start_stmt(&t) {
            let stmt: Node = self.stmt();
            stmts.push(Box::new(stmt));
            if self.consume(&Token::RBRACE) {
                return Node::BLOCK(stmts);
            }
            t = self.get_token();
        }
        Node::BLOCK(stmts)
    }
    fn muldiv(&mut self) -> Node {
        let mut lhs: Node = self.unary();
        self.check_invalid(&lhs);
        loop {
            match self.cur_token() {
                Token::STAR | Token::SLASH | Token::PERCENT => {
                    let op: Token = self.get_token();
                    self.next_token();
                    lhs = Node::BINOP(op, Box::new(lhs), Box::new(self.unary()), None);
                }
                _ => {
                    break;
                }
            }
        }
        lhs
    }
    fn adsub(&mut self) -> Node {
        let mut lhs: Node = self.muldiv();
        self.check_invalid(&lhs);
        loop {
            match self.cur_token() {
                Token::PLUS | Token::MINUS => {
                    let op: Token = self.get_token();
                    self.next_token();
                    lhs = Node::BINOP(op, Box::new(lhs), Box::new(self.muldiv()), None);
                }
                _ => {
                    break;
                }
            }
        }
        lhs
    }
    fn shift(&mut self) -> Node {
        let mut lhs: Node = self.adsub();
        self.check_invalid(&lhs);
        loop {
            match self.cur_token() {
                Token::LSHIFT | Token::RSHIFT => {
                    let op: Token = self.get_token();
                    self.next_token();
                    lhs = Node::BINOP(op, Box::new(lhs), Box::new(self.adsub()), None);
                }
                _ => {
                    break;
                }
            }
        }
        lhs
    }
    fn relation(&mut self) -> Node {
        let mut lhs: Node = self.shift();
        self.check_invalid(&lhs);
        loop {
            match self.cur_token() {
                Token::LT | Token::GT | Token::LTEQ | Token::GTEQ => {
                    let op: Token = self.get_token();
                    self.next_token();
                    lhs = Node::BINOP(op, Box::new(lhs), Box::new(self.shift()), None);
                }
                _ => {
                    break;
                }
            }
        }
        lhs
    }
    fn equal(&mut self) -> Node {
        let mut lhs: Node = self.relation();
        self.check_invalid(&lhs);
        loop {
            match self.cur_token() {
                Token::EQ | Token::NTEQ => {
                    let op: Token = self.get_token();
                    self.next_token();
                    lhs = Node::BINOP(op, Box::new(lhs), Box::new(self.relation()), None);
                }
                _ => {
                    break;
                }
            }
        }
        lhs
    }
    fn unary(&mut self) -> Node {
        let op: Token = self.get_token();
        match op {
            Token::STAR | Token::AMPERSAND | Token::MINUS => {
                self.next_token();
                Node::UNARY(op, Box::new(self.term()), None)
            }
            _ => self.term(),
        }
    }
    fn defarg(&mut self) -> Node {
        let arg_name: String = self.consume_ident();
        self.consume(&Token::COLON);
        let ty: Token = self.consume_typename();
        self.cur_env
            .table
            .insert(arg_name.clone(), Symbol::new(0, ty.clone()));
        Node::DEFARG(arg_name, ty)
    }
    fn term(&mut self) -> Node {
        let t: &Token = self.cur_token();
        match t {
            Token::LPAREN => {
                self.next_token();
                let expr: Node = self.expr();
                self.next_token();
                expr
            }
            Token::INTEGER(int) => {
                self.next_token();
                Node::NUMBER(Type::INTEGER(IntType {
                    val: Some(*int),
                    type_size: 8,
                }))
            }
            Token::IDENT(_) => self.parse_ident(),
            Token::CHARLIT(char_val) => {
                self.next_token();
                Node::CHARLIT(*char_val)
            }
            Token::LBRACKET => {
                self.next_token();
                let mut elems: Vec<(Option<String>, Node)> = Vec::new();
                loop {
                    if let &Token::RBRACKET = self.cur_token() {
                        break;
                    }
                    elems.push((None, self.expr()));
                    if !self.consume(&Token::COMMA) {
                        self.consume(&Token::RBRACKET);
                        break;
                    }
                }
                Node::ARRAYLIT(elems)
            }
            _ => {
                Error::PARSE.found(&format!("unexpected {} while parsing term", t.string(),));
                Node::INVALID
            }
        }
    }
    fn parse_ident(&mut self) -> Node {
        let ident_name: String = self.consume_ident();
        let tok: &Token = self.cur_token();
        match tok {
            &Token::LBRACKET => {
                self.next_token();
                let expr: Node = self.expr();
                self.consume(&Token::RBRACKET);
                Node::INDEX(ident_name.clone(), Box::new(expr))
            }
            &Token::LPAREN => {
                self.next_token();
                let mut args: Vec<Box<Node>> = Vec::new();
                loop {
                    if self.consume(&Token::RPAREN) {
                        break;
                    }
                    args.push(Box::new(self.expr()));
                    if !self.consume(&Token::COMMA) {
                        self.consume(&Token::RPAREN);
                        break;
                    }
                }
                Node::CALL(ident_name.clone(), args)
            }
            _ => Node::IDENT(ident_name.to_string()),
        }
    }
    fn check_invalid(&mut self, n: &Node) {
        if let &Node::INVALID = n {
            Error::PARSE.found(&format!("got INVALID Node",));
            std::process::exit(1);
        }
    }
    fn consume(&self, t: &Token) -> bool {
        if self.cur_token() == t {
            self.next_token();
            return true;
        }
        false
    }
    fn consume_ident(&self) -> String {
        if let Token::IDENT(name) = self.cur_token() {
            self.next_token();
            return name.to_string();
        }
        Error::PARSE.found(&format!(
            "expected identifier but got {}",
            self.cur_token().string()
        ));
        String::new()
    }
    fn consume_typename(&mut self) -> Token {
        let t: Token = self.get_token();
        match t {
            Token::I8 | Token::I16 | Token::I32 | Token::I64 | Token::CHAR => {
                self.next_token();
                t
            }
            Token::POINTER(_ptr_to) => {
                self.next_token();
                self.consume(&Token::LT);
                let inner: Token = self.consume_typename();
                self.consume(&Token::GT);
                Token::POINTER(Box::new(inner))
            }
            Token::ARRAY(_, _) => {
                self.next_token();
                self.consume(&Token::LT);
                let elem_type: Token = self.consume_typename();
                self.consume(&Token::COMMA);
                let ary_size: Token = self.get_token();
                self.next_token();
                self.consume(&Token::GT);
                Token::ARRAY(Box::new(elem_type), Box::new(ary_size))
            }
            _ => {
                Error::PARSE.found(&format!("expected typename but got {}", t.string()));
                Token::EOF
            }
        }
    }
    fn peek_token(&self) -> &Token {
        unsafe {
            if CUR == self.tokens.len() {
                return &Token::EOF;
            }
            &self.tokens[NEXT]
        }
    }
    fn cur_token(&self) -> &Token {
        unsafe {
            if CUR == self.tokens.len() {
                return &Token::EOF;
            }
            &self.tokens[CUR]
        }
    }
    fn get_token(&mut self) -> Token {
        unsafe {
            if CUR == self.tokens.len() {
                return Token::EOF;
            }
            self.tokens[CUR].clone()
        }
    }
    fn next_token(&self) {
        unsafe {
            CUR += 1;
            NEXT += 1;
        }
    }
}
