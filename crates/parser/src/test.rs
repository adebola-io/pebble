#![cfg(test)]

use crate::scanner::Scanner;
use ast::{Comment, CommentKind, Literal, LiteralKind, Token, TokenKind};

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

#[test]
fn it_scans_block_comment() {
    let mut scanner = Scanner::new("/* This\nis\na\nblock\ncomment */");
    scanner.run();
    assert_eq!(
        scanner.tokens[0],
        Token {
            kind: TokenKind::Comment(Comment {
                kind: CommentKind::Block,
                content: String::from(" This\nis\na\nblock\ncomment ")
            }),
            span: [[1, 1], [5, 10]]
        }
    )
}

#[test]
fn it_scans_doc_comments() {
    let mut scanner = Scanner::new("# This is a doc comment.");
    scanner.run();
    assert_eq!(
        scanner.tokens[0],
        Token {
            span: [[1, 1], [1, 24]],
            kind: TokenKind::Comment(Comment {
                kind: CommentKind::Doc,
                content: String::from(" This is a doc comment.")
            })
        }
    )
}

#[test]
fn it_scans_strings() {
    let mut scanner = Scanner::new("\"This is a string.\"");
    scanner.run();
    assert_eq!(
        scanner.tokens[0],
        Token {
            span: [[1, 1], [1, 19]],
            kind: TokenKind::Literal(Literal {
                kind: LiteralKind::StringLiteral,
                value: String::from("This is a string.")
            })
        }
    )
}

#[test]
fn it_scans_string_with_escape() {
    let mut scanner = Scanner::new("\"This is a\\\" string.\"");
    scanner.run();
    assert_eq!(
        scanner.tokens[0],
        Token {
            span: [[1, 1], [1, 21]],
            kind: TokenKind::Literal(Literal {
                kind: LiteralKind::StringLiteral,
                value: String::from("This is a\" string.")
            })
        }
    )
}
