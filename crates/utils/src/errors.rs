#[derive(Debug)]
pub struct CompileError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}
