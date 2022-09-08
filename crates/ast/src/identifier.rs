#[derive(Debug, PartialEq, Clone)]
pub struct TokenIdentifier {
    pub value: String,
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
    CharacterLiteral,
}
