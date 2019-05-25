use super::super::lex::{lexing, token};
use super::node::{Node, NodeType};
use token::{Token, TokenType, TokenVal};

pub struct Parser {
    pub l: lexing::Lexer,
    pub cur: Token,
    pub next: Token,
}

impl Parser {
    pub fn new(mut lexer: lexing::Lexer) -> Parser {
        let cur: Token = lexer.next_token();
        let next: Token = lexer.next_token();
        Parser {
            l: lexer,
            cur: cur,
            next: next,
        }
    }
    pub fn next_token(&mut self) {
        self.cur = self.next.clone();
        self.next = self.l.next_token();
    }
    pub fn consume(&mut self, ty: TokenType) -> bool {
        if self.cur.ty == ty {
            self.next_token();
            return true;
        }
        println!(
            "Error! {} expected but got {}",
            ty.string(),
            self.cur.ty.string()
        );
        false
    }
    pub fn expect(&mut self, ty: TokenType) {
        if self.next.ty == ty {
            self.next_token();
            return;
        }
        println!(
            "Error! {} expected but got {}",
            ty.string(),
            self.next.ty.string()
        );
    }
    fn term(&mut self) -> Node {
        let t: Token = self.cur.clone();
        self.next_token();
        match t.ty {
            TokenType::TkIntlit | TokenType::TkUintlit => Node::new_num(t),
            TokenType::TkIdent => Node::new_ident(t.literal),
            _ => Node::new(NodeType::INVALID),
        }
    }
    fn equal(&mut self) -> Node {
        let mut lchild: Node = self.cmp();
        loop {
            let t: Token = self.cur.clone();
            if t.ty != TokenType::TkEq && t.ty != TokenType::TkNoteq {
                break;
            }
            self.next_token();
            lchild = Node::new_binop(t.ty, lchild, self.cmp());
        }
        lchild
    }
    fn cmp(&mut self) -> Node {
        let mut lchild: Node = self.shift();
        loop {
            let t: Token = self.cur.clone();
            if t.ty != TokenType::TkLt
                && t.ty != TokenType::TkGt
                && t.ty != TokenType::TkGteq
                && t.ty != TokenType::TkLteq
            {
                break;
            }
            self.next_token();
            lchild = Node::new_binop(t.ty, lchild, self.shift());
        }
        lchild
    }
    fn shift(&mut self) -> Node {
        let mut lchild: Node = self.adsub();
        loop {
            let t: Token = self.cur.clone();
            if t.ty != TokenType::TkLshift && t.ty != TokenType::TkRshift {
                break;
            }
            self.next_token();
            lchild = Node::new_binop(t.ty, lchild, self.adsub());
        }
        lchild
    }
    fn adsub(&mut self) -> Node {
        let mut lchild: Node = self.muldiv();
        loop {
            let t: Token = self.cur.clone();
            if t.ty != TokenType::TkPlus && t.ty != TokenType::TkMinus {
                break;
            }
            self.next_token();
            lchild = Node::new_binop(t.ty, lchild, self.muldiv());
        }
        lchild
    }
    fn muldiv(&mut self) -> Node {
        let mut lchild: Node = self.term();
        loop {
            let t: Token = self.cur.clone();
            if t.ty != TokenType::TkStar
                && t.ty != TokenType::TkSlash
                && t.ty != TokenType::TkPercent
            {
                break;
            }
            self.next_token();
            lchild = Node::new_binop(t.ty, lchild, self.term());
        }
        lchild
    }
    fn expr(&mut self) -> Node {
        if self.cur.ty != TokenType::TkIntlit && self.cur.ty != TokenType::TkUintlit {
            println!(
                "Error! Number-Literal expected but got {}",
                self.cur.ty.string()
            );
        }
        self.equal()
    }
    fn stmt(&mut self) -> Node {
        match self.cur.ty {
            TokenType::TkReturn => self.parse_return(),
            TokenType::TkLet => self.parse_let(),
            TokenType::TkLoop => self.parse_loop(),
            TokenType::TkFor => self.parse_for(),
            _ => Node::new(NodeType::INVALID),
        }
    }

    fn parse_return(&mut self) -> Node {
        let ret_keyword: TokenType = self.cur.ty.clone();
        self.next_token();
        Node::new_rets(ret_keyword, self.expr())
    }
    fn parse_let(&mut self) -> Node {
        let let_keyword: TokenType = self.cur.ty.clone();
        self.next_token();
        let ident_name: String = self.cur.literal.clone();
        self.expect(TokenType::TkColon);
        self.next_token();
        if !self.cur.ty.is_typename() {
            println!("expected typename but got {}", self.cur.literal);
        }
        let typename: TokenType = self.cur.ty.clone();
        self.expect(TokenType::TkAssign);
        self.next_token();
        let expr: Node = self.expr();
        Node::new_lets(let_keyword, ident_name, typename, expr)
    }
    fn parse_loop(&mut self) -> Node {
        let loop_keyword: TokenType = self.cur.ty.clone();
        self.expect(TokenType::TkLbrace);
        self.next_token();
        let mut statements: Vec<Node> = Vec::new();
        while self.cur.ty != TokenType::TkRbrace {
            let n: Node = self.stmt();
            statements.push(n);
        }
        self.consume(TokenType::TkRbrace);
        Node::new_loops(loop_keyword, statements)
    }
    fn parse_for(&mut self) -> Node {
        let for_keyword: TokenType = self.cur.ty.clone();
        self.next_token();
        let tmp_ident: String = self.cur.literal.clone();
        self.expect(TokenType::TkIn);
        self.next_token();
        let src_ident: String = self.cur.literal.clone();
        self.expect(TokenType::TkLbrace);
        self.next_token();
        let mut statements: Vec<Node> = Vec::new();
        while self.cur.ty != TokenType::TkRbrace {
            let n: Node = self.stmt();
            statements.push(n);
        }
        self.consume(TokenType::TkRbrace);
        Node::new_fors(for_keyword, tmp_ident, src_ident, statements)
    }
    fn func(&mut self) -> Node {
        if !self.consume(TokenType::TkF) {
            println!("invalid f {}", self.cur.literal);
            return Node::new(NodeType::INVALID);
        }
        let func_name: String = self.cur.literal.clone();
        self.expect(TokenType::TkLparen);
        let mut arguments: Vec<Node> = Vec::new();
        self.expect(TokenType::TkRparen);

        let mut ret: TokenType = if self.next.ty == TokenType::TkArrow {
            self.next_token();
            self.next_token();
            self.cur.ty.clone()
        } else {
            TokenType::TkIllegal
        };

        self.expect(TokenType::TkLbrace);
        self.next_token();
        let mut statements: Vec<Node> = Vec::new();
        while self.cur.ty != TokenType::TkRbrace {
            let n: Node = self.stmt();
            statements.push(n);
        }
        self.consume(TokenType::TkRbrace);
        self.consume(TokenType::TkEof);
        Node::new_func(func_name, arguments, ret, statements)
    }
}

pub fn parse(lexer: lexing::Lexer) -> Vec<Node> {
    let mut parser: Parser = Parser::new(lexer);
    let mut nodes: Vec<Node> = Vec::new();
    nodes.push(parser.func());
    nodes
}
