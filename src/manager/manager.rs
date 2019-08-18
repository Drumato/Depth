use super::super::token::token::Token;
pub struct Manager {
    pub tokens: Vec<Token>,
}

impl Manager {
    pub fn new(tokens: Vec<Token>) -> Manager {
        Manager { tokens: tokens }
    }
}
