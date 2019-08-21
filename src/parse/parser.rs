use super::super::ce::types::Error;
use super::super::manager::semantics::Type;
use super::super::token::token::Token;
use super::node::{Func, Node};
struct Parser {
    tokens: Vec<Token>,
    funcs: Vec<Func>,
}
static mut CUR: usize = 0;
static mut NEXT: usize = 0;
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
        }
    }
    fn toplevel(&mut self) {
        while let Token::FUNC = self.cur_token() {
            let func: Func = self.func();
            self.funcs.push(func);
        }
    }
    fn func(&mut self) -> Func {
        let mut f: Func = Func {
            name: String::new(),
            stmts: Vec::new(),
        };
        self.next_token();
        let ident_name: String = self.consume_ident();
        f.name = ident_name.to_string();
        self.expect(&Token::LPAREN);
        self.expect(&Token::RPAREN);
        self.expect(&Token::LBRACE);
        let mut t: Token = self.get_token();
        while let Some(_) = Token::start_stmt(&t) {
            let stmt: Node = self.stmt();
            f.stmts.push(stmt);
            self.consume(&Token::RBRACE);
            t = self.get_token();
        }
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
            Error::PARSE.found(&format!("expected colon before declaring type"));
        }
        let typename: Token = self.consume_typename();
        if !self.consume(&Token::ASSIGN) {
            Error::PARSE.found(&format!("expected assign after declaring type"));
        }
        Node::LET(ident_name, typename, Box::new(self.expr()))
    }
    fn compound_stmt(&mut self) -> Node {
        self.next_token();
        let mut stmts: Vec<Box<Node>> = Vec::new();
        let mut t: Token = self.get_token();
        while let Some(_) = Token::start_stmt(&t) {
            let stmt: Node = self.stmt();
            stmts.push(Box::new(stmt));
            self.consume(&Token::RBRACE);
            t = self.get_token();
            if self.consume(&Token::RBRACE) {
                return Node::BLOCK(stmts);
            }
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
        if self.consume(&Token::MINUS) {
            let op: Token = self.get_token();
            return Node::UNARY(op, Box::new(self.term()), None);
        }
        self.term()
    }
    fn term(&mut self) -> Node {
        let t: &Token = self.cur_token();
        if let Token::LPAREN = t {
            self.next_token();
            let expr: Node = self.expr();
            self.next_token();
            return expr;
        }
        if let Token::INTEGER(int) = t {
            self.next_token();
            return Node::NUMBER(Type::INTEGER(*int, 8, None));
        }
        if let Token::IDENT(ident_name) = t {
            self.next_token();
            return Node::IDENT(ident_name.to_string());
        }
        Error::PARSE.found(&format!("unexpected {} while parsing term", t.string(),));
        Node::INVALID
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
        if let Token::I8 = t {
            self.next_token();
            return t;
        }
        Error::PARSE.found(&format!("expected typename but got {}", t.string()));
        return Token::EOF;
    }
    fn expect(&self, t: &Token) -> bool {
        if self.peek_token() == t {
            self.next_token();
            return true;
        }
        Error::PARSE.found(&format!(
            "{} expected but got {}",
            t.string(),
            self.cur_token().string()
        ));
        false
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
