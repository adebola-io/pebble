#![cfg(test)]

use std::{cell::RefCell, marker::PhantomData, vec};

use crate::{
    parser::{Parser, Provider},
    scanner::Scanner,
};
use ast::{
    ArrayExpression, Block, BracketKind, Break, CallExpression, Comment, CommentKind, ConcreteType,
    CrashStatement, Expression, FnExpression, Function, Identifier, IfStatement, Import,
    Injunction, Keyword, Literal, LiteralKind, Loop, Operator, Parameter, PrependStatement,
    PrintLnStatement, Punctuation, RecoverBlock, ReturnStatement, Statement, TestBlock, TextString,
    Token, TokenIdentifier, TokenKind, TryBlock, Type, UseImport, VarKind, VariableDeclaration,
    WhileStatement,
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
                kind: TokenKind::Identifier(TokenIdentifier {
                    value: String::from("name")
                })
            },
            Token {
                span: [[1, 6], [1, 8]],
                kind: TokenKind::Keyword(Keyword::In)
            },
            Token {
                span: [[1, 9], [1, 12]],
                kind: TokenKind::Identifier(TokenIdentifier {
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
                kind: TokenKind::Identifier(TokenIdentifier {
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
                kind: TokenKind::Identifier(TokenIdentifier {
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
        Statement::create_expr_stmnt(Expression::create_num_expr("2", [[1, 1], [1, 2]]))
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
        Statement::create_expr_stmnt(Expression::create_bool_expr("true", [[1, 1], [1, 5]]))
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
        Statement::create_expr_stmnt(Expression::create_str_expr(
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
        Statement::create_expr_stmnt(Expression::create_bin_expr(
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
        Statement::create_expr_stmnt(Expression::create_bin_expr(
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
        Statement::create_expr_stmnt(Expression::create_bin_expr(
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
        Statement::create_expr_stmnt(Expression::create_ident_expr(
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
        Statement::create_expr_stmnt(Expression::create_call_expr(
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
        Statement::create_expr_stmnt(Expression::create_call_expr(
            Expression::create_ident_expr("doStuff", [[1, 1], [1, 8]]),
            vec![
                Expression::create_ident_expr("argument1", [[1, 9], [1, 18]]),
                Expression::create_ident_expr("argument2", [[1, 20], [1, 29]])
            ],
            [1, 30]
        ))
    )
}

#[test]
fn it_parses_dot_expression() {
    let mut scanner = Scanner::new("object.property;");
    scanner.run();
    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    let statements = parser.statements.borrow().clone();
    assert_eq!(
        statements[0],
        Statement::create_expr_stmnt(Expression::create_dot_expr(
            Expression::create_ident_expr("object", [[1, 1], [1, 7]]),
            Expression::create_ident_expr("property", [[1, 8], [1, 16]])
        ))
    )
}

#[test]
fn it_parses_namespace_expression() {
    let mut scanner = Scanner::new("object::property;");
    scanner.run();
    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    let statements = parser.statements.borrow().clone();
    assert_eq!(
        statements[0],
        Statement::create_expr_stmnt(Expression::create_namespace_expr(
            Expression::create_ident_expr("object", [[1, 1], [1, 7]]),
            Expression::create_ident_expr("property", [[1, 9], [1, 17]])
        ))
    )
}

#[test]
fn it_parses_index_expression() {
    let mut scanner = Scanner::new("accessor[property];");
    scanner.run();
    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    let statements = parser.statements.borrow().clone();
    assert_eq!(
        statements[0],
        Statement::create_expr_stmnt(Expression::create_index_expr(
            Expression::create_ident_expr("accessor", [[1, 1], [1, 9]]),
            Expression::create_ident_expr("property", [[1, 10], [1, 18]]),
            [1, 19]
        ))
    )
}

#[test]
fn it_parses_complex_expression_1() {
    let mut scanner = Scanner::new("doStuff(arg1 + arg2).with().anArray[index];");
    scanner.run();
    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    let statements = parser.statements.borrow().clone();
    assert_eq!(
        statements[0],
        Statement::create_expr_stmnt(Expression::create_index_expr(
            Expression::create_dot_expr(
                Expression::create_call_expr(
                    Expression::create_dot_expr(
                        Expression::create_call_expr(
                            Expression::create_ident_expr("doStuff", [[1, 1], [1, 8]]),
                            vec![Expression::create_bin_expr(
                                Expression::create_ident_expr("arg1", [[1, 9], [1, 13]]),
                                &Operator::Add,
                                Expression::create_ident_expr("arg2", [[1, 16], [1, 20]])
                            )],
                            [1, 21]
                        ),
                        Expression::create_ident_expr("with", [[1, 22], [1, 26]])
                    ),
                    vec![],
                    [1, 28]
                ),
                Expression::create_ident_expr("anArray", [[1, 29], [1, 36]])
            ),
            Expression::create_ident_expr("index", [[1, 37], [1, 42]]),
            [1, 43]
        ))
    )
}

#[test]
fn it_parses_unary_expression() {
    let mut scanner = Scanner::new("-2 + 3;");
    scanner.run();
    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    let statements = parser.statements.borrow().clone();
    assert_eq!(
        statements[0],
        Statement::create_expr_stmnt(Expression::create_bin_expr(
            Expression::create_unary_expr(
                [1, 1],
                &Operator::Subtract,
                Expression::create_num_expr("2", [[1, 2], [1, 3]])
            ),
            &Operator::Add,
            Expression::create_num_expr("3", [[1, 6], [1, 7]])
        ))
    )
}

#[test]
fn it_parses_range_expression() {
    let mut scanner = Scanner::new("psrandom(2..9);");
    scanner.run();
    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    let statements = parser.statements.borrow().clone();
    assert_eq!(
        statements[0],
        Statement::create_expr_stmnt(Expression::create_call_expr(
            Expression::create_ident_expr("psrandom", [[1, 1], [1, 9]]),
            vec![Expression::create_range_expr(
                Expression::create_num_expr("2", [[1, 10], [1, 11]]),
                Expression::create_num_expr("9", [[1, 13], [1, 14]])
            )],
            [1, 15]
        ))
    )
}

#[test]
fn it_parses_logical_expression() {
    let mut scanner = Scanner::new("is_false || is_true;");
    scanner.run();
    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    let statements = parser.statements.borrow().clone();
    assert_eq!(
        statements[0],
        Statement::create_expr_stmnt(Expression::create_logical_expr(
            Expression::create_ident_expr("is_false", [[1, 1], [1, 9]]),
            &Operator::LogicalOr,
            Expression::create_ident_expr("is_true", [[1, 13], [1, 20]])
        ))
    )
}

#[test]
fn it_parses_ternary_expression() {
    let mut scanner = Scanner::new("is_true ? doStuff() : doOtherStuff();");
    scanner.run();
    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    let statements = parser.statements.borrow().clone();
    assert_eq!(
        statements[0],
        Statement::create_expr_stmnt(Expression::create_ternary_expr(
            Expression::create_ident_expr("is_true", [[1, 1], [1, 8]]),
            Expression::create_call_expr(
                Expression::create_ident_expr("doStuff", [[1, 11], [1, 18]]),
                vec![],
                [1, 20]
            ),
            Expression::create_call_expr(
                Expression::create_ident_expr("doOtherStuff", [[1, 23], [1, 35]]),
                vec![],
                [1, 37]
            )
        ))
    )
}

#[test]
fn it_parses_nested_ternary_expression() {
    let mut scanner = Scanner::new(
        "age >= 21 ? serveDrink() : age > 18 ? collectBribe().then(serveDrink) : reject();",
    );
    scanner.run();
    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    let statements = parser.statements.borrow().clone();
    assert_eq!(
        statements[0],
        Statement::create_expr_stmnt(Expression::create_ternary_expr(
            Expression::create_bin_expr(
                Expression::create_ident_expr("age", [[1, 1], [1, 4]]),
                &Operator::GreaterThanOrEquals,
                Expression::create_num_expr("21", [[1, 8], [1, 10]])
            ),
            Expression::create_call_expr(
                Expression::create_ident_expr("serveDrink", [[1, 13], [1, 23]]),
                vec![],
                [1, 25]
            ),
            Expression::create_ternary_expr(
                Expression::create_bin_expr(
                    Expression::create_ident_expr("age", [[1, 28], [1, 31]]),
                    &Operator::GreaterThan,
                    Expression::create_num_expr("18", [[1, 34], [1, 36]])
                ),
                Expression::create_call_expr(
                    Expression::create_dot_expr(
                        Expression::create_call_expr(
                            Expression::create_ident_expr("collectBribe", [[1, 39], [1, 51]]),
                            vec![],
                            [1, 53]
                        ),
                        Expression::create_ident_expr("then", [[1, 54], [1, 58]])
                    ),
                    vec![Expression::create_ident_expr(
                        "serveDrink",
                        [[1, 59], [1, 69]]
                    )],
                    [1, 70]
                ),
                Expression::create_call_expr(
                    Expression::create_ident_expr("reject", [[1, 73], [1, 79]]),
                    vec![],
                    [1, 81]
                )
            )
        ))
    )
}

#[test]
fn it_parses_assignment_expression() {
    let mut scanner = Scanner::new("name = \"sefunmi\";");
    scanner.run();
    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    let statements = parser.statements.borrow().clone();
    assert_eq!(
        statements[0],
        Statement::create_expr_stmnt(Expression::create_assign_expr(
            Expression::create_ident_expr("name", [[1, 1], [1, 5]]),
            &Operator::Assign,
            Expression::create_str_expr("sefunmi", [[1, 8], [1, 16]])
        ))
    );
}

#[test]
fn it_parses_nested_assignment_expression() {
    let mut scanner = Scanner::new("variable += value = value2;");
    scanner.run();
    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    let statements = parser.statements.borrow().clone();
    assert_eq!(
        statements[0],
        Statement::create_expr_stmnt(Expression::create_assign_expr(
            Expression::create_ident_expr("variable", [[1, 1], [1, 9]]),
            &Operator::AddAssign,
            Expression::create_assign_expr(
                Expression::create_ident_expr("value", [[1, 13], [1, 18]]),
                &Operator::Assign,
                Expression::create_ident_expr("value2", [[1, 21], [1, 27]])
            )
        ))
    )
}

#[test]
fn it_parses_grouped_expression() {
    let mut scanner = Scanner::new("( 2 + 2 ) * 8;");
    scanner.run();
    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    let statements = parser.statements.borrow().clone();
    assert_eq!(
        statements[0],
        Statement::create_expr_stmnt(Expression::create_bin_expr(
            Expression::create_bin_expr(
                Expression::create_num_expr("2", [[1, 3], [1, 4]]),
                &Operator::Add,
                Expression::create_num_expr("2", [[1, 7], [1, 8]])
            ),
            &Operator::Multiply,
            Expression::create_num_expr("8", [[1, 13], [1, 14]])
        ))
    )
}

#[test]
fn it_parses_if_statement() {
    let mut scanner = Scanner::new(
        "
    if (condition) { 
        doStuff();
    } else {
        doOtherStuff();
    }",
    );
    scanner.run();
    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    let statements = parser.statements.borrow().clone();
    assert_eq!(
        statements[0],
        Statement::IfStatement(IfStatement {
            test: Expression::create_ident_expr("condition", [[1, 10], [1, 19]]),
            body: Box::new(Statement::BlockStatement(Block {
                body: vec![Statement::create_expr_stmnt(Expression::create_call_expr(
                    Expression::create_ident_expr("doStuff", [[2, 9], [2, 16]]),
                    vec![],
                    [2, 18]
                ))],
                span: [[1, 21], [3, 6]]
            })),
            alternate: Some(Box::new(Statement::BlockStatement(Block {
                body: vec![Statement::create_expr_stmnt(Expression::create_call_expr(
                    Expression::create_ident_expr("doOtherStuff", [[4, 9], [4, 21]]),
                    vec![],
                    [4, 23]
                ))],
                span: [[3, 12], [5, 5]]
            }))),
            span: [[1, 6], [5, 5]]
        })
    )
}

#[test]
fn it_parses_if_statement_without_block() {
    let mut scanner = Scanner::new("if (condition) doStuff(); else doOtherStuff();");
    scanner.run();
    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    let statements = parser.statements.borrow().clone();
    assert_eq!(
        statements[0],
        Statement::IfStatement(IfStatement {
            test: Expression::create_ident_expr("condition", [[1, 5], [1, 14]]),
            body: Box::new(Statement::create_expr_stmnt(Expression::create_call_expr(
                Expression::create_ident_expr("doStuff", [[1, 16], [1, 23]]),
                vec![],
                [1, 25]
            ))),
            alternate: Some(Box::new(Statement::create_expr_stmnt(
                Expression::create_call_expr(
                    Expression::create_ident_expr("doOtherStuff", [[1, 32], [1, 44]]),
                    vec![],
                    [1, 46]
                )
            ))),
            span: [[1, 1], [1, 46]]
        })
    )
}

#[test]
fn it_parses_if_statement_without_else() {
    let mut scanner = Scanner::new("if (condition) doStuff();");
    scanner.run();
    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    let statements = parser.statements.borrow().clone();
    assert_eq!(
        statements[0],
        Statement::IfStatement(IfStatement {
            test: Expression::create_ident_expr("condition", [[1, 5], [1, 14]]),
            body: Box::new(Statement::create_expr_stmnt(Expression::create_call_expr(
                Expression::create_ident_expr("doStuff", [[1, 16], [1, 23]]),
                vec![],
                [1, 25]
            ))),
            alternate: None,
            span: [[1, 1], [1, 25]]
        })
    )
}

#[test]
fn it_parses_print_statement() {
    let mut scanner = Scanner::new("println \"Hello, world!\";");
    scanner.run();
    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    let statements = parser.statements.borrow().clone();
    assert_eq!(
        statements[0],
        Statement::PrintLnStatement(PrintLnStatement {
            argument: Expression::create_str_expr("Hello, world!", [[1, 9], [1, 23]]),
            span: [[1, 1], [1, 24]]
        })
    )
}

#[test]
fn it_parses_prepend_statement() {
    let mut scanner = Scanner::new("@prepend \"./example.peb\";");
    scanner.run();
    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    let statements = parser.statements.borrow().clone();
    assert_eq!(
        statements[0],
        Statement::PrependStatement(PrependStatement {
            source: Expression::create_str_expr("./example.peb", [[1, 10], [1, 24]]),
            span: [[1, 1], [1, 25]]
        })
    )
}

#[test]
fn it_parses_test_block() {
    let mut scanner = Scanner::new("@tests {}");
    scanner.run();
    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    let statements = parser.statements.borrow().clone();
    assert_eq!(
        statements[0],
        Statement::TestBlock(TestBlock {
            body: Block {
                body: vec![],
                span: [[1, 8], [1, 9]]
            },
            span: [[1, 1], [1, 9]]
        })
    )
}

#[test]
fn it_parses_while_statement() {
    let mut scanner = Scanner::new("while (is_true) doStuff();");
    scanner.run();
    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    let statements = parser.statements.borrow().clone();
    assert_eq!(
        statements[0],
        Statement::WhileStatement(WhileStatement {
            test: Expression::create_ident_expr("is_true", [[1, 8], [1, 15]]),
            body: Box::new(Statement::create_expr_stmnt(Expression::create_call_expr(
                Expression::create_ident_expr("doStuff", [[1, 17], [1, 24]]),
                vec![],
                [1, 26]
            ))),
            span: [[1, 1], [1, 26]]
        })
    )
}

#[test]
fn it_parses_return_statement() {
    let mut scanner = Scanner::new("return \"Hello, world!\";");
    scanner.run();
    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    let statements = parser.statements.borrow().clone();
    assert_eq!(
        statements[0],
        Statement::ReturnStatement(ReturnStatement {
            argument: Some(Expression::create_str_expr(
                "Hello, world!",
                [[1, 8], [1, 22]]
            )),
            span: [[1, 1], [1, 23]]
        })
    )
}

#[test]
fn it_parses_return_statement_without_argument() {
    let mut scanner = Scanner::new("return;");
    scanner.run();
    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    let statements = parser.statements.borrow().clone();
    assert_eq!(
        statements[0],
        Statement::ReturnStatement(ReturnStatement {
            argument: None,
            span: [[1, 1], [1, 7]]
        })
    )
}

#[test]
fn it_parses_loop_statement() {
    let mut scanner = Scanner::new("loop (3) { doStuff(); }");
    scanner.run();
    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    let statements = parser.statements.borrow().clone();
    assert_eq!(
        statements[0],
        Statement::LoopStmnt(Loop {
            constraint: Some(Expression::create_num_expr("3", [[1, 7], [1, 8]])),
            body: Block {
                body: vec![Statement::create_expr_stmnt(Expression::create_call_expr(
                    Expression::create_ident_expr("doStuff", [[1, 12], [1, 19]]),
                    vec![],
                    [1, 21]
                ))],
                span: [[1, 10], [1, 23]]
            },
            span: [[1, 1], [1, 23]]
        })
    )
}

#[test]
fn it_parses_break_statement() {
    let mut scanner = Scanner::new("break;");
    scanner.run();
    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    let statements = parser.statements.borrow().clone();
    assert_eq!(
        statements[0],
        Statement::Break(Break {
            span: [[1, 1], [1, 6]],
            phantom: PhantomData
        })
    )
}

#[test]
fn it_parses_crash_statement() {
    let mut scanner = Scanner::new("crash Error(\"This is an error.\");");
    scanner.run();
    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    let statements = parser.statements.borrow().clone();
    assert_eq!(
        statements[0],
        Statement::CrashStmnt(CrashStatement {
            argument: Expression::create_call_expr(
                Expression::create_ident_expr("Error", [[1, 7], [1, 12]]),
                vec![Expression::create_str_expr(
                    "This is an error.",
                    [[1, 13], [1, 31]]
                )],
                [1, 33]
            ),
            span: [[1, 1], [1, 33]]
        })
    )
}

#[test]
fn it_parses_try_recover_block() {
    let mut scanner = Scanner::new("try {} recover (e) {}");
    scanner.run();
    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    let statements = parser.statements.borrow().clone();
    assert_eq!(
        statements[0],
        Statement::TryBlock(TryBlock {
            body: Block {
                body: vec![],
                span: [[1, 5], [1, 7]]
            },
            recover: Some(RecoverBlock {
                params: vec![Parameter {
                    label: None,
                    name: Identifier {
                        value: "e",
                        span: [[1, 17], [1, 18]]
                    },
                    span: [[1, 17], [1, 18]]
                }],
                span: [[1, 8], [1, 21]],
                body: Block {
                    body: vec![],
                    span: [[1, 20], [1, 21]]
                }
            }),
            span: [[1, 1], [1, 21]],
        })
    )
}

#[test]
fn it_parses_use_import() {
    let mut scanner = Scanner::new("@use {querySelector, createElement as createDocumentElement, * as document} from \"pile:document\";");
    scanner.run();
    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    let statements = parser.statements.borrow().clone();
    assert_eq!(
        statements[0],
        Statement::UseImport(UseImport {
            imports: vec![
                Import {
                    collapsed_import: false,
                    imported_name: Identifier {
                        value: "querySelector",
                        span: [[1, 7], [1, 20]]
                    },
                    local_name: None,
                    span: [[1, 7], [1, 20]]
                },
                Import {
                    collapsed_import: false,
                    imported_name: Identifier {
                        value: "createElement",
                        span: [[1, 22], [1, 35]]
                    },
                    local_name: Some(Identifier {
                        value: "createDocumentElement",
                        span: [[1, 39], [1, 60]]
                    }),
                    span: [[1, 22], [1, 60]]
                },
                Import {
                    collapsed_import: true,
                    imported_name: Identifier {
                        value: "*",
                        span: [[1, 62], [1, 63]]
                    },
                    local_name: Some(Identifier {
                        value: "document",
                        span: [[1, 67], [1, 75]]
                    }),
                    span: [[1, 62], [1, 75]]
                }
            ],
            source: TextString {
                value: "pile:document",
                span: [[1, 82], [1, 96]]
            },
            span: [[1, 1], [1, 97]]
        })
    )
}

#[test]
fn it_parses_plain_function() {
    let mut scanner = Scanner::new(
        " 
    @function add (x: Number, y: Number) -> Number {
        return x + y;
    }",
    );
    scanner.run();
    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    let statements = parser.statements.borrow().clone();
    assert_eq!(
        statements[0],
        Statement::Function(Function {
            name: Identifier {
                value: "add",
                span: [[2, 15], [2, 18]]
            },
            generic_arguments: None,
            parameters: vec![
                Parameter {
                    name: Identifier {
                        value: "x",
                        span: [[2, 20], [2, 21]]
                    },
                    label: Some(Type::Concrete(ConcreteType {
                        name: Identifier {
                            value: "Number",
                            span: [[2, 23], [2, 29]]
                        },
                        arguments: vec![],

                        span: [[2, 23], [2, 29]]
                    })),
                    span: [[2, 20], [2, 30]]
                },
                Parameter {
                    name: Identifier {
                        value: "y",
                        span: [[2, 31], [2, 32]]
                    },
                    label: Some(Type::Concrete(ConcreteType {
                        name: Identifier {
                            value: "Number",
                            span: [[2, 34], [2, 40]]
                        },
                        arguments: vec![],
                        span: [[2, 34], [2, 40]]
                    })),
                    span: [[2, 31], [2, 40]]
                }
            ],
            return_type: Some(Type::Concrete(ConcreteType {
                name: Identifier {
                    value: "Number",
                    span: [[2, 45], [2, 51]]
                },
                arguments: vec![],
                span: [[2, 45], [2, 51]]
            })),
            body: Block {
                body: vec![Statement::ReturnStatement(ReturnStatement {
                    argument: Some(Expression::create_bin_expr(
                        Expression::create_ident_expr("x", [[3, 16], [3, 17]]),
                        &Operator::Add,
                        Expression::create_ident_expr("y", [[3, 20], [3, 21]]),
                    )),
                    span: [[3, 9], [4, 0]]
                })],
                span: [[2, 52], [4, 5]]
            },
            span: [[2, 5], [4, 5]]
        })
    )
}

#[test]
fn it_parses_functional_expression() {
    let mut scanner = Scanner::new("map(fn (item) item * 2 );");
    scanner.run();
    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    let statements = parser.statements.borrow().clone();
    assert_eq!(
        statements[0],
        Statement::create_expr_stmnt(Expression::CallExpression(CallExpression {
            callee: Box::new(Expression::create_ident_expr("map", [[1, 1], [1, 4]])),
            arguments: vec![Expression::FnExpression(FnExpression {
                labels: None,
                parameters: vec![Parameter {
                    name: Identifier {
                        value: "item",
                        span: [[1, 9], [1, 13]]
                    },
                    label: None,
                    span: [[1, 9], [1, 13]]
                }],
                return_type: None,
                body: None,
                implicit_return: Some(Box::new(Expression::create_bin_expr(
                    Expression::create_ident_expr("item", [[1, 15], [1, 19]]),
                    &Operator::Multiply,
                    Expression::create_num_expr("2", [[1, 22], [1, 23]])
                ),)),
                span: [[1, 5], [1, 23]]
            })],
            span: [[1, 1], [1, 25]]
        }))
    )
}

#[test]
fn it_parses_let_statement() {
    let mut scanner = Scanner::new("@let name: String = \"Akomolafe\";");
    scanner.run();
    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    let statements = parser.statements.borrow().clone();
    assert_eq!(
        statements[0],
        Statement::VariableDeclaration(VariableDeclaration {
            name: Identifier {
                value: "name",
                span: [[1, 6], [1, 10]]
            },
            kind: VarKind::Let,
            initializer: Some(Expression::StringExpression(TextString {
                value: "Akomolafe",
                span: [[1, 21], [1, 31]]
            })),
            type_label: Some(Type::Concrete(ConcreteType {
                name: Identifier {
                    value: "String",
                    span: [[1, 12], [1, 18]]
                },
                arguments: vec![],
                span: [[1, 12], [1, 18]]
            })),
            span: [[1, 1], [1, 31]]
        })
    )
}

#[test]
fn it_parses_const_statement() {
    let mut scanner =
        Scanner::new("@const NAMES: ArrayList<String> = [\"Akomolafe\", \"Sefunmi\"];");
    scanner.run();
    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    let statements = parser.statements.borrow().clone();
    assert_eq!(
        statements[0],
        Statement::VariableDeclaration(VariableDeclaration {
            name: Identifier {
                value: "NAMES",
                span: [[1, 8], [1, 13]]
            },
            kind: VarKind::Const,
            initializer: Some(Expression::ArrayExpression(ArrayExpression {
                elements: vec![
                    Expression::StringExpression(TextString {
                        value: "Akomolafe",
                        span: [[1, 36], [1, 46]]
                    }),
                    Expression::StringExpression(TextString {
                        value: "Sefunmi",
                        span: [[1, 49], [1, 57]]
                    })
                ],
                span: [[1, 35], [1, 59]]
            })),
            type_label: Some(Type::Concrete(ConcreteType {
                name: Identifier {
                    value: "ArrayList",
                    span: [[1, 15], [1, 24]]
                },
                arguments: vec![Type::Concrete(ConcreteType {
                    name: Identifier {
                        value: "String",
                        span: [[1, 25], [1, 31]]
                    },
                    arguments: vec![],
                    span: [[1, 25], [1, 31]]
                }),],
                span: [[1, 15], [1, 32]]
            })),

            span: [[1, 1], [1, 59]]
        })
    )
}

#[test]
fn misc_test_1() {
    let mut scanner =
        Scanner::new("
    ## A test function that checks that two expressions are equal to one another.
    ## The function will initiate a crash if the two expressions are not equal.
    @public @function isEqual<T implements Equatable + Display>(left: T, right: T) {
        if (left != right) {
            crash AssertionError(1, \"Right does not equal left.\");
        }
    }
    ## A test function that asserts that two expressions are not equal to one another.
    ## The function will initiate a crash if the two expressions are equal.
    @public @function isNotEqual<T implements Equatable + Display>(left: T, right: T) {
        if (left != right) {
            crash AssertionError(2, \"Right equals left.\");
        }
    }
    ## A test function that asserts than an expression is greather than another.
    ## The function will initiate a crash if the left expression is not greater than the right.
    @public @function isGreater<T>(left: T, right: T) {
       if (left <= right) {
            crash AssertionError(3, \"Left is not greater than right.\");
       }
    }
    ## A test function that asserts that a condition is true and initiates an assertion error otherwise.
    @public @function isTrue (condition: Boolean) {
        if (condition) {
            crash AssertionError(3, \"Assertion failed because condition is false.\");
        }
    }");
    scanner.run();
    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    assert_eq!(parser.diagnostics, RefCell::new(vec![]))
}

#[test]
fn it_parses_type_alias() {
    let mut scanner = Scanner::new("@type HashMap<T> = Global.HashMap<T, U>;");
    scanner.run();
    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    assert_eq!(parser.diagnostics, RefCell::new(vec![]))
}

#[test]
fn it_parses_interface() {
    let mut scanner = Scanner::new(
        "
    @interface Display {
        toString: () -> String,
        @implement Regularity
    }
    ",
    );
    scanner.run();
    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    assert_eq!(parser.diagnostics, RefCell::new(vec![]))
}

#[test]
fn it_parses_class() {
    let mut scanner = Scanner::new(
        "
    @class Receiver {
        Receiver(port: UnsignedInt) {
            self.port = port;
            self.messages = [];
        },
        port: UnsignedInt,
        messages: Array<Messages>
    }   
    ",
    );
    scanner.run();
    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    assert_eq!(parser.diagnostics, RefCell::new(vec![]))
}
