use crate::{lexer, token_type::{Token, TokenType}};
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
            line: 1,
        }
    }
    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len().try_into().unwrap()
    }

    pub fn advance(&mut self) -> char {
        let c = self.source.as_bytes()[self.current as usize] as char;
        self.current += 1;
        c
    }
    pub fn add_token_helper(
        &mut self,
        token_type: TokenType,
        literal: Option<Box<dyn std::any::Any>>,
    ) {
        let text: String = self.source[self.start as usize..self.current as usize].to_string();
        self.tokens
            .push(Token::new(token_type, text, literal, self.line))
    }

    pub fn add_token(&mut self, token_type: TokenType) {
        self.add_token_helper(token_type, None)
    }
    pub fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if (self.source.chars().nth(self.current as usize) != Some(expected)) {
            return false;
        }
        self.current += 1;
        true
    }

    pub fn peek(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current as usize)
    }

    pub fn scan_token(&self) {
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenType::LEFT_PAREN),
            ')' => self.add_token(TokenType::RIGHT_PAREN),
            '{' => self.add_token(TokenType::LEFT_BRACE),
            '}' => self.add_token(TokenType::RIGHT_BRACE),
            ',' => self.add_token(TokenType::COMMA),
            '.' => self.add_token(TokenType::DOT),
            '-' => self.add_token(TokenType::MINUS),
            '+' => self.add_token(TokenType::PLUS),
            ';' => self.add_token(TokenType::SEMICOLON),
            '*' => self.add_token(TokenType::STAR),
            '!' => {
                if self.match_char('=') {
                    self.add_token(TokenType::BANG_EQUAL)
                } else {
                    self.add_token(TokenType::BANG)
                }
            }
            '=' => {
                if self.match_char('=') {
                    self.add_token(TokenType::EQUAL_EQUAL);
                } else {
                    self.add_token(TokenType::EQUAL);
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.add_token(TokenType::LESS_EQUAL);
                } else {
                    self.add_token(TokenType::LESS);
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.add_token(TokenType::GREATER_EQUAL);
                } else {
                    self.add_token(TokenType::GREATER);
                }
            }
            '/' => {
                if self.match_char('/') {
                    while (peek() != '\n' && !self.is_at_end()) {
                        advance();
                    }
                } else {
                    self.add_token(TokenType::GREATER);
                }
            }
            '\n' => {self.line+=1}
            ' ' | '\r' | '\t' =>{}
            _ => {
                lexer::error(String::from("Unexpected line"), self.line)
            }
        }
    }

    pub fn scanner(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens
            .push(Token::new(TokenType::EOF, String::new(), None, self.line));
        &self.tokens
    }
}
