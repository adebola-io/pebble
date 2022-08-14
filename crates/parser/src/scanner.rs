use ast::Token;

/// A state machine that goes over the input text and scans it into a stream of tokens.<br>
/// The scanner does not perform any validation on its input. It picks out recognised tokens and flag the unknown tokens as invalid.
pub struct Scanner<'a> {
    /// The result tokens.
    pub tokens: Vec<Token<'a>>,
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
}

impl<'a> Scanner<'a> {
    /// Creates a new scanner.
    pub fn new(input: &'a str) -> Self {
        Scanner {
            tokens: Vec::new(),
            text: input.chars().collect(),
            index: 0,
            end: false,
            char: '\0',
            pos: [1, 1],
            span: [[0, 0], [0, 0]],
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
    fn mark_end(&mut self) {
        self.span[1] = self.pos.clone();
    }
    /// Checks that the next stream of tokens match a scan rule.
    pub fn sees(&mut self, rule: &str) -> bool {
        let front = self.index + rule.len();
        if front <= self.text.len() {
            let actual: String = self.text[self.index..front].iter().collect();
            rule == actual.as_str()
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
                let token = self.scan_next();
                self.tokens.push(token);
                if self.end {
                    break;
                }
            }
        }
    }
    /// Scans for the next token in the stream.
    fn scan_next(&mut self) -> Token<'a> {
        if self.sees("//") {
            self.scan_line_comment()
        } else if self.sees("/*") {
            self.scan_block_comment()
        } else {
            todo!()
        }
    }
    fn scan_line_comment(&mut self) -> Token<'a> {
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
    fn scan_block_comment(&mut self) -> Token<'a> {
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
}
