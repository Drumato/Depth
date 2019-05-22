pub mod lr;
pub mod node;
#[cfg(test)]

mod tests {
    use super::super::lex::lexing::Lexer;
    use super::super::lex::token::{Token, TokenType, TokenVal};
    use super::lr::Parser;
    #[test]
    fn test_new_parser() {
        let input_str: &str = "f main(){}";
        let mut lexer = Lexer::new(input_str.to_string()).unwrap();
        let mut parser: Parser = Parser::new(lexer);
        assert_eq!(
            Token::new((TokenType::TkF, String::from("f"), TokenVal::InVal)).dump(),
            parser.cur.dump()
        );
        assert_eq!(
            Token::new((TokenType::TkIdent, String::from("main"), TokenVal::InVal)).dump(),
            parser.next.dump()
        );
    }
}
