#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TokenKind {
    Add,       // +
    Sub,       // -
    Right,     // >
    Left,      // <
    Read,      // ,
    Write,     // .
    BeginLoop, // [
    EndLoop,   // ]
}
