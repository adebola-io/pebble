#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Error {
    ScannerError,
    ParserError,
    RuntimeError,
}
