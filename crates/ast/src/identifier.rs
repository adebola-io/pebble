#[derive(Debug, PartialEq, Clone)]
pub struct Identifier<'a> {
    value: &'a str,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Literal {
    pub kind: LiteralKind,
    pub value: String,
}

#[derive(Debug, PartialEq, Clone)]
pub enum LiteralKind {
    StringLiteral,
    NumericLiteral,
    BooleanLiteral,
}
