mod semantic;
mod syntax;

pub use semantic::*;
pub use syntax::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Error {
    ParserError,
    RuntimeError,
}

pub enum LexicalError {
    /// A String does not have a closing quote mark.
    UnterminatedStringLiteral,
    /// A character token has more than one character in its body.
    InvalidCharacterCount,
}

impl Error {
    pub fn scanner_error(error_code: i32) -> &'static str {
        match error_code {
            1 => "Unterminated String Literal.",
            _ => unreachable!(),
        }
    }
}
