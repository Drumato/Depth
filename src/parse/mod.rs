pub mod error;
pub mod lr;
pub mod node;
pub mod parser;
#[cfg(test)]

mod tests {
    use super::super::lex::lexing::Lexer;
    use super::super::lex::token::{Token, TokenType, TokenVal};
    use super::super::parse::parser;
    use super::super::parse::parser::Parser;
    use super::node::Node;
    #[test]
    fn test_new_parser() {
        let input_str: &str = "f main(){}";
        let lexer = Lexer::new(input_str.to_string()).unwrap();
        let parser: Parser = Parser::new(lexer);
        assert_eq!(
            Token::new((TokenType::TkF, String::from("f"), TokenVal::InVal)).dump(),
            parser.cur.dump()
        );
        assert_eq!(
            Token::new((TokenType::TkIdent, String::from("main"), TokenVal::InVal)).dump(),
            parser.next.dump()
        );
    }
    #[test]
    fn test_consume() {
        let input_str: &str = "f main(){}";
        let lexer = Lexer::new(input_str.to_string()).unwrap();
        let mut parser: Parser = Parser::new(lexer);
        assert_eq!(true, parser.consume(&TokenType::TkF));
    }
}
