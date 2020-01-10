#[derive(Debug, PartialEq, Copy, Clone, Eq, Hash)]
pub struct Operation {
    pub kind: OperationKind,
    pub argument: u32,
    pub offset: i32,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, PartialEq, Copy, Clone, Eq, Hash)]
pub enum OperationKind {
    Add,       // +
    Sub,       // -
    Right,     // >
    Left,      // <
    Read,      // ,
    Write,     // .
    BeginLoop, // [
    EndLoop,   // ],
    SetZero,   // [-], [+]
}
