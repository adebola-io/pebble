pub const INJUNCTIONS: &'static [&'static str; 13] = &[
    "implement",
    "interface",
    "struct",
    "public",
    "utilize",
    "rec",
    "constant",
    "let",
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

pub const OPERATORS: &'static [&'static str; 37] = &[
    "&&=", "||=", "...", "||", "&&", "..", ">>", "<<", "=>", ">=", "<=", "!=", "==", "*=", "-=",
    "/=", "%=", "+=", "++", "**", "--", "::", "^", ".", "=", "+", "-", ">", "<", "/", "&", "|",
    "%", "*", "!", "~", "?",
];

/// Return the precedence of an operator according to the Pebble precedence chart.
/// An operator with a higher precedence has a higher influence on parsing arrangement than one with a lower precedence.
pub fn precedence_of(operator: &str) -> usize {
    match operator {
        ")" => 19,
        "." => 18,
        "**" => 10,
        "*" => 9,
        "+" => 8,
        "-" => 8,
        "[" => 18,
        _ => 0,
    }
}

pub fn is_identifier_char(c: char) -> bool {
    c == '$' || c == '_' || c.is_alphabetic() || c.is_digit(10)
}
pub fn is_injunction_value(value: &str) -> bool {
    INJUNCTIONS.contains(&value)
}
pub fn is_keyword(value: &str) -> bool {
    KEYWORDS.contains(&value)
}
