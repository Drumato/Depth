use super::super::token::token::Token;
use super::node::Node;
pub struct Parser {
    tokens: Vec<Token>,
    nodes: Vec<Node>,
}
static mut CUR: usize = 0;
static mut NEXT: usize = 0;
pub fn parsing(tokens: Vec<Token>) -> Vec<Node> {
    let mut parser: Parser = Parser::new(tokens);
    parser.toplevel();
    parser.nodes
}
impl Parser {
    fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens: tokens,
            nodes: Vec::new(),
        }
    }
    fn toplevel(&mut self) {
        while let Some(_) = Token::is_valid(self.cur_token()) {
            let expr: Node = self.expr();
            self.nodes.push(expr);
        }
    }
    fn expr(&mut self) -> Node {
        self.adsub()
    }
    fn muldiv(&mut self) -> Node {
        let mut lhs: Node = self.unary();
        self.check_invalid(&lhs);
        loop {
            match self.cur_token() {
                Token::STAR | Token::SLASH | Token::PERCENT => {
                    let op: Token = self.get_token();
                    self.next_token();
                    lhs = Node::BINOP(op, Box::new(lhs), Box::new(self.unary()));
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
                    lhs = Node::BINOP(op, Box::new(lhs), Box::new(self.muldiv()));
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
            return Node::UNARY(op, Box::new(self.term()));
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
            return Node::INTEGER(*int);
        }
        Node::INVALID
    }
    fn check_invalid(&mut self, n: &Node) {
        if let &Node::INVALID = n {
            eprintln!("got INVALID Node");
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
    fn cur_token(&self) -> &Token {
        unsafe {
            if CUR == self.tokens.len() {
                return &Token::EOF;
            }
            &self.tokens[CUR]
        }
    }
    fn get_token(&mut self) -> Token {
        unsafe { self.tokens[CUR].clone() }
    }
    fn next_token(&self) {
        unsafe {
            CUR += 1;
            NEXT += 1;
        }
    }
}
