#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum StringInnerToken {
    Sequence,
    Expression,
}
#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Token {
    Comment {
        /// The kind of comment tokenized, either block, line or doc.
        kind: CommentKind,
        value: String,
    },
    /// A token that begins with an @ symbol.
    Injunction {
        value: String,
        loc: [usize; 4],
    },
    Number {
        /// The kind of number tokenized, whether a decimal, hexadecimal, octal, exponent, etc.
        kind: NumericKind,
        raw: String,
    },
    Character {
        value: String,
    },
    StringToken {
        inner: Vec<StringInnerToken>,
    },
    Identifier {
        value: String,
    },
    Bracket {
        kind: BracketKind,
    },
    Keyword {
        value: String,
    },
    Comma,
    SemiColon,
    Operator {
        value: String,
    },
    EOF,
}
#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum CommentKind {
    Block,
    Line,
    Documentation,
}
#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum NumericKind {
    Hexadecimal,
    Decimal,
    Octal,
    Binary,
}
#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum BracketKind {
    LCurly,
    RCurly,
    LSquare,
    RSquare,
    RParen,
    LParen,
}
