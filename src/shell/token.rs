#[derive(Clone, Debug)]
pub struct Token {
    pub kind: Kind,
    pub start: usize,
    pub end: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Kind {
    StringLiteral,
    Use,
    Eq,
    EqCmp,
    EndStmt,
    Eof,
}
