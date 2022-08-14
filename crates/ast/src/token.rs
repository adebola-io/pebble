use crate::{identifier::Literal, Identifier, Keyword, Operator, Punctuation, TextRange};

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind<'a> {
    Operator(Operator),
    Punctuation(Punctuation),
    Keyword(Keyword),
    Literal(Literal<'a>),
    Identifier(Identifier<'a>),
}

/// A piece of code collected when scanning the input source file.
#[derive(Debug, PartialEq, Clone)]
pub struct Token<'a> {
    kind: TokenKind<'a>,
    span: TextRange,
}
