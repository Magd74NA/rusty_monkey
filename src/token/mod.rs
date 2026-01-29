#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    Illegal,
    Eof,
    // Identifiers + literals
    Ident,
    Int,
    // Operators
    Assign,
    Plus,
    Minus,
    Slash,
    Asterisk,
    Gt,
    Lt,
    Bang,
    // Two-char operators
    Eq,
    NotEq,
    // Delimiters
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    // Keywords
    Function,
    Let,
    Return,
    If,
    True,
    Else,
    False,
}

impl TokenType {
    pub fn as_str(self) -> &'static str {
        match self {
            TokenType::Illegal => "ILLEGAL",
            TokenType::Eof => "EOF",
            TokenType::Ident => "IDENT",
            TokenType::Int => "INT",
            TokenType::Assign => "=",
            TokenType::Plus => "+",
            TokenType::Minus => "-",
            TokenType::Slash => "/",
            TokenType::Asterisk => "*",
            TokenType::Gt => ">",
            TokenType::Lt => "<",
            TokenType::Bang => "!",
            TokenType::Eq => "==",
            TokenType::NotEq => "!=",
            TokenType::Comma => ",",
            TokenType::Semicolon => ";",
            TokenType::LParen => "(",
            TokenType::RParen => ")",
            TokenType::LBrace => "{",
            TokenType::RBrace => "}",
            TokenType::Function => "FUNCTION",
            TokenType::Let => "LET",
            TokenType::Return => "RETURN",
            TokenType::If => "IF",
            TokenType::True => "TRUE",
            TokenType::Else => "ELSE",
            TokenType::False => "FALSE",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

pub fn lookup_ident(ident: &str) -> TokenType {
    match ident {
        "fn" => TokenType::Function,
        "let" => TokenType::Let,
        "return" => TokenType::Return,
        "if" => TokenType::If,
        "else" => TokenType::Else,
        "true" => TokenType::True,
        "false" => TokenType::False,
        _ => TokenType::Ident,
    }
}
