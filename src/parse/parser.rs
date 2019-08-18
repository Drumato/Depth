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
            let expr: Node = self.adsub();
            self.nodes.push(expr);
        }
    }
    fn adsub(&mut self) -> Node {
        let mut lhs: Node = self.number();
        self.check_invalid(&lhs);
        loop {
            match self.cur_token() {
                Token::PLUS | Token::MINUS => {
                    let op: Token = self.get_token();
                    self.next_token();
                    lhs = Node::BINOP(op, Box::new(lhs), Box::new(self.number()));
                }
                _ => {
                    break;
                }
            }
        }
        lhs
    }
    fn number(&mut self) -> Node {
        let t: &Token = self.cur_token();
        if let Token::INTEGER(int) = t {
            self.next_token();
            return Node::INTEGER(*int);
        }
        Node::INVALID
    }
    fn check_invalid(&mut self, n: &Node) {
        if let &Node::INVALID = n {
            eprintln!("got INVALID Node");
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
        unsafe { self.tokens[CUR].clone() }
    }
    fn next_token(&self) {
        unsafe {
            CUR += 1;
            NEXT += 1;
        }
    }
}
