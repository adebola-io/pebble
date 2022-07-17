pub enum StringInnerToken {
    Sequence,
    Expression,
}
pub enum Token {
    Injunction { content: String },
    StringToken { tokens: Array<StringInnerToken> },
    Number { content: String },
    Bracket { content: String },
    Keyword { content: String },
    Identifier { content: String },
    Terminator { content: String },
    Operator { content: String },
}
