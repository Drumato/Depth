pub mod lexing;
pub mod token;

#[cfg(test)]
mod tests {
    #[test]
    fn test_lookup() {
        assert_eq!(
            super::token::Token::new((super::token::TokenType::TkMutable, "mut".to_string(), ()))
                .lookup(),
            true
        );
        assert_eq!(
            super::token::Token::new((super::token::TokenType::TkF, "f".to_string(), ())).lookup(),
            true
        );
        assert_eq!(
            super::token::Token::new((super::token::TokenType::TkIntlit, "30".to_string(), 30))
                .lookup(),
            false
        );
        assert_eq!(
            super::token::Token::new((super::token::TokenType::TkCharlit, "a".to_string(), ()))
                .lookup(),
            false
        );
        assert_eq!(
            super::token::Token::new((super::token::TokenType::TkIdent, "x".to_string(), ()))
                .lookup(),
            false
        );
        assert_eq!(
            super::token::Token::new((
                super::token::TokenType::TkReallit,
                "3.0".to_string(),
                "3.0"
            ))
            .lookup(),
            false
        );
    }

    #[test]
    fn test_new_lexer() {
        let input_str: &str = "f main(){}";
        let lexer = super::lexing::Lexer::new(input_str.to_string()).unwrap();
        assert_eq!(0x66, lexer.ch); //f
        assert_eq!(0, lexer.pos);
        assert_eq!(1, lexer.npos);
        assert_eq!(input_str, lexer.input);
    }
    #[test]
    fn test_read_char() {
        let input_str: &str = "f main(){}";
        let mut lexer = super::lexing::Lexer::new(input_str.to_string()).unwrap();
        lexer.read_char();
        assert_eq!(0x20, lexer.ch); //' '
    }
    #[test]
    fn test_peek_char() {
        let input_str: &str = "main(){}";
        let lexer = super::lexing::Lexer::new(input_str.to_string()).unwrap();
        assert_eq!('a', lexer.peak_char());
    }
    #[test]
    fn test_peek_byte() {
        let input_str: &str = "main(){}";
        let lexer = super::lexing::Lexer::new(input_str.to_string()).unwrap();
        assert_eq!(0x61, lexer.peak_byte());
    }
    #[test]
    fn test_read_ident() {
        let input_str: &str = "main(){}";
        let mut lexer = super::lexing::Lexer::new(input_str.to_string()).unwrap();
        assert_eq!(String::from("main"), lexer.read_ident());
    }
    #[test]
    fn test_read_number() {
        let input_str: &str = "123main";
        let mut lexer = super::lexing::Lexer::new(input_str.to_string()).unwrap();
        assert_eq!(String::from("123"), lexer.read_number());
    }
    #[test]
    fn test_skip_whitespace() {
        let input_str: &str = "           123";
        let mut lexer = super::lexing::Lexer::new(input_str.to_string()).unwrap();
        lexer.skip_whitespace();
        assert_eq!(String::from("123"), lexer.read_number());
    }
}
