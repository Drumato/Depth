use super::super::super::super::ce::types::Error;
use super::super::frontmanager::frontmanager::{Env, Symbol};
use super::super::sema::semantics::Type;
use super::super::token::token::Token;
use super::node::{Func, Node};
use std::collections::BTreeMap;
struct Parser {
    tokens: Vec<Token>,
    funcs: Vec<Func>,
    cur_env: Env,
    cur: usize,
    next: usize,
    lit: usize,
}
pub fn parsing(tokens: Vec<Token>) -> Vec<Func> {
    let mut parser: Parser = Parser::new(tokens);
    parser.toplevel();
    parser.funcs
}
impl Parser {
    fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens: tokens,
            funcs: Vec::with_capacity(100),
            cur_env: Env::new(),
            cur: 0,
            next: 1,
            lit: 0,
        }
    }
    fn toplevel(&mut self) {
        loop {
            let t: &Token = self.cur_token();
            match t {
                &Token::FUNC => {
                    self.func();
                }
                _ => break,
            }
        }
    }
    fn func(&mut self) {
        self.cur_env = Env::new();
        self.next_token();
        let func_name: String = self.consume_ident();
        self.expect(&Token::LPAREN);
        let mut func_args: Vec<Node> = Vec::new();
        loop {
            if self.consume(&Token::RPAREN) {
                break;
            }
            func_args.push(self.define_arg());
            if !self.consume(&Token::COMMA) {
                self.expect(&Token::RPAREN);
                break;
            }
        }
        let func_stmts: Vec<Node> = self.compound_stmt();
        self.funcs.push(Func {
            name: func_name,
            args: func_args,
            stmts: func_stmts,
            env: self.cur_env.clone(),
        });
    }
    fn stmt(&mut self) -> Node {
        let t: &Token = self.cur_token();
        match t {
            &Token::RETURN => self.parse_return(),
            &Token::LET => self.parse_let(),
            &Token::IDENT(_) => self.parse_assign(),
            _ => {
                Error::PARSE.found(&format!("statement can't start with '{}'", t.string(),));
                Node::INVALID
            }
        }
    }
    fn define_arg(&mut self) -> Node {
        let mutable: bool = self.consume(&Token::MUT);
        let arg_name: String = self.consume_ident();
        self.consume(&Token::COLON);
        let type_name: Token = self.consume_typename();
        self.cur_env
            .sym_table
            .insert(arg_name.clone(), Symbol::new(0, Err(type_name), mutable));
        Node::DEFARG(arg_name)
    }
    fn parse_let(&mut self) -> Node {
        self.expect(&Token::LET);
        let mutable_flg: bool = self.consume(&Token::MUT);
        let ident_name: String = self.consume_ident();
        self.expect(&Token::COLON);
        let type_name: Token = self.consume_typename();
        self.expect(&Token::ASSIGN);
        let expr: Node = self.expr();
        if let Some(_) = self.cur_env.sym_table.insert(
            ident_name.clone(),
            Symbol::new(0, Err(type_name), mutable_flg),
        ) {
            Error::TYPE.found(&format!("already defined identifier '{}'", &ident_name));
        }
        Node::LET(ident_name, Box::new(expr))
    }
    fn parse_return(&mut self) -> Node {
        self.expect(&Token::RETURN);
        let expr: Node = self.expr();
        Node::RETURN(Box::new(expr))
    }
    fn parse_assign(&mut self) -> Node {
        let ident_name: String = self.consume_ident();
        self.expect(&Token::ASSIGN);
        let expr: Node = self.expr();
        Node::ASSIGN(ident_name, Box::new(expr))
    }
    fn expr(&mut self) -> Node {
        self.equal()
    }
    fn equal(&mut self) -> Node {
        eprintln!("not implemented equal()");
        self.relation()
    }
    fn relation(&mut self) -> Node {
        eprintln!("not implemented relation()");
        self.shift()
    }
    fn shift(&mut self) -> Node {
        eprintln!("not implemented shift()");
        self.adsub()
    }
    fn adsub(&mut self) -> Node {
        let mut lhs: Node = self.muldiv();
        self.check_invalid(&lhs);
        loop {
            if !self.check_vec(vec![Token::PLUS, Token::MINUS]) {
                break;
            }
            let op: Token = self.get_token();
            self.next_token();
            if let Token::PLUS = op {
                lhs = Node::ADD(Box::new(lhs), Box::new(self.muldiv()), None);
            } else if let Token::MINUS = op {
                lhs = Node::SUB(Box::new(lhs), Box::new(self.muldiv()), None);
            }
        }
        lhs
    }
    fn muldiv(&mut self) -> Node {
        let mut lhs: Node = self.unary();
        self.check_invalid(&lhs);
        loop {
            if !self.check_vec(vec![Token::STAR, Token::SLASH]) {
                break;
            }
            let op: Token = self.get_token();
            self.next_token();
            if let Token::STAR = op {
                lhs = Node::MUL(Box::new(lhs), Box::new(self.unary()), None);
            } else if let Token::SLASH = op {
                lhs = Node::DIV(Box::new(lhs), Box::new(self.unary()), None);
            }
        }
        lhs
    }
    fn unary(&mut self) -> Node {
        let t: Token = self.get_token();
        match t {
            Token::AMPERSAND => {
                self.next_token();
                Node::ADDRESS(Box::new(self.unary()), None)
            }
            Token::STAR => {
                self.next_token();
                Node::DEREFERENCE(Box::new(self.unary()), None)
            }
            _ => {
                let n: Node = self.term();
                if self.check_vec(vec![Token::LBRACKET]) {
                    return self.postfix(n);
                }
                n
            }
        }
    }
    fn postfix(&mut self, n: Node) -> Node {
        let t: Token = self.get_token();
        match t {
            Token::LBRACKET => {
                self.next_token();
                let ind_n: Node = self.expr();
                self.expect(&Token::RBRACKET);
                self.postfix(Node::INDEX(Box::new(n), Box::new(ind_n)))
            }
            _ => n,
        }
    }
    fn term(&mut self) -> Node {
        let t: Token = self.get_token();
        match t {
            Token::LBRACKET => {
                self.next_token();
                let mut elems: Vec<Node> = Vec::new();
                loop {
                    if let &Token::RBRACKET = self.cur_token() {
                        break;
                    }
                    elems.push(self.expr());
                    if !self.consume(&Token::COMMA) {
                        self.consume(&Token::RBRACKET);
                        break;
                    }
                }
                self.cur_env.sym_table.insert(
                    format!("Array{}", self.lit),
                    Symbol::new(0, Err(Token::EOF), false),
                );
                let num = self.lit;
                self.lit += 1;
                Node::ARRAYLIT(Box::new(elems), num)
            }
            Token::INTEGER(val) => {
                self.next_token();
                Node::INTEGER(val)
            }
            Token::IDENT(name) => {
                self.next_token();
                if !self.consume(&Token::LPAREN) {
                    return Node::IDENT(name);
                }
                let mut args: Vec<Node> = Vec::new();
                loop {
                    if self.consume(&Token::RPAREN) {
                        break;
                    }
                    args.push(self.expr());
                    if !self.consume(&Token::COMMA) {
                        self.expect(&Token::RPAREN);
                        break;
                    }
                }
                Node::CALL(name, Box::new(args))
            }
            _ => {
                Error::PARSE.found(&format!("term can't start with '{}'", t.string()));
                Node::INVALID
            }
        }
    }
    fn compound_stmt(&mut self) -> Vec<Node> {
        let mut stmts: Vec<Node> = Vec::new();
        self.expect(&Token::LBRACE);
        loop {
            if self.consume(&Token::RBRACE) {
                break;
            }
            let st: Node = self.stmt();
            stmts.push(st);
        }
        stmts
    }
    fn expect(&mut self, t: &Token) {
        let cur: &Token = self.cur_token();
        if t == cur {
            self.next_token();
            return;
        }
        Error::PARSE.found(&format!(
            "expected {} but got '{}'",
            t.string(),
            cur.string()
        ));
    }
    fn consume(&mut self, t: &Token) -> bool {
        let cur: &Token = self.cur_token();
        if t == cur {
            self.next_token();
            true
        } else {
            false
        }
    }
    fn consume_typename(&mut self) -> Token {
        let t: Token = self.get_token();
        match t {
            Token::I64 => {
                self.next_token();
                Token::I64
            }
            Token::IDENT(name) => {
                self.next_token();
                eprintln!("not implemented deftype");
                Token::EOF
            }
            Token::POINTER(_ptr_to) => {
                self.next_token();
                self.expect(&Token::LT);
                let inner: Token = self.consume_typename();
                self.expect(&Token::GT);
                Token::POINTER(Box::new(inner))
            }
            Token::ARRAY(_type_name, _ary_size) => {
                self.next_token();
                self.expect(&Token::LT);
                let elem_type: Token = self.consume_typename();
                self.expect(&Token::COMMA);
                let ary_size: Token = self.get_token();
                self.next_token();
                self.expect(&Token::GT);
                Token::ARRAY(Box::new(elem_type), Box::new(ary_size))
            }
            _ => {
                Error::PARSE.found(&format!("got {} it's not typename ", t.string()));
                Token::EOF
            }
        }
    }
    fn consume_ident(&mut self) -> String {
        let t: Token = self.get_token();
        if let Token::IDENT(name) = t {
            self.next_token();
            name.to_string()
        } else {
            Error::PARSE.found(&format!("expected identifier but got '{}'", t.string()));
            String::new()
        }
    }
    fn check_invalid(&mut self, n: &Node) {
        if let &Node::INVALID = n {
            Error::PARSE.found(&"got INVALID Node".to_string());
        }
    }
    fn check_vec(&self, tks: Vec<Token>) -> bool {
        for t in tks.iter() {
            if self.cur_token() == t {
                return true;
            }
        }
        false
    }
    fn get_token(&self) -> Token {
        if self.cur >= self.tokens.len() {
            return Token::EOF;
        }
        self.tokens[self.cur].clone()
    }
    fn cur_token(&self) -> &Token {
        if self.cur >= self.tokens.len() {
            return &Token::EOF;
        }
        &self.tokens[self.cur]
    }
    fn next_token(&mut self) {
        self.cur += 1;
        self.next += 1;
    }
}
