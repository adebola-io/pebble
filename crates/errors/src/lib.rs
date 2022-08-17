#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Error {
    ScannerError,
    ParserError,
    RuntimeError,
}

impl Error {
    pub fn scanner_error(error_code: i32) -> &'static str {
        match error_code {
            0 => "Unterminated String Literal.",
            _ => unreachable!(),
        }
    }
}
