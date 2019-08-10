use super::super::lex::token;
use super::error::CompileError;
use super::node::{Node, NodeType};
use std::collections::HashMap;
use token::{Token, TokenType};

struct Parser {
    /* 構文解析用の構造体 */
    tokens: Vec<Token>, //字句解析により得られるトークン列
    cur: Token,         //現在見ているトークン
    next: Token,        //一つ次のトークン
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Parser {
        /* Constructor */
        let cur: Token = tokens[0].clone();
        let next: Token = tokens[1].clone();
        Parser {
            tokens: tokens,
            cur: cur,
            next: next,
            pos: 2,
        }
    }
    fn next_token(&mut self) {
        /* オフセットを進める */
        self.cur = self.next.clone();
        if self.pos == self.tokens.len() {
            return;
        }
        self.next = self.tokens[self.pos].clone();
        self.pos += 1;
    }
    fn consume(&mut self, ty: &TokenType) -> bool {
        if &self.cur.ty == ty {
            self.next_token();
            return true;
        }
        false
    }
    pub fn expect(&mut self, ty: &TokenType) {
        if &self.next.ty == ty {
            self.next_token();
            return;
        }
        CompileError::PARSE(format!(
            "{} expected but got {}",
            ty.string(),
            self.next.ty.string()
        ))
        .found();
    }
    fn term(&mut self) -> Node {
        //identifier or integer-literal or array-literal
        let t: Token = self.cur.clone();
        self.next_token();
        match t.ty {
            TokenType::TkMinus => self.parse_minus(), //負数の解析
            TokenType::TkIntlit | TokenType::TkUintlit => Node::new_num(t), //数値リテラル
            TokenType::TkStrlit => Node::new_string(t), //文字列リテラル
            TokenType::TkCharlit => Node::new_char(t), //文字リテラル
            TokenType::TkPerStr | TokenType::TkPerChar | TokenType::TkPerInt => self.parse_array(t), //%記法
            TokenType::TkIdent => self.parse_ident(t), //変数解析
            _ => {
                CompileError::PARSE(format!("got invalid node {} when parsing term", t.dump()))
                    .found();
                Node::new(NodeType::INVALID)
            }
        }
    }
    /* 変数の解析を行う関数 */
    fn parse_ident(&mut self, t: Token) -> Node {
        let mut arguments: Vec<Node> = Vec::new();
        if self.cur.ty != TokenType::TkLparen {
            return Node::new_ident(t.literal);
        }
        //もし呼び出し式なら
        self.next_token();
        loop {
            if self.cur.ty == TokenType::TkRparen {
                break;
            }
            arguments.push(self.expr().clone());
            //self.next_token();
        }
        self.next_token();
        Node::new_call(t.literal, arguments)
    }
    fn parse_minus(&mut self) -> Node {
        let mut token: Token = self.cur.clone();
        self.next_token();
        token.literal = String::from("-") + &token.literal;
        Node::new_num(token)
    }
    fn parse_array(&mut self, ty: Token) -> Node {
        //%s[abc def ghi]
        self.consume(&TokenType::TkLbracket); //( -> abc
        let mut elements: Vec<Token> = Vec::new();
        while self.next.ty != TokenType::TkRbracket {
            elements.push(self.cur.clone());
            self.next_token();
        }
        self.expect(&TokenType::TkRbracket);
        self.next_token();
        match ty.ty {
            TokenType::TkPerStr => Node::new_strary(elements),
            TokenType::TkPerChar => Node::new_charary(elements),
            TokenType::TkPerInt => Node::new_intary(elements),
            _ => {
                CompileError::PARSE(format!("got invalid node {} when parsing array", ty.dump()))
                    .found();
                Node::new(NodeType::INVALID)
            }
        }
    }
    fn logor(&mut self) -> Node {
        // ||
        let mut lchild: Node = self.logand();
        loop {
            let t: Token = self.cur.clone();
            if !self.consume(&TokenType::TkLogor) {
                break;
            }
            lchild = Node::new_binop(t.ty, lchild.clone(), self.logand().clone());
        }
        lchild
    }
    fn logand(&mut self) -> Node {
        // &&
        let mut lchild: Node = self.equal();
        loop {
            let t: Token = self.cur.clone();
            if !self.consume(&TokenType::TkLogand) {
                break;
            }
            lchild = Node::new_binop(t.ty, lchild.clone(), self.equal().clone());
        }
        lchild
    }
    fn equal(&mut self) -> Node {
        // == or !=
        let mut lchild: Node = self.cmp();
        loop {
            let t: Token = self.cur.clone();
            if !self.consume(&TokenType::TkEq) && !self.consume(&TokenType::TkNoteq) {
                break;
            }
            lchild = Node::new_binop(t.ty, lchild.clone(), self.cmp().clone());
        }
        lchild
    }
    fn cmp(&mut self) -> Node {
        // <= or < or >= or >
        let mut lchild: Node = self.shift();
        loop {
            let t: Token = self.cur.clone();
            if !self.consume(&TokenType::TkLt)
                && !self.consume(&TokenType::TkLteq)
                && !self.consume(&TokenType::TkGt)
                && !self.consume(&TokenType::TkGteq)
            {
                break;
            }
            lchild = Node::new_binop(t.ty, lchild.clone(), self.shift().clone());
        }
        lchild
    }
    fn shift(&mut self) -> Node {
        // << or >>
        let mut lchild: Node = self.adsub();
        loop {
            let t: Token = self.cur.clone();
            if !self.consume(&TokenType::TkLshift) && !self.consume(&TokenType::TkRshift) {
                break;
            }
            lchild = Node::new_binop(t.ty, lchild.clone(), self.adsub().clone());
        }
        lchild
    }
    fn adsub(&mut self) -> Node {
        // + or -
        let mut lchild: Node = self.muldiv();
        loop {
            let t: Token = self.cur.clone();
            if !self.consume(&TokenType::TkPlus) && !self.consume(&TokenType::TkMinus) {
                break;
            }
            lchild = Node::new_binop(t.ty, lchild.clone(), self.muldiv().clone());
        }
        lchild
    }
    fn muldiv(&mut self) -> Node {
        // * or / or %
        let mut lchild: Node = self.term();
        loop {
            let t: Token = self.cur.clone();
            if !self.consume(&TokenType::TkStar)
                && !self.consume(&TokenType::TkSlash)
                && !self.consume(&TokenType::TkPercent)
            {
                break;
            }
            lchild = Node::new_binop(t.ty, lchild.clone(), self.term().clone());
        }
        lchild
    }
    fn expr(&mut self) -> Node {
        match self.cur.ty {
            _ => self.logor(),
        }
    }
    fn stmt(&mut self) -> Node {
        match self.cur.ty {
            TokenType::TkReturn => self.parse_return(),
            TokenType::TkLet => self.parse_let(),
            TokenType::TkLoop => self.parse_loop(),
            TokenType::TkFor => self.parse_for(),
            TokenType::TkIf => self.parse_if(),
            TokenType::TkStruct => self.parse_struct(),
            _ => {
                CompileError::PARSE(format!(
                    "got invalid node {} when parsing stmt",
                    self.cur.dump()
                ))
                .found();
                Node::new(NodeType::INVALID)
            }
        }
    }
    fn parse_struct(&mut self) -> Node {
        self.consume(&TokenType::TkStruct);
        if self.cur.ty != TokenType::TkIdent {
            CompileError::PARSE(format!("expected identifier but got {}", self.cur.literal))
                .found();
        }
        let struct_name: String = self.cur.literal.clone();
        self.consume(&TokenType::TkIdent);
        self.consume(&TokenType::TkLbrace);
        let mut members: Vec<Node> = Vec::new();
        while self.next.ty != TokenType::TkRbrace {
            if self.cur.ty != TokenType::TkIdent {
                CompileError::PARSE(format!("expected identifier but got {}", self.cur.literal))
                    .found();
            }
            let member_name: String = self.cur.literal.clone();
            self.consume(&TokenType::TkIdent);
            self.consume(&TokenType::TkColon);
            if !self.cur.ty.is_typename() {
                CompileError::PARSE(format!("expected typename but got {}", self.cur.literal))
                    .found();
            }
            let typename: TokenType = self.cur.ty.clone();
            self.next_token();
            members.push(Node::new_member(member_name, typename));
            self.consume(&TokenType::TkComma);
        }
        self.consume(&TokenType::TkRbrace);
        Node::new_structs(struct_name, members)
    }
    fn parse_if(&mut self) -> Node {
        //if expr {} else {}
        let if_keyword: TokenType = self.cur.ty.clone();
        self.next_token(); //if -> the start of expr
        let condition: Node = self.expr();
        self.consume(&TokenType::TkLbrace);
        let mut statements: Vec<Node> = Vec::new();
        while self.cur.ty != TokenType::TkRbrace {
            let n: Node = self.stmt();
            statements.push(n);
        }
        self.consume(&TokenType::TkRbrace);
        let mut alternatives: Vec<Node> = Vec::new();
        if self.cur.ty == TokenType::TkElse {
            self.consume(&TokenType::TkElse);
            self.consume(&TokenType::TkLbrace);
            while self.cur.ty != TokenType::TkRbrace {
                let n: Node = self.stmt();
                alternatives.push(n.clone());
            }
            self.consume(&TokenType::TkRbrace);
        }
        Node::new_ifs(
            if_keyword,
            condition,
            statements,
            TokenType::TkElse,
            alternatives,
        )
    }
    fn parse_return(&mut self) -> Node {
        let ret_keyword: TokenType = self.cur.ty.clone();
        self.next_token(); // return -> the start of expr
        Node::new_rets(ret_keyword, self.expr())
    }
    fn parse_let(&mut self) -> Node {
        let let_keyword: TokenType = self.cur.ty.clone();
        self.next_token(); // let -> identifier
        let ident_name: Vec<Node> = vec![Node::new_ident(self.cur.literal.clone())];
        self.expect(&TokenType::TkColon);
        self.next_token(); // : -> typename
        if !self.cur.ty.is_typename()
            && self.cur.ty != TokenType::TkAmpersand(Box::new(TokenType::TkNone))
        {
            CompileError::PARSE(format!("expected typename but got {}", self.cur.literal)).found();
        }
        let mut typename: TokenType = self.cur.ty.clone();
        if let TokenType::TkAmpersand(ref mut n) = typename {
            self.next_token();
            *n = Box::new(self.cur.ty.clone());
        }
        self.expect(&TokenType::TkAssign);
        self.next_token(); // = -> the start of expr
        let expr: Node = self.expr();
        Node::new_lets(let_keyword, ident_name, typename, expr)
    }
    fn parse_loop(&mut self) -> Node {
        let loop_keyword: TokenType = self.cur.ty.clone();
        self.expect(&TokenType::TkLbrace);
        self.next_token(); // { -> first of statements
        let mut statements: Vec<Node> = Vec::new();
        while self.cur.ty != TokenType::TkRbrace {
            let n: Node = self.stmt();
            statements.push(n.clone());
        }
        self.consume(&TokenType::TkRbrace);
        Node::new_loops(loop_keyword, statements)
    }
    fn parse_for(&mut self) -> Node {
        let for_keyword: TokenType = self.cur.ty.clone();
        self.next_token(); //for -> identifier
        let tmp_ident: String = self.cur.literal.clone();
        self.expect(&TokenType::TkIn);
        self.next_token(); // in -> identifier
        let src_ident: String = self.cur.literal.clone();
        self.expect(&TokenType::TkLbrace);
        self.next_token(); // { -> first of statements
        let mut statements: Vec<Node> = Vec::new();
        while self.cur.ty != TokenType::TkRbrace {
            let n: Node = self.stmt();
            statements.push(n.clone());
        }
        self.consume(&TokenType::TkRbrace);
        Node::new_fors(for_keyword, tmp_ident, src_ident, statements)
    }
    fn func(&mut self) -> Node {
        let func_name: String = self.cur.literal.clone();
        self.expect(&TokenType::TkLparen);
        let mut arguments: HashMap<String, TokenType> = HashMap::new();
        while self.next.ty != TokenType::TkRparen {
            self.expect(&TokenType::TkIdent);
            let ident_name: String = self.cur.literal.clone();
            self.expect(&TokenType::TkColon);
            self.next_token(); // : -> typename
            if !self.cur.ty.is_typename() {
                CompileError::PARSE(format!("expected typename but got {}", self.cur.literal))
                    .found();
            }
            arguments.insert(ident_name, self.cur.ty.clone());
            if self.next.ty == TokenType::TkComma {
                self.expect(&TokenType::TkComma);
            }
        }
        self.expect(&TokenType::TkRparen);
        let ret: TokenType = if self.next.ty == TokenType::TkArrow {
            self.next_token();
            self.next_token();
            self.cur.ty.clone()
        } else {
            TokenType::TkIllegal
        };

        self.expect(&TokenType::TkLbrace);
        self.next_token();
        let mut statements: Vec<Node> = Vec::new();
        while self.cur.ty != TokenType::TkRbrace {
            let n: Node = self.stmt();
            statements.push(n.clone());
        }
        self.consume(&TokenType::TkRbrace);
        Node::new_func(func_name, arguments, ret, statements)
    }
}

pub fn parse(tokens: Vec<Token>) -> Vec<Node> {
    let mut parser: Parser = Parser::new(tokens);
    let mut nodes: Vec<Node> = Vec::new();
    while parser.cur.ty == TokenType::TkF {
        parser.next_token();
        nodes.push(parser.func());
    }
    parser.consume(&TokenType::TkEof);
    nodes
}
