pub const INJUNCTIONS: &'static [&'static str; 13] = &[
    "implement",
    "interface",
    "struct",
    "public",
    "utilize",
    "record",
    "constant",
    "set",
    "enum",
    "func",
    "type",
    "use",
    "prepend",
];

pub const KEYWORDS: &'static [&'static str; 19] = &[
    "if", "else", "for", "in", "of", "match", "case", "from", "as", "do", "while", "break",
    "continue", "return", "crash", "try", "recover", "println", "readonly",
];

pub const OPERATORS: &'static [&'static str; 34] = &[
    "&&=", "||=", "...", "||", "&&", "..", ">=", "<=", "!=", "==", "*=", "-=", "/=", "%=", "+=",
    "++", "**", "--", "::", "^", ".", "=", "+", "-", ">", "<", "/", "&", "|", "%", "*", "!", "~",
    "?",
];

pub fn is_identifier_char(c: char) -> bool {
    c == '$' || c == '_' || c.is_alphabetic() || c.is_digit(10)
}
pub fn is_injunction_value(value: &str) -> bool {
    INJUNCTIONS.contains(&value)
}
pub fn is_keyword(value: &str) -> bool {
    KEYWORDS.contains(&value)
}
