mod _trait;
mod emit;
mod resolve;
mod types;
mod syntax;

use std::fmt::Display;

pub use _trait::*;
pub use emit::*;
pub use resolve::*;
pub use types::*;
pub use syntax::*;

pub enum LexicalError {
    /// A character not included in the language specification has been encountered.
    UnknownToken(String),
    /// A String does not have a closing quote mark.
    UnterminatedStringLiteral,
    /// A character token has more than one character in its body.
    InvalidCharacterCount,
}

impl Display for LexicalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.",
            match self {
                LexicalError::UnknownToken(x) => format!("Unexpected token {x}"),
                LexicalError::UnterminatedStringLiteral => format!("String has no closing quotes"),
                LexicalError::InvalidCharacterCount => format!(
                    "Invalid character. Characters can only consist of a single..well, character"
                ),
            }
        )
    }
}
