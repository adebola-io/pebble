pub const KEYWORDS: &'static [&'static str; 18] = &[
    "if",
    "else",
    "for",
    "fn",
    "in",
    "of",
    "loop",
    "from",
    "as",
    "implements",
    "while",
    "break",
    "continue",
    "return",
    "crash",
    "try",
    "recover",
    "println",
];
pub const INJUNCTIONS: &'static [&'static str; 15] = &[
    "implement",
    "interface",
    "class",
    "public",
    "module",
    "specify",
    "record",
    "const",
    "let",
    "enum",
    "function",
    "type",
    "use",
    "prepend",
    "tests",
];
#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    For,
    If,
    Else,
    Fn,
    In,
    Loop,
    Break,
    From,
    As,
    While,
    Continue,
    Implements,
    Return,
    Crash,
    Try,
    Recover,
    Println,
    Injunction(Injunction),
}
#[derive(Debug, Clone, PartialEq)]
pub enum Injunction {
    Function,
    Type,
    Class,
    Record,
    Const,
    Let,
    Use,
    Prepend,
    Test,
    Enum,
    Interface,
    Implement,
    Module,
    Public,
    /// An unrecognized injunction.
    Unknown(String),
}
