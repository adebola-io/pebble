mod helpers;
mod token;
use helpers::*;
use token::{
    Comment, CommentKind,
    Token::{self, *},
};
#[derive(Debug)]
pub struct ScannerError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

type ScanResult = Result<Vec<Token>, ScannerError>;

pub fn scan(content: String) -> ScanResult {
    let content: Vec<char> = content.chars().collect();
    let mut scanner = Scanner::new(content);
    scanner.run()?;
    println!("{:?}", scanner.comments);
    Ok(scanner.tokens)
}

struct Scanner {
    source: Vec<char>,
    /// The current character in the stream.
    current: char,
    tokens: Vec<Token>,
    /// The tracker for the lines and columns in the source text.
    pos: [usize; 2],
    index: usize,
    end: bool,
    store: [usize; 4],
    comments: Vec<Comment>,
}

impl Scanner {
    fn new(content: Vec<char>) -> Self {
        Scanner {
            source: content,
            tokens: vec![],
            comments: vec![],
            current: '\0',
            pos: [1, 0],
            index: 0,
            store: [0, 0, 0, 0],
            end: false,
        }
    }
    /// Run the scanner.
    fn run(&mut self) -> Result<(), ScannerError> {
        self.set();
        while !self.end {
            if self.sees("##") {
                println!("Hello.");
            }
            match self.current {
                '/' => match self.lookahead(1) {
                    Some('*') => self.scan_block_comment()?,
                    Some('/') => self.scan_line_comment()?,
                    _ => {}
                },
                '@' => self.scan_injunction()?,
                _ => {}
            }
            self.next();
        }
        Ok(())
    }
    /// Set the scanner to the positions to start scanning.
    fn set(&mut self) {
        if let Some(c) = self.source.get(self.index) {
            self.current = *c;
        } else {
            self.end = true;
        };
        self.shift();
    }
    /// Advance to the next character in the stream.
    fn next(&mut self) {
        self.index += 1;
        if self.index >= self.source.len() {
            self.end = true;
        } else {
            self.current = self.source[self.index];
            self.shift();
        }
    }
    /// Advance by a particular length.
    fn next_by(&mut self, l: usize) {
        for _ in 0..l {
            self.next();
        }
    }
    /// Shift the line tracker.
    fn shift(&mut self) {
        if self.current == '\n' {
            self.pos[0] += 1;
            self.pos[1] = 0;
        } else {
            self.pos[1] += 1;
        }
    }
    fn lookahead(&self, i: usize) -> Option<char> {
        if self.index + i > self.source.len() {
            None
        } else {
            Some(self.source[self.index + i])
        }
    }
    fn expects(&self, pattern: &str) -> bool {
        let end = self.index + pattern.len();
        if end > self.source.len() {
            false
        } else {
            let section: String = self.source[self.index..end].iter().collect();
            section == pattern.to_string()
        }
    }
    fn sees(&self, _pattern: &str) -> bool {
        true
    }
    /// Emits an error encountered during scanning.
    fn error(&self, message: &str) -> Result<(), ScannerError> {
        Err(ScannerError {
            message: message.to_string(),
            line: self.pos[0],
            column: self.pos[1],
        })
    }
    /// Takes a snapshot of the position of the scanner at a point during the scanning.
    fn loc_start(&mut self) {
        self.store[0] = self.pos[0];
        self.store[1] = self.pos[1];
    }
    /// Takes a snapshot of the position of the scanner at a point during the scanning.
    fn loc_end(&mut self) {
        self.store[2] = self.pos[0];
        self.store[3] = self.pos[1];
    }
    fn scan_block_comment(&mut self) -> Result<(), ScannerError> {
        self.loc_start();
        self.next_by(2);
        let mut value = String::new();
        while !(self.end || self.expects("*/")) {
            value.push(self.current);
            self.next()
        }
        if self.end {
            self.error("Unclosed block comment.")?
        }
        self.next_by(2);
        self.loc_end();
        self.comments.push(Comment {
            kind: CommentKind::Block,
            value,
            loc: self.store,
        });
        Ok(())
    }
    fn scan_line_comment(&mut self) -> Result<(), ScannerError> {
        self.loc_start();
        self.next_by(2);
        let mut value = String::new();
        while !(self.end || self.current == '\n') {
            value.push(self.current);
            self.next();
        }
        if !self.end {
            self.next();
        }
        self.loc_end();
        self.comments.push(Comment {
            kind: CommentKind::Line,
            value,
            loc: self.store,
        });
        Ok(())
    }
    fn scan_injunction(&mut self) -> Result<(), ScannerError> {
        self.loc_start();
        self.next();
        if !is_identifier_char(self.current) || self.current.is_digit(10) {
            self.error("The scannner expected an identifier character.")?
        }
        let mut value = String::new();
        while !self.end && is_identifier_char(self.current) {
            value.push(self.current);
            self.next();
        }
        self.loc_end();
        if !is_injunction_value(&value) {
            let message = format!("Unrecognized injunction \"{}\".", value);
            self.error(message.as_str())?
        }
        self.tokens.push(Injunction {
            value,
            loc: self.store,
        });
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_scans_injunction_token() {
        let tokens = scan("@public ".to_string()).unwrap();
        assert_eq!(
            tokens[0],
            Injunction {
                value: String::from("public"),
                loc: [1, 1, 1, 8]
            }
        );
    }
}
