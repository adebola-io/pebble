#![cfg(test)]

use crate::scanner::Scanner;
use ast::{Comment, CommentKind, Token, TokenKind};

#[test]
fn it_scans_line_comment() {
    let mut scanner = Scanner::new("// This is a comment.");
    scanner.run();
    assert_eq!(
        scanner.tokens[0],
        Token {
            kind: TokenKind::Comment(Comment {
                kind: CommentKind::Line,
                content: String::from(" This is a comment.")
            }),
            span: [[1, 1], [1, 21]]
        }
    )
}
