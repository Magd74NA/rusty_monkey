use crate::token;

pub struct Lexer {
    input: String,
    position: u32,
    read_position: u32,
    ch: u8
}

