#![cfg(test)]

use std::vec;

use crate::{parser::Parser, scanner::Scanner};
use ast::{
    BooleanExpr, BracketKind, Comment, CommentKind, Expression, ExpressionStatement, Identifier,
    Injunction, Keyword, Literal, LiteralKind, Node, NodeData, NumericExpr, Operator, Punctuation,
    Statement, StringExpr, Token, TokenKind,
};

#[test]
fn it_scans_line_comment() {
    let mut scanner = Scanner::new("// This is a comment.");
    scanner.run();
    assert_eq!(
        scanner.comments[0],
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
        scanner.comments[0],
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
    let mut scanner = Scanner::new("## This is a doc comment.");
    scanner.run();
    assert_eq!(
        scanner.comments[0],
        Token {
            span: [[1, 1], [1, 25]],
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

#[test]
fn it_scans_brackets() {
    let mut scanner = Scanner::new("{}[]()");
    scanner.run();
    assert_eq!(
        scanner.tokens,
        vec![
            Token {
                span: [[1, 1], [1, 2]],
                kind: TokenKind::Punctuation(Punctuation::Bracket(BracketKind::LeftCurly))
            },
            Token {
                span: [[1, 2], [1, 3]],
                kind: TokenKind::Punctuation(Punctuation::Bracket(BracketKind::RightCurly))
            },
            Token {
                span: [[1, 3], [1, 4]],
                kind: TokenKind::Punctuation(Punctuation::Bracket(BracketKind::LeftSquare))
            },
            Token {
                span: [[1, 4], [1, 5]],
                kind: TokenKind::Punctuation(Punctuation::Bracket(BracketKind::RightSquare))
            },
            Token {
                span: [[1, 5], [1, 6]],
                kind: TokenKind::Punctuation(Punctuation::Bracket(BracketKind::LeftParenthesis))
            },
            Token {
                span: [[1, 6], [1, 6]],
                kind: TokenKind::Punctuation(Punctuation::Bracket(BracketKind::RightParenthesis))
            },
            Token {
                span: [[1, 6], [1, 6]],
                kind: TokenKind::EOF
            }
        ]
    )
}

#[test]
fn it_scans_identifiers_and_keywords() {
    let mut scanner = Scanner::new("name in word");
    scanner.run();
    assert_eq!(
        scanner.tokens,
        vec![
            Token {
                span: [[1, 1], [1, 5]],
                kind: TokenKind::Identifier(Identifier {
                    value: String::from("name")
                })
            },
            Token {
                span: [[1, 6], [1, 8]],
                kind: TokenKind::Keyword(Keyword::In)
            },
            Token {
                span: [[1, 9], [1, 12]],
                kind: TokenKind::Identifier(Identifier {
                    value: String::from("word")
                })
            },
            Token {
                span: [[1, 9], [1, 12]],
                kind: TokenKind::EOF
            }
        ]
    );
}

#[test]
fn it_scans_operators() {
    let mut scanner = Scanner::new("name++");
    scanner.run();
    assert_eq!(
        scanner.tokens,
        vec![
            Token {
                span: [[1, 1], [1, 5]],
                kind: TokenKind::Identifier(Identifier {
                    value: String::from("name")
                })
            },
            Token {
                span: [[1, 5], [1, 6]],
                kind: TokenKind::Operator(Operator::Increment)
            },
            Token {
                span: [[1, 5], [1, 6]],
                kind: TokenKind::EOF
            }
        ]
    )
}

#[test]
fn it_scans_operators_2() {
    let mut scanner = Scanner::new("2+4+new Number()");
    scanner.run();
    assert_eq!(
        scanner.tokens,
        vec![
            Token {
                span: [[1, 1], [1, 2]],
                kind: TokenKind::Literal(Literal {
                    kind: LiteralKind::NumericLiteral,
                    value: String::from("2")
                })
            },
            Token {
                span: [[1, 2], [1, 3]],
                kind: TokenKind::Operator(Operator::Add)
            },
            Token {
                span: [[1, 3], [1, 4]],
                kind: TokenKind::Literal(Literal {
                    kind: LiteralKind::NumericLiteral,
                    value: String::from("4")
                })
            },
            Token {
                span: [[1, 4], [1, 5]],
                kind: TokenKind::Operator(Operator::Add)
            },
            Token {
                span: [[1, 5], [1, 8]],
                kind: TokenKind::Operator(Operator::New)
            },
            Token {
                span: [[1, 9], [1, 15]],
                kind: TokenKind::Identifier(Identifier {
                    value: String::from("Number")
                })
            },
            Token {
                span: [[1, 15], [1, 16]],
                kind: TokenKind::Punctuation(Punctuation::Bracket(BracketKind::LeftParenthesis))
            },
            Token {
                span: [[1, 16], [1, 16]],
                kind: TokenKind::Punctuation(Punctuation::Bracket(BracketKind::RightParenthesis))
            },
            Token {
                span: [[1, 16], [1, 16]],
                kind: TokenKind::EOF
            }
        ]
    )
}

#[test]
fn it_scans_unknown_token() {
    let mut scanner = Scanner::new("`");
    scanner.run();
    assert_eq!(
        scanner.tokens,
        vec![
            Token {
                span: [[1, 1], [1, 1]],
                kind: TokenKind::Invalid(String::from("`"))
            },
            Token {
                span: [[1, 1], [1, 1]],
                kind: TokenKind::EOF
            }
        ],
    )
}

#[test]
fn it_parses_numeric_literal() {
    assert_eq!(
        Parser::from_scanner(Scanner::new("2;")).statements[0],
        Node {
            range: [[1, 1], [1, 2]],
            data: NodeData::Statement {
                kind: Statement::ExpressionStatement(ExpressionStatement {
                    expression: Box::new(Node {
                        range: [[1, 1], [1, 2]],
                        data: NodeData::Expression {
                            kind: Expression::NumericExpr(NumericExpr {
                                value: String::from("2")
                            })
                        }
                    })
                })
            }
        }
    );
}

#[test]
fn it_parses_string_literal() {
    assert_eq!(
        Parser::from_scanner(Scanner::new("\"Hello, world!\";")).statements[0],
        Node {
            range: [[1, 1], [1, 16]],
            data: NodeData::Statement {
                kind: Statement::ExpressionStatement(ExpressionStatement {
                    expression: Box::new(Node {
                        range: [[1, 1], [1, 15]],
                        data: NodeData::Expression {
                            kind: Expression::StringExpr(StringExpr {
                                value: String::from("Hello, world!")
                            })
                        }
                    })
                })
            }
        }
    );
}

#[test]
fn it_parses_boolean_literal() {
    assert_eq!(
        Parser::from_scanner(Scanner::new("true;")).statements[0],
        Node {
            range: [[1, 1], [1, 5]],
            data: NodeData::Statement {
                kind: Statement::ExpressionStatement(ExpressionStatement {
                    expression: Box::new(Node {
                        range: [[1, 1], [1, 5]],
                        data: NodeData::Expression {
                            kind: Expression::BooleanExpr(BooleanExpr { value: true })
                        }
                    })
                })
            }
        }
    );
}
