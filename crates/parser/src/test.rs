#![cfg(test)]

use crate::scanner::Scanner;
use ast::{Comment, CommentKind, Injunction, Keyword, Literal, LiteralKind, Token, TokenKind};

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
                value: String::from("This is a\\\" string.")
            })
        }
    )
}

#[test]
fn it_scans_whole_numbers() {
    let mut scanner = Scanner::new("89");
    scanner.run();
    assert_eq!(
        scanner.tokens[0],
        Token {
            span: [[1, 1], [1, 2]],
            kind: TokenKind::Literal(Literal {
                value: String::from("89"),
                kind: LiteralKind::NumericLiteral
            })
        }
    )
}

#[test]
fn it_scans_floating_numbers() {
    let mut scanner = Scanner::new("9.9999");
    scanner.run();
    assert_eq!(
        scanner.tokens[0],
        Token {
            span: [[1, 1], [1, 6]],
            kind: TokenKind::Literal(Literal {
                kind: LiteralKind::NumericLiteral,
                value: String::from("9.9999")
            })
        }
    )
}

#[test]
fn it_scans_hexadecimal_numbers() {
    let mut scanner = Scanner::new("0x90384");
    scanner.run();
    assert_eq!(
        scanner.tokens[0],
        Token {
            span: [[1, 1], [1, 7]],
            kind: TokenKind::Literal(Literal {
                value: String::from("0x90384"),
                kind: LiteralKind::NumericLiteral,
            })
        }
    )
}

#[test]
fn it_scans_binary_numbers() {
    let mut scanner = Scanner::new("0b10100");
    scanner.run();
    assert_eq!(
        scanner.tokens[0],
        Token {
            span: [[1, 1], [1, 7]],
            kind: TokenKind::Literal(Literal {
                value: String::from("0b10100"),
                kind: LiteralKind::NumericLiteral,
            })
        }
    )
}

#[test]
fn it_scans_octal_numbers() {
    let mut scanner = Scanner::new("0o75266");
    scanner.run();
    assert_eq!(
        scanner.tokens[0],
        Token {
            span: [[1, 1], [1, 7]],
            kind: TokenKind::Literal(Literal {
                value: String::from("0o75266"),
                kind: LiteralKind::NumericLiteral,
            })
        }
    )
}
#[test]
fn it_scans_octal_exponents() {
    let mut scanner = Scanner::new("0o75266e2");
    scanner.run();
    assert_eq!(
        scanner.tokens[0],
        Token {
            span: [[1, 1], [1, 9]],
            kind: TokenKind::Literal(Literal {
                value: String::from("0o75266e2"),
                kind: LiteralKind::NumericLiteral,
            })
        }
    )
}

#[test]
fn it_scans_injunction() {
    let mut scanner = Scanner::new("@public");
    scanner.run();
    assert_eq!(
        scanner.tokens[0],
        Token {
            span: [[1, 1], [1, 7]],
            kind: TokenKind::Keyword(Keyword::Injunction(Injunction::Public))
        }
    )
}

#[test]
fn it_scans_characters() {
    let mut scanner = Scanner::new("'h'");
    scanner.run();
    assert_eq!(
        scanner.tokens[0],
        Token {
            span: [[1, 1], [1, 3]],
            kind: TokenKind::Literal(Literal {
                value: String::from("h"),
                kind: LiteralKind::CharacterLiteral
            })
        }
    );
}
