#![allow(unused)]
use std::{
    fmt::Display,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use crate::Error;

pub enum Mode {
    File,
    REPL,
}

/// Emits an error to the console.
pub fn emit_error<T: Error + Display>(error: T, span: Option<[[u64; 2]; 2]>, source: (Mode, &str)) {
    if let Mode::File = source.0 {}
}
