use crate::ce::types::Error;
use crate::compile::frontend;
use frontend::frontmanager::frontmanager::{Env, Symbol};
use frontend::parse::node::{Func, Node};
use frontend::sema::semantics::Type;
use frontend::token::token::Token;

use std::collections::BTreeMap;
struct Parser {
    tokens: Vec<Token>,
    funcs: Vec<Func>,
    cur_env: Env,
    cur: usize,
    next: usize,
    lit: usize,
    comp_table: BTreeMap<String, i128>,
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
            comp_table: BTreeMap::new(),
        }
    }
    fn toplevel(&mut self) {
        let mut global = Env::new();
        loop {
            let t: &Token = &self.cur_token().clone();
            match t {
                &Token::COMPINT => {
                    self.parse_compint();
                }
                &Token::TYPE => {
                    self.parse_alias(&mut global);
                }
                &Token::FUNC => {
                    self.parse_func(global.clone());
                }
                Token::INFORMATION(contents) => {
                    self.next_token();
                    self.parse_func(global.clone());
                    let insert_number = self.funcs.len();
                    self.funcs[insert_number - 1].document = Some(contents.to_string());
                }
                &Token::STRUCT => {
                    self.parse_struct();
                }
                _ => break,
            }
        }
    }
    fn stmt(&mut self) -> Node {
        let t: &Token = self.cur_token();
        match t {
            &Token::RETURN => self.parse_return(),
            &Token::LET => self.parse_let(),
            &Token::IDENT(_) => self.parse_assign(),
            &Token::LBRACE => self.parse_block(),
            &Token::CONDLOOP => self.parse_condloop(),
            &Token::IF => self.parse_if(),
            &Token::COLON => self.parse_label(),
            &Token::GOTO => self.parse_goto(),
            _ => {
                Error::PARSE.found(&format!("statement can't start with '{}'", t.string(),));
                Node::INVALID
            }
        }
    }
    fn parse_compint(&mut self) {
        self.expect(&Token::COMPINT);
        let comp_name: String = self.consume_ident();
        self.expect(&Token::ASSIGN);
        let t = self.get_token();
        if let Token::INTEGER(val) = t {
            self.comp_table.insert(comp_name, val);
        } else {
            Error::PARSE.found(&"compint statement's expression must be integer".to_string());
        }
        self.next_token();
    }
    fn parse_func(&mut self, global: Env) {
        self.cur_env = Env::new();
        self.cur_env.prev = Some(Box::new(global));
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
        if !self.consume(&Token::DOUBLECOLON) {
            Error::PARSE.found(
                &"the function's return type of Depth must be declare explicit.".to_string(),
            );
        }
        let return_type_t = self.consume_typename();
        let return_type: Type = if let Some(type_name) = return_type_t.name() {
            if let Some(global_t) = &self.cur_env.prev {
                if let Some(alias_t) = global_t.type_table.get(&type_name) {
                    alias_t.clone()
                } else {
                    Error::TYPE.found(&format!("not found such an alias -> {}", type_name));
                    Type::UNKNOWN
                }
            } else {
                Type::UNKNOWN
            }
        } else {
            Type::from_token(return_type_t)
        };
        let func_stmts: Vec<Node> = self.compound_stmt();
        self.funcs.push(Func {
            name: func_name,
            args: func_args,
            stmts: func_stmts,
            return_type: return_type,
            document: None,
            env: self.cur_env.clone(),
        });
    }
    fn parse_alias(&mut self, global: &mut Env) {
        self.expect(&Token::TYPE);
        let alias_name: String = self.consume_ident();
        self.expect(&Token::ASSIGN);
        let type_name: Token = self.consume_typename();
        global.type_table.insert(
            alias_name,
            Type::ALIAS(Box::new(Type::from_token(type_name))),
        );
    }
    fn parse_struct(&mut self) {
        self.expect(&Token::STRUCT);
        let type_name: String = self.consume_ident();
        self.expect(&Token::LBRACE);
        let mut members: BTreeMap<String, Symbol> = BTreeMap::new();
        loop {
            if self.consume(&Token::RBRACE) {
                break;
            }
            let member_name: String = self.consume_ident();
            self.expect(&Token::COLON);
            let member_type: Token = self.consume_typename();
            members.insert(member_name, Symbol::new(0, Err(member_type), false));
        }
        let mut total_size: usize = 0;
        for (_name, s) in members.iter() {
            total_size += s.size();
        }
        if let Some(ref mut global) = self.cur_env.prev {
            global
                .type_table
                .insert(type_name, Type::STRUCT(members, total_size));
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
    fn parse_label(&mut self) -> Node {
        self.next_token();
        let label: String = self.consume_ident();
        Node::LABEL(label)
    }
    fn parse_goto(&mut self) -> Node {
        self.next_token();
        if !self.consume(&Token::COLON) {
            Error::PARSE.found(&"labelname must be started colon".to_string());
        }
        let label: String = self.consume_ident();
        Node::GOTO(label)
    }
    fn parse_condloop(&mut self) -> Node {
        self.expect(&Token::CONDLOOP);
        self.expect(&Token::LPAREN);
        let cond: Node = self.expr();
        self.expect(&Token::RPAREN);
        let stmt: Node = self.stmt();
        Node::CONDLOOP(Box::new(cond), Box::new(stmt))
    }
    fn parse_block(&mut self) -> Node {
        let stmts: Vec<Node> = self.compound_stmt();
        Node::BLOCK(Box::new(stmts))
    }
    fn parse_if(&mut self) -> Node {
        self.expect(&Token::IF);
        self.expect(&Token::LPAREN);
        let cond: Node = self.expr();
        self.expect(&Token::RPAREN);
        let stmt: Node = self.stmt();
        if !self.consume(&Token::ELSE) {
            return Node::IF(Box::new(cond), Box::new(stmt), None);
        }
        Node::IF(Box::new(cond), Box::new(stmt), Some(Box::new(self.stmt())))
    }
    fn parse_let(&mut self) -> Node {
        self.expect(&Token::LET);
        let mutable_flg: bool = self.consume(&Token::MUT);
        let ident_name: String = self.consume_ident();
        self.expect(&Token::COLON);
        let type_name: Token = self.consume_typename();
        self.expect(&Token::ASSIGN);
        let mut expr: Node = self.expr();
        if let Node::STRUCTLIT(ref mut name, ref mut _members) = expr {
            *name = ident_name.clone();
        } else if let Node::ARRAYLIT(ref mut _belems, ref mut name) = expr {
            self.cur_env.sym_table.remove(name);
            *name = ident_name.clone();
        }
        if let Some(_) = self.cur_env.sym_table.insert(
            ident_name.clone(),
            Symbol::new(0, Err(type_name), mutable_flg),
        ) {}
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
        let mut lhs: Node = self.relation();
        self.check_invalid(&lhs);
        loop {
            if !self.check_vec(vec![Token::EQ, Token::NTEQ]) {
                break;
            }
            let op: Token = self.get_token();
            self.next_token();
            if let &Token::EQ = &op {
                lhs = Node::EQ(Box::new(lhs), Box::new(self.relation()));
            } else if let &Token::NTEQ = &op {
                lhs = Node::NTEQ(Box::new(lhs), Box::new(self.relation()));
            }
        }
        lhs
    }
    fn relation(&mut self) -> Node {
        let mut lhs: Node = self.shift();
        self.check_invalid(&lhs);
        loop {
            if !self.check_vec(vec![Token::LT, Token::GT, Token::LTEQ, Token::GTEQ]) {
                break;
            }
            let op: Token = self.get_token();
            self.next_token();
            if let &Token::LT = &op {
                lhs = Node::LT(Box::new(lhs), Box::new(self.relation()));
            } else if let &Token::GT = &op {
                lhs = Node::GT(Box::new(lhs), Box::new(self.relation()));
            } else if let &Token::LTEQ = &op {
                lhs = Node::LTEQ(Box::new(lhs), Box::new(self.relation()));
            } else if let &Token::GTEQ = &op {
                lhs = Node::GTEQ(Box::new(lhs), Box::new(self.relation()));
            }
        }
        lhs
    }
    fn shift(&mut self) -> Node {
        let mut lhs: Node = self.adsub();
        self.check_invalid(&lhs);
        loop {
            if self.check(&Token::LSHIFT) {
                self.next_token();
                lhs = Node::LSHIFT(Box::new(lhs), Box::new(self.adsub()));
            } else if self.check(&Token::GT) {
                if self.peek(&Token::GT) {
                    self.next_token();
                    self.next_token();
                    lhs = Node::RSHIFT(Box::new(lhs), Box::new(self.adsub()));
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        lhs
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
                lhs = Node::ADD(Box::new(lhs), Box::new(self.muldiv()));
            } else if let Token::MINUS = op {
                lhs = Node::SUB(Box::new(lhs), Box::new(self.muldiv()));
            }
        }
        lhs
    }
    fn muldiv(&mut self) -> Node {
        let mut lhs: Node = self.unary();
        self.check_invalid(&lhs);
        loop {
            if !self.check_vec(vec![Token::STAR, Token::SLASH, Token::PERCENT]) {
                break;
            }
            let op: Token = self.get_token();
            self.next_token();
            if let Token::STAR = op {
                lhs = Node::MUL(Box::new(lhs), Box::new(self.unary()));
            } else if let Token::SLASH = op {
                lhs = Node::DIV(Box::new(lhs), Box::new(self.unary()));
            } else if let Token::PERCENT = op {
                lhs = Node::MOD(Box::new(lhs), Box::new(self.unary()));
            }
        }
        lhs
    }
    fn unary(&mut self) -> Node {
        let t: Token = self.get_token();
        match t {
            Token::AMPERSAND => {
                self.next_token();
                Node::ADDRESS(Box::new(self.unary()))
            }
            Token::STAR => {
                self.next_token();
                Node::DEREFERENCE(Box::new(self.unary()))
            }
            Token::MINUS => {
                self.next_token();
                Node::MINUS(Box::new(self.unary()))
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
                self.expect(&Token::LBRACKET);
                let ind_n: Node = self.expr();
                self.expect(&Token::RBRACKET);
                self.postfix(Node::INDEX(Box::new(n), Box::new(ind_n)))
            }
            _ => n,
        }
    }
    fn term(&mut self) -> Node {
        if let Some(val) = self.consume_compint() {
            self.next_token();
            return Node::INTEGER(val);
        }
        let t: Token = self.get_token();

        match t {
            Token::LPAREN => {
                self.expect(&Token::LPAREN);
                let n: Node = self.expr();
                self.expect(&Token::RPAREN);
                n
            }
            Token::LBRACKET => {
                self.expect(&Token::LBRACKET);
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
                Node::ARRAYLIT(Box::new(elems), format!("Array{}", num))
            }
            Token::INTEGER(val) => {
                self.next_token();
                Node::INTEGER(val)
            }
            Token::IDENT(name) => {
                self.next_token();
                let t: Token = self.get_token();
                match t {
                    Token::DOT => {
                        self.expect(&Token::DOT);
                        let member_name: String = self.consume_ident();
                        Node::MEMBER(Box::new(Node::IDENT(name)), member_name)
                    }
                    Token::LPAREN => {
                        self.expect(&Token::LPAREN);
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
                    Token::LBRACE => {
                        self.expect(&Token::LBRACE);
                        let mut members: BTreeMap<String, Node> = BTreeMap::new();
                        loop {
                            if self.consume(&Token::RBRACE) {
                                break;
                            }
                            let member_name: String = self.consume_ident();
                            self.expect(&Token::COLON);
                            let member_expr: Node = self.expr();
                            members.insert(member_name, member_expr);
                            if !self.consume(&Token::COMMA) {
                                self.expect(&Token::RBRACE);
                                break;
                            }
                        }
                        Node::STRUCTLIT(name, Box::new(members))
                    }
                    _ => Node::IDENT(name),
                }
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
                Token::IDENT(name.to_string())
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
    fn consume_compint(&mut self) -> Option<i128> {
        let t: Token = self.get_token();
        if let Token::IDENT(name) = t {
            if let None = self.comp_table.get(&name) {
                return None;
            }
            return Some(*self.comp_table.get(&name).unwrap());
        } else {
            return None;
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
    fn check(&self, t: &Token) -> bool {
        if self.cur_token() == t {
            return true;
        }
        false
    }
    fn peek(&self, t: &Token) -> bool {
        if self.peek_token() == t {
            return true;
        }
        false
    }
    fn peek_token(&self) -> &Token {
        if self.next >= self.tokens.len() {
            return &Token::EOF;
        }
        &self.tokens[self.next]
    }
}
