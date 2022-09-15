#![cfg(test)]

use ast::Operator;
use errors::SemanticError;
use parser::{Parser, Provider, Scanner};

use crate::checker::{Checker, Type};

#[test]
fn it_validates_number_plus_number() {
    let mut scanner = Scanner::new("2 + 2;");
    scanner.run();
    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    let errors = Checker::check(parser.statements.take());
    assert_eq!(errors.len(), 0)
}

#[test]
fn it_faults_number_plus_string() {
    let mut scanner = Scanner::new("2 + \"hello\";");
    scanner.run();
    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    let errors = Checker::check(parser.statements.take());
    assert_eq!(
        errors,
        vec![(
            SemanticError::UnsupportedBinaryOperation(
                Operator::Add,
                Type::number(),
                Type::string(),
            ),
            [[1, 1], [1, 11]]
        )]
    )
}
