use ast::{Node, Token, TokenKind};

use crate::scanner::Scanner;

/// The recursive descent parser that goes over the array of tokens scanned from the source text, and iteratively builds out the AST syntax tree.
pub struct Parser<'a> {
    scanner: Scanner,
    token: Option<&'a Token>,
    pub ast: Node,
}

impl<'a> Parser<'a> {
    pub fn new(scanner: Scanner) -> Self {
        Parser {
            scanner,
            token: None,
            ast: Node::program(),
        }
    }
}
