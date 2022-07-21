pub const INJUNCTIONS: &'static [&'static str; 12] = &[
    "implement",
    "interface",
    "struct",
    "public",
    "utilize",
    "constant",
    "variable",
    "enum",
    "function",
    "type",
    "use",
    "prepend",
];

pub fn is_identifier_char(c: char) -> bool {
    c == '$' || c == '_' || c.is_alphabetic() || c.is_digit(10)
}
pub fn is_injunction_value(value: &str) -> bool {
    INJUNCTIONS.contains(&value)
}
