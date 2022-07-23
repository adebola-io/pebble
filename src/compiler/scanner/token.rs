#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum StringInnerToken {
    StringSequence { value: String, loc: [usize; 4] },
    StringExpression { tokens: Vec<Token>, loc: [usize; 4] },
}
#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub struct Comment {
    /// The kind of comment tokenized, either block, line or doc.
    pub kind: CommentKind,
    pub value: String,
    pub loc: [usize; 4],
}
#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Token {
    /// A token that begins with an @ symbol.
    Injunction {
        value: String,
        loc: [usize; 4],
    },
    Number {
        /// The kind of number tokenized, whether a decimal, hexadecimal, octal, exponent, etc.
        kind: NumericKind,
        raw: String,
        loc: [usize; 4],
    },
    Character {
        value: String,
        loc: [usize; 4],
    },
    StringToken {
        inner: Vec<StringInnerToken>,
        loc: [usize; 4],
    },
    Identifier {
        value: String,
        loc: [usize; 4],
    },
    Bracket {
        kind: BracketKind,
        loc: [usize; 4],
    },
    Keyword {
        value: String,
        loc: [usize; 4],
    },
    Comma {
        loc: [usize; 4],
    },
    SemiColon {
        loc: [usize; 4],
    },
    Colon {
        loc: [usize; 4],
    },
    Operator {
        value: String,
        loc: [usize; 4],
    },
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
