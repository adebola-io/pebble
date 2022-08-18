use crate::scanner::Scanner;
use ast::{Node, Token, TokenKind};

type NodeOrError = Result<Node, (&'static str, [u64; 2])>;
/// The recursive descent parser that goes over the array of tokens scanned from the source text, and iteratively builds out the AST syntax tree.
pub struct Parser {
    scanner: Scanner,
    token: Token,
    statements: Vec<Node>,
    index: usize,
}

impl Parser {
    pub fn next(&mut self) {
        self.index += 1;
        if self.index < self.scanner.tokens.len() {
            self.token = self.scanner.tokens[self.index].clone()
        }
    }
}

impl Parser {
    pub fn from_scanner(mut scanner: Scanner) {
        scanner.run();
        let mut parser = Parser {
            token: scanner.tokens[0].clone(),
            scanner,
            statements: vec![],
            index: 0,
        };
        while parser.token.is_eof() {
            let parsed = parser.statement();
            match parsed {
                Ok(statement) => parser.statements.push(statement),
                Err(_) => todo!(),
            }
        }
    }
    fn statement(&mut self) -> NodeOrError {
        match self.token.kind {
            TokenKind::Keyword(_) => todo!(),
            TokenKind::Identifier(_) => self.expression_statement(),
            _ => todo!(),
        }
    }
    fn expression_statement(&mut self) -> NodeOrError {
        let start = self.token.span;
        let expression = self.expression()?;
        if self.token.is_semi_colon() {
            self.next();
            let end = self.token.span;
            todo!()
        } else {
            Err(("Expected a semicolon here.", self.token.span[1]))
        }
    }
    fn expression(&mut self) -> NodeOrError {
        match self.token.kind {
            TokenKind::Literal(_) => self.literal(),
            _ => todo!(),
        }
    }
    fn literal(&mut self) -> NodeOrError {
        todo!()
    }
}
