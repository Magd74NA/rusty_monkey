pub type TokenType = &'static str;

pub const ILLEGAL: TokenType = "ILLEGAL";
pub const EOF: TokenType = "EOF";

// Identifiers + literals
pub const IDENT: TokenType = "IDENT";
pub const INT: TokenType = "INT";

// Operators
pub const ASSIGN: TokenType = "=";
pub const PLUS: TokenType = "+";
pub const MINUS: TokenType = "-";
pub const SLASH: TokenType = "/";
pub const ASTERISK: TokenType = "*";
pub const GT: TokenType = ">";
pub const LT: TokenType = "<";
pub const BANG: TokenType = "!";

// Two-char operators
pub const EQ: TokenType = "==";
pub const NOT_EQ: TokenType = "!=";

// Delimiters
pub const COMMA: TokenType = ",";
pub const SEMICOLON: TokenType = ";";

pub const LPAREN: TokenType = "(";
pub const RPAREN: TokenType = ")";
pub const LBRACE: TokenType = "{";
pub const RBRACE: TokenType = "}";

// Keywords
pub const FUNCTION: TokenType = "FUNCTION";
pub const LET: TokenType = "LET";
pub const RETURN: TokenType = "RETURN";
pub const IF: TokenType = "IF";
pub const TRUE: TokenType = "TRUE";
pub const ELSE: TokenType = "ELSE";
pub const FALSE: TokenType = "FALSE";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

pub fn lookup_ident(ident: &str) -> TokenType {
    match ident {
        "fn" => FUNCTION,
        "let" => LET,
        "return" => RETURN,
        "if" => IF,
        "else" => ELSE,
        "true" => TRUE,
        "false" => FALSE,
        _ => IDENT,
    }
}
