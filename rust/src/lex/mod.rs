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
}
