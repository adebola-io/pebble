#[allow(dead_code)]
pub enum StringInnerToken {
    Sequence,
    Expression,
}
#[allow(dead_code)]
pub enum Token {
    Injunction { content: String },
    StringToken { tokens: Vec<StringInnerToken> },
    Number { content: String },
    Bracket { content: String },
    Keyword { content: String },
    Identifier { content: String },
    Terminator { content: String },
    Operator { content: String },
}
