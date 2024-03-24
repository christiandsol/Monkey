use crate::token_type::{Token, TokenType};
use std::vec::Vec; // Import TokenType from the token_type module
pub struct Scanner {
    pub source: String,
    pub tokens: Vec<Token>,
    pub start: i32,
    pub current: i32,
    pub line: i32,
}

impl Scanner {
    pub fn new(&mut self, source: String, tokens: Vec<Token>) -> Self {
        Scanner {
            source,
            tokens,
            start: 0, 
            current: 0, 
            line: 1 
        }
    }
    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len().try_into().unwrap()
    }

    pub fn scanner(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scanner();
        }
        self.tokens.push(Token::new(TokenType::EOF, String::new(), None, self.line));
        &self.tokens
    }
}
