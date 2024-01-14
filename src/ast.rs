#[derive(Debug)]
pub enum AST {
    Root(Vec<AST>),
    Right(usize),
    Left(usize),
    Add(usize),
    Subtract(usize),
    PrintChar,
    GetChar,
    Loop(Vec<AST>),
}
