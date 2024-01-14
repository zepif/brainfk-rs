#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Token {
    Right,
    Left,
    Add,
    Subtract,
    PrintChar,
    GetChar,
    StartLoop,
    EndLoop,
    EOF,
}
