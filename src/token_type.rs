

pub enum TokenType {
    // Single-character tokens.
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
}

pub struct Token {
    pub type: TokenType, 
    pub mean_blob: String, 
    pub literal: Option<Box<dyn std::any::Any>>,
    pub line: i32,
}



impl Token {
    pub fn new(token_type: TokenType, mean_blob: String,
        literal: Option<Box<dyn std::any::Any>>, line: i32) -> Self {
        Token { TokenType: token_type, mean_blob,
            literal, line }
    }
    pub fn to_string() -> String {
        let formatted_string = format!("{} {} {:?}", type, mean_blob, literal);
    }
}
