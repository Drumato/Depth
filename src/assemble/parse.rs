//use super::super::ce::types::Error;
use super::lex::Token;
use std::collections::HashMap;
pub enum Inst {
    /*
BINARG(usize),
NOARG(usize),
*/}
pub struct Info {
    //inst_name: String,
}

struct Parser {
    //tokens: Vec<Token>,
    info_map: HashMap<usize, Info>,
    insts: Vec<Inst>,
    //entry: usize,
}
impl Parser {
    fn parse(&mut self) {}
}
pub fn parsing(_tokens: Vec<Token>) -> (Vec<Inst>, HashMap<usize, Info>) {
    let mut parser: Parser = Parser {
        // tokens: tokens,
        info_map: HashMap::new(),
        insts: Vec::new(),
        //entry: 0,
    };
    parser.parse();
    (parser.insts, parser.info_map)
}
