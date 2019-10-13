use super::super::super::super::ce::types::Error;
use super::super::frontmanager::frontmanager::{DefType, Env, Symbol};
use super::super::sema::semantics::{IntType, Type};
use super::super::token::token::Token;
use super::node::{Func, Node};
use std::collections::BTreeMap;
struct Parser {
    tokens: Vec<Token>,
    funcs: Vec<Func>,
    cur_env: Env,
}
static mut CUR: usize = 0;
static mut NEXT: usize = 1;
static mut LIT: usize = 0;
pub fn parsing(tokens: Vec<Token>) -> Vec<Func> {
    unsafe {
        CUR = 0;
        NEXT = 1;
        LIT = 0;
    }
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
        }
    }
    fn toplevel(&mut self) {
        let mut global: Env = Env::new();
        loop {
            let t: &Token = self.cur_token();
            match t {
                Token::FUNC => {
                    let mut func: Func = self.func();
                    func.env.prev = Some(Box::new(global.clone()));
                    self.funcs.push(func);
                }
                Token::TYPE => {
                    self.next_token();
                    let type_name: String = self.consume_ident();
                    if !self.consume(&Token::ASSIGN) {
                        Error::PARSE.found(&format!(
                            "expected assign before declaring type but got {}",
                            self.cur_token().string()
                        ));
                    }
                    let token_ty: Token = self.consume_typename();
                    global
                        .type_table
                        .insert(type_name, DefType::new(Some(Type::from_type(token_ty))));
                }
                Token::STRUCT => {
                    self.next_token();
                    let type_name: String = self.consume_ident();
                    let mut member_map: BTreeMap<String, Symbol> = BTreeMap::new();
                    if !self.consume(&Token::LBRACE) {
                        Error::PARSE.found(&format!(
                            "expected LBRACE before declaring struct-type but got {}",
                            self.cur_token().string()
                        ));
                    }
                    loop {
                        if self.consume(&Token::RBRACE) {
                            break;
                        }
                        let member_name: String = self.consume_ident();
                        if !self.consume(&Token::COLON) {
                            Error::PARSE.found(&format!(
                                "expected COLON before declaring member-type but got {}",
                                self.cur_token().string()
                            ));
                        }
                        let member_ty: Token = self.consume_typename();
                        member_map.insert(member_name, Symbol::new(0, member_ty, false));
                    }
                    global
                        .type_table
                        .insert(type_name, DefType::new_struct(member_map));
                }
                _ => break,
            }
        }
    }
    fn func(&mut self) -> Func {
        let mut f: Func = Func {
            name: String::new(),
            stmts: Vec::with_capacity(100),
            args: Vec::with_capacity(6),
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
        let t: &Token = self.cur_token();
        match t {
            &Token::RETURN => self.parse_return(),
            &Token::IF => self.parse_if(),
            &Token::LBRACE => self.parse_block(),
            &Token::LET => self.parse_let(),
            &Token::IDENT(_) => self.parse_assign(),
            &Token::CONDLOOP => self.parse_condloop(),
            &Token::COLON => self.parse_label(),
            &Token::GOTO => self.parse_goto(),
            _ => self.expr(),
        }
    }
    fn expr(&mut self) -> Node {
        self.equal()
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
    fn parse_assign(&mut self) -> Node {
        let ident_name: String = self.consume_ident();
        self.consume(&Token::ASSIGN);
        let e: Node = self.expr();
        Node::ASSIGN(ident_name, Box::new(e))
    }
    fn parse_if(&mut self) -> Node {
        self.next_token();
        let cond: Node = self.expr();
        let stmt: Node = self.stmt();
        if !self.consume(&Token::ELSE) {
            return Node::IF(Box::new(cond), Box::new(stmt), None);
        }
        Node::IF(Box::new(cond), Box::new(stmt), Some(Box::new(self.stmt())))
    }
    fn parse_condloop(&mut self) -> Node {
        self.next_token();
        let cond: Node = self.expr();
        let stmt: Node = self.stmt();
        Node::CONDLOOP(Box::new(cond), Box::new(stmt))
    }
    fn parse_block(&mut self) -> Node {
        self.compound_stmt()
    }
    fn parse_let(&mut self) -> Node {
        self.next_token();
        let mut mutable_flg: bool = false;
        if self.consume(&Token::MUT) {
            mutable_flg = true;
        }
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
        let expr: Node = self.expr();

        if let Some(_symbol) = self.cur_env.sym_table.get(&ident_name) {
            Error::TYPE.found(&format!("already defined identifier '{}'", &ident_name));
        }
        self.cur_env.sym_table.insert(
            ident_name.clone(),
            Symbol::new(0, typename.clone(), mutable_flg),
        );
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
            if !self.check_vec(vec![Token::STAR, Token::SLASH, Token::PERCENT]) {
                break;
            }
            let op: Token = self.get_token();
            self.next_token();
            lhs = Node::BINOP(op, Box::new(lhs), Box::new(self.unary()), None);
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
            lhs = Node::BINOP(op, Box::new(lhs), Box::new(self.muldiv()), None);
        }
        lhs
    }
    fn shift(&mut self) -> Node {
        let mut lhs: Node = self.adsub();
        self.check_invalid(&lhs);
        loop {
            if self.check(&Token::LSHIFT) {
                let op: Token = self.get_token();
                self.next_token();
                lhs = Node::BINOP(op, Box::new(lhs), Box::new(self.adsub()), None);
            } else if self.check(&Token::GT) {
                if self.peek(&Token::GT) {
                    self.next_token();
                    self.next_token();
                    let op: Token = Token::RSHIFT;
                    lhs = Node::BINOP(op, Box::new(lhs), Box::new(self.adsub()), None);
                } else {
                    break;
                }
            } else {
                break;
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
            lhs = Node::BINOP(op, Box::new(lhs), Box::new(self.shift()), None);
        }
        lhs
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
            lhs = Node::BINOP(op, Box::new(lhs), Box::new(self.relation()), None);
        }
        lhs
    }
    fn unary(&mut self) -> Node {
        if self.check_vec(vec![Token::STAR, Token::AMPERSAND, Token::MINUS]) {
            let op: Token = self.get_token();
            self.next_token();
            return Node::UNARY(op, Box::new(self.unary()), None);
        }
        let mut n: Node = self.term();
        loop {
            let t: &Token = self.cur_token();
            if let &Token::LBRACKET = t {
                self.next_token();
                let expr: Node = self.expr();
                self.consume(&Token::RBRACKET);
                n = Node::INDEX(Box::new(n), Box::new(expr));
            } else {
                break;
            }
        }
        n
    }
    fn defarg(&mut self) -> Node {
        let mut mutable: bool = false;
        if self.consume(&Token::MUT) {
            mutable = true;
        }
        let arg_name: String = self.consume_ident();
        self.consume(&Token::COLON);
        let ty: Token = self.consume_typename();
        self.cur_env
            .sym_table
            .insert(arg_name.clone(), Symbol::new(0, ty.clone(), mutable));
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
                    format!("Array{}", unsafe { LIT }),
                    Symbol::new(0, Token::EOF, false),
                );
                let num = unsafe { LIT };
                unsafe {
                    LIT += 1;
                }
                Node::ARRAYLIT(elems, num)
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
    fn check_vec(&self, tks: Vec<Token>) -> bool {
        for t in tks.iter() {
            if self.cur_token() == t {
                return true;
            }
        }
        false
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
            Token::I64 | Token::CHAR => {
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
            Token::IDENT(_) => {
                self.next_token();
                t
            }
            _ => {
                Error::PARSE.found(&format!("expected typename but got {}", t.string()));
                Token::EOF
            }
        }
    }
    fn cur_token(&self) -> &Token {
        unsafe {
            if CUR >= self.tokens.len() {
                return &Token::EOF;
            }
            &self.tokens[CUR]
        }
    }
    fn peek_token(&self) -> &Token {
        unsafe {
            if NEXT >= self.tokens.len() {
                return &Token::EOF;
            }
            &self.tokens[NEXT]
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
