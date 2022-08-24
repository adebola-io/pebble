#![cfg(test)]

use crate::{
    parser::{Parser, Provider},
    scanner::Scanner,
};
use ast::{
    BracketKind, Comment, CommentKind, Expression, Identifier, Injunction, Keyword, Literal,
    LiteralKind, Operator, Punctuation, Statement, Token, TokenKind,
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
    let mut scanner = Scanner::new("2+4+ Number()");
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
                span: [[1, 6], [1, 12]],
                kind: TokenKind::Identifier(Identifier {
                    value: String::from("Number")
                })
            },
            Token {
                span: [[1, 12], [1, 13]],
                kind: TokenKind::Punctuation(Punctuation::Bracket(BracketKind::LeftParenthesis))
            },
            Token {
                span: [[1, 13], [1, 13]],
                kind: TokenKind::Punctuation(Punctuation::Bracket(BracketKind::RightParenthesis))
            },
            Token {
                span: [[1, 13], [1, 13]],
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
    let mut scanner = Scanner::new("2;");
    scanner.run();
    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    let statements = parser.statements.borrow().clone();
    assert_eq!(
        statements[0],
        Statement::create_expression_statement(Expression::create_num_expr("2", [[1, 1], [1, 2]]))
    )
}

#[test]
fn it_parses_boolean_literal() {
    let mut scanner = Scanner::new("true;");
    scanner.run();
    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    let statements = parser.statements.borrow().clone();
    assert_eq!(
        statements[0],
        Statement::create_expression_statement(Expression::create_bool_expr(
            "true",
            [[1, 1], [1, 5]]
        ))
    )
}

#[test]
fn it_parses_string_literal() {
    let mut scanner = Scanner::new("\"This is a string.\";");
    scanner.run();
    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    let statements = parser.statements.borrow().clone();
    assert_eq!(
        statements[0],
        Statement::create_expression_statement(Expression::create_str_expr(
            "This is a string.",
            [[1, 1], [1, 19]]
        ))
    )
}

#[test]
fn it_parses_binary_expression() {
    let mut scanner = Scanner::new("2+2;");
    scanner.run();
    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    let statements = parser.statements.borrow().clone();
    assert_eq!(
        statements[0],
        Statement::create_expression_statement(Expression::create_bin_expr(
            Expression::create_num_expr("2", [[1, 1], [1, 2]]),
            &Operator::Add,
            Expression::create_num_expr("2", [[1, 3], [1, 4]])
        ))
    )
}

#[test]
fn it_parses_continuous_binary_expression() {
    let mut scanner = Scanner::new("2 + 2 + 2;");
    scanner.run();
    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    let statements = parser.statements.borrow().clone();
    assert_eq!(
        statements[0],
        Statement::create_expression_statement(Expression::create_bin_expr(
            Expression::create_bin_expr(
                Expression::create_num_expr("2", [[1, 1], [1, 2]]),
                &Operator::Add,
                Expression::create_num_expr("2", [[1, 5], [1, 6]])
            ),
            &Operator::Add,
            Expression::create_num_expr("2", [[1, 9], [1, 10]])
        ))
    )
}

#[test]
fn it_parses_binary_expression_with_multiple_operands() {
    let mut scanner = Scanner::new("2 + 4 * 5;");
    scanner.run();
    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    let statements = parser.statements.borrow().clone();
    assert_eq!(
        statements[0],
        Statement::create_expression_statement(Expression::create_bin_expr(
            Expression::create_num_expr("2", [[1, 1], [1, 2]]),
            &Operator::Add,
            Expression::create_bin_expr(
                Expression::create_num_expr("4", [[1, 5], [1, 6]]),
                &Operator::Multiply,
                Expression::create_num_expr("5", [[1, 9], [1, 10]])
            ),
        ))
    )
}

#[test]
fn it_parses_identifier() {
    let mut scanner = Scanner::new("identifier_1;");
    scanner.run();
    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    let statements = parser.statements.borrow().clone();
    assert_eq!(
        statements[0],
        Statement::create_expression_statement(Expression::create_ident_expr(
            "identifier_1",
            [[1, 1], [1, 13]]
        ))
    )
}

#[test]
fn it_parses_call_expression() {
    let mut scanner = Scanner::new("doStuff();");
    scanner.run();
    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    let statements = parser.statements.borrow().clone();
    assert_eq!(
        statements[0],
        Statement::create_expression_statement(Expression::create_call_expr(
            Expression::create_ident_expr("doStuff", [[1, 1], [1, 8]]),
            vec![],
            [1, 10]
        ))
    )
}

#[test]
fn it_parses_call_expression_with_arguments() {
    let mut scanner = Scanner::new("doStuff(argument1, argument2);");
    scanner.run();
    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    let statements = parser.statements.borrow().clone();
    assert_eq!(
        statements[0],
        Statement::create_expression_statement(Expression::create_call_expr(
            Expression::create_ident_expr("doStuff", [[1, 1], [1, 8]]),
            vec![
                Expression::create_ident_expr("argument1", [[1, 9], [1, 18]]),
                Expression::create_ident_expr("argument2", [[1, 20], [1, 29]])
            ],
            [1, 30]
        ))
    )
}
