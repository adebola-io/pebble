#[derive(Debug, PartialEq, Clone)]
pub struct Identifier<'a> {
    value: &'a str,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Literal<'a> {
    kind: LiteralKind,
    raw: &'a str,
    value: &'a str,
}

#[derive(Debug, PartialEq, Clone)]
pub enum LiteralKind {
    StringLiteral,
    NumericLiteral,
    BooleanLiteral,
}
