use ast::{TextSpan, Token, KEYWORDS, OPERATORS};
use errors::Error;
use std::collections::HashMap;

/// A state machine that goes over the input text and scans it into a stream of tokens.<br>
/// The scanner does not perform any validation on its input. It picks out recognised tokens and flag the unknown tokens as invalid.
pub struct Scanner<'a> {
    /// The result syntactic tokens.
    pub tokens: Vec<Token<'a>>,
    /// The result comment tokens,
    pub comments: Vec<Token<'a>>,
    /// The characters in the input text.
    text: Vec<char>,
    /// The current position in the text stream while scanning.
    index: usize,
    /// Indicates that the scanning has reached the end of the character stream.
    end: bool,
    /// The current char.
    char: char,
    /// A duo of numbers that mark the position of the scanner in the input text, by line and by column
    pos: [u64; 2],
    span: [[u64; 2]; 2],
    errors: HashMap<TextSpan, &'static str>,
}

fn is_identifier_char(char: char) -> bool {
    char == '$' || char == '_' || char.is_alphanumeric()
}

impl<'a> Scanner<'a> {
    /// Creates a new scanner.
    pub fn new(input: &'a str) -> Self {
        Scanner {
            tokens: Vec::new(),
            comments: Vec::new(),
            text: input.chars().collect(),
            index: 0,
            end: false,
            char: '\0',
            pos: [1, 1],
            span: [[0, 0], [0, 0]],
            errors: HashMap::new(),
        }
    }
    // Advances to the next character in the stream.
    fn next(&mut self) {
        self.index += 1;
        if self.index < self.text.len() {
            self.char = self.text[self.index];
            self.shift();
        } else {
            self.char = '\0';
            self.end = true;
        }
    }
    /// Next by a length.
    fn next_by(&mut self, i: usize) {
        for _ in 0..i {
            self.next();
        }
    }
    /// Tracks the position of each character by line and column. Useful in getting the range and location of each token.
    fn shift(&mut self) {
        if self.char == '\n' {
            self.pos[0] += 1;
            self.pos[1] = 0;
        } else {
            self.pos[1] += 1;
        }
    }
    /// Marks the current line and column as the start of a token.
    fn mark_start(&mut self) {
        self.span[0] = self.pos.clone();
    }
    /// Marks the current line and column as the end of a token.
    fn mark_end(&mut self) {
        self.span[1] = self.pos.clone();
    }
    fn error(&mut self, code: i32) {
        self.errors.insert(self.span, Error::scanner_error(code));
    }
    /// Checks if the current character is a bracket.
    fn is_bracket(&mut self) -> bool {
        matches!(self.char, '{' | '}' | '(' | ')' | '[' | ']')
    }
    /// Checks that the next stream of tokens match a scan rule.
    fn sees(&self, rule: &str) -> bool {
        let front = self.index + rule.len();
        if front <= self.text.len() {
            let actual: String = self.text[self.index..front].iter().collect();
            rule.chars().nth(0) == actual.chars().nth(0) && rule == actual.as_str()
        } else {
            false
        }
    }
    /// Checks that the next stream of tokens match a scan rule.
    fn matches(&mut self, rule: &str) -> bool {
        if self.sees(rule) {
            match self.text.get(self.index + rule.len()) {
                None => true,
                Some(v) => !is_identifier_char(*v),
            }
        } else {
            false
        }
    }
}

impl<'a> Scanner<'a> {
    /// Entry point to the scanner.
    pub fn run(&mut self) {
        self.char = *self.text.get(0).unwrap_or_else(|| &'\0');
        if self.char != '\0' {
            loop {
                while self.char == ' ' {
                    self.next()
                }
                let token = self.scan_next();
                if token.is_comment() {
                    self.comments.push(token)
                } else {
                    self.tokens.push(token);
                }
                if self.end {
                    break;
                }
            }
        }
    }
    /// Scans for the next token in the stream.
    fn scan_next(&mut self) -> Token<'a> {
        if self.sees("//") {
            self.line_comment()
        } else if self.sees("/*") {
            self.block_comment()
        } else if self.sees("##") {
            self.doc_comment()
        } else if self.char == '"' {
            self.string()
        } else if self.char == '\'' {
            self.character()
        } else if self.char.is_digit(10) {
            self.number()
        } else if self.matches("new") {
            self.operator("new")
        } else if let Some(op) = OPERATORS.iter().find(|op| self.sees(op)) {
            self.operator(op)
        } else if self.char == '@' {
            self.injunction()
        } else if self.is_bracket() {
            self.bracket()
        } else if self.char == ';' {
            self.semi_colon()
        } else if is_identifier_char(self.char) {
            self.identifier_or_keyword()
        } else {
            println!(r#"No production for {}"#, self.char);
            todo!()
        }
    }
    fn line_comment(&mut self) -> Token<'a> {
        self.mark_start();
        self.next_by(2);
        let mut content = String::new();
        while !(self.end || self.char == '\n') {
            content.push(self.char);
            self.next();
        }
        self.mark_end();
        Token::create_line_comment(content, self.span.clone())
    }
    fn block_comment(&mut self) -> Token<'a> {
        self.mark_start();
        self.next_by(2);
        let mut content = String::new();
        while !(self.end || self.sees("*/")) {
            content.push(self.char);
            self.next();
        }
        self.next_by(2);
        self.mark_end();
        Token::create_block_comment(content, self.span.clone())
    }
    fn doc_comment(&mut self) -> Token<'a> {
        self.mark_start();
        self.next_by(2);
        let mut content = String::new();
        while !(self.end || self.char == '\n') {
            content.push(self.char);
            self.next();
        }
        self.mark_end();
        Token::create_doc_comment(content, self.span.clone())
    }
    fn string(&mut self) -> Token<'a> {
        self.mark_start();
        self.next();
        let mut value = String::new();
        while !(self.end || self.char == '"') {
            if self.sees("\\\\") {
                value.push_str("\\\\");
                self.next_by(2);
            } else if self.sees("\\\"") {
                value.push_str("\\\"");
                self.next_by(2);
            } else {
                value.push(self.char);
                self.next()
            }
        }
        self.mark_end();
        if self.end {
            self.error(1);
        } else {
            self.next();
        }
        Token::create_literal("string", value, self.span.clone())
    }
    fn number(&mut self) -> Token<'a> {
        self.mark_start();
        let mut value = String::new();
        if self.sees("0x") {
            value.push_str("0x");
            self.next_by(2);
            while self.char.is_digit(16) {
                value.push(self.char);
                self.next();
            }
        } else if self.sees("0b") {
            value.push_str("0b");
            self.next_by(2);
            while self.char.is_digit(2) {
                value.push(self.char);
                self.next();
            }
        } else if self.sees("0o") {
            value.push_str("0o");
            self.next_by(2);
            while self.char.is_digit(8) {
                value.push(self.char);
                self.next();
            }
        } else {
            while self.char.is_digit(10) {
                value.push(self.char);
                self.next();
            }
            if self.char == '.' {
                value.push(self.char);
                self.next();
                while self.char.is_digit(10) {
                    value.push(self.char);
                    self.next();
                }
            }
        }
        value.push_str(self.exponent().as_str());
        self.mark_end();
        Token::create_literal("number", value, self.span.clone())
    }
    fn exponent(&mut self) -> String {
        let mut exponential = String::new();
        if self.char == 'e' {
            loop {
                exponential.push(self.char);
                self.next();
                if !self.char.is_digit(10) {
                    break;
                }
            }
        }
        exponential
    }
    fn operator(&mut self, op: &str) -> Token<'a> {
        self.mark_start();
        self.next_by(op.len());
        self.mark_end();
        Token::create_operator(op, self.span.clone())
    }
    fn injunction(&mut self) -> Token<'a> {
        self.mark_start();
        self.next();
        let mut value = String::new();
        while is_identifier_char(self.char) {
            value.push(self.char);
            self.next();
        }
        self.mark_end();
        Token::create_injunction(&value, self.span.clone())
    }
    fn identifier_or_keyword(&mut self) -> Token<'a> {
        self.mark_start();
        let mut value = String::new();
        while is_identifier_char(self.char) {
            value.push(self.char);
            self.next();
        }
        self.mark_end();
        if KEYWORDS.contains(&value.as_str()) {
            Token::create_keyword(value, self.span.clone())
        } else {
            Token::create_identifier(value, self.span.clone())
        }
    }
    fn character(&mut self) -> Token<'a> {
        self.mark_start();
        self.next();
        let mut value = String::new();
        while !(self.end || self.char == '\'') {
            if self.sees("\\\\") {
                value.push_str("\\\\");
                self.next_by(2);
            } else if self.sees("\\\'") {
                value.push_str("\\\'");
                self.next_by(2);
            } else {
                value.push(self.char);
                self.next()
            }
        }
        self.mark_end();
        self.next();
        Token::create_literal("character", value, self.span.clone())
    }
    fn bracket(&mut self) -> Token<'a> {
        self.mark_start();
        let char = self.char;
        self.next();
        self.mark_end();
        Token::create_bracket(&char, self.span.clone())
    }
    fn semi_colon(&mut self) -> Token<'a> {
        self.mark_start();
        self.next();
        self.mark_end();
        Token::create_semi_colon(self.span.clone())
    }
    fn comma(&mut self) -> Token<'a> {
        self.mark_start();
        self.next();
        self.mark_end();
        Token::create_comma(self.span.clone())
    }
}
