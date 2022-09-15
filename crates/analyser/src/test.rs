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
    assert_eq!(parser.diagnostics.take().len(), 0);
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
    assert_eq!(parser.diagnostics.take().len(), 0);
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

#[test]
fn it_remembers_declared_values() {
    let mut scanner = Scanner::new(
        "
     @let name = \"Sefunmi\";
     println name;
     ",
    );
    scanner.run();
    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    assert_eq!(parser.diagnostics.take().len(), 0);
    let errors = Checker::check(parser.statements.take());
    assert_eq!(errors.len(), 0)
}

#[test]
fn it_faults_undeclared_values() {
    let mut scanner = Scanner::new(
        "
     println name;
     ",
    );
    scanner.run();
    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    assert_eq!(parser.diagnostics.take().len(), 0);
    let errors = Checker::check(parser.statements.take());
    assert_eq!(
        errors,
        vec![(
            SemanticError::Undeclared(String::from("name")),
            [[1, 15], [1, 19]]
        )]
    )
}

#[test]
fn it_faults_use_of_uninitialized_values() {
    let mut scanner = Scanner::new(
        "
    @let name: String;
    println name;
     ",
    );
    scanner.run();
    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    assert_eq!(parser.diagnostics.take().len(), 0);
    let errors = Checker::check(parser.statements.take());
    assert_eq!(
        errors,
        vec![(
            SemanticError::Uninitialized(String::from("name")),
            [[2, 13], [2, 17]]
        )]
    )
}

#[test]
fn it_remembers_primitives() {
    let mut scanner = Scanner::new(
        "
    @let name: String = \"akomolafe\";
    println name;
     ",
    );
    scanner.run();
    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    assert_eq!(parser.diagnostics.take().len(), 0);
    let errors = Checker::check(parser.statements.take());
    assert_eq!(errors.len(), 0)
}

#[test]
fn it_matches_type_to_value() {
    let mut scanner = Scanner::new(
        "
    @let no_of_bugs: Number = 190;
    @let money_lost = no_of_bugs * 14000;
    @let days_to_getting_sacked: Number = money_lost / 10;
     ",
    );
    scanner.run();
    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    assert_eq!(parser.diagnostics.take().len(), 0);
    let errors = Checker::check(parser.statements.take());
    assert_eq!(errors.len(), 0)
}

#[test]
fn it_faults_inconsistent_declaration() {
    let mut scanner = Scanner::new(
        "
    @let is_valid: String = true;
     ",
    );
    scanner.run();
    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    assert_eq!(parser.diagnostics.take().len(), 0);
    let errors = Checker::check(parser.statements.take());
    assert_eq!(
        errors,
        vec![(
            SemanticError::InconsistentAssignment(Type::string(), Type::boolean(),),
            [[1, 6], [1, 34]]
        )]
    )
}

#[test]
fn it_faults_uninferable_declaration() {
    let mut scanner = Scanner::new(
        "
    @let is_valid: Number = true + 900;
     ",
    );
    scanner.run();
    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    assert_eq!(parser.diagnostics.take().len(), 0);
    let errors = Checker::check(parser.statements.take());
    assert_eq!(
        errors,
        vec![
            (
                SemanticError::UnsupportedBinaryOperation(
                    Operator::Add,
                    Type::boolean(),
                    Type::number(),
                ),
                [[1, 30], [1, 40]]
            ),
            (
                SemanticError::InconsistentAssignment(Type::number(), Type::Uninferable),
                [[1, 6], [1, 40]]
            )
        ]
    )
}

#[test]
fn it_validates_logical_operation() {
    let mut scanner = Scanner::new(
        "
    @let age_is_valid: Boolean = true;
    @let account_is_valid = true;
    @let is_allowed_entry: Boolean = account_is_valid && age_is_valid;
     ",
    );
    scanner.run();
    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    assert_eq!(parser.diagnostics.take().len(), 0);
    let errors = Checker::check(parser.statements.take());
    assert_eq!(errors.len(), 0)
}

#[test]
fn it_validates_array_expression() {
    let mut scanner = Scanner::new(
        "
    @let array: Array<String> = [\"This\", \"is\", \"an\", \"array\"];
    @let arrayRef = array;
    @let arrayRef2: Array<String> = array;
    @let array2 = [1, 3, 4, 5, 6];
    @let a: Array<Number> = array2;
     ",
    );
    scanner.run();
    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    assert_eq!(parser.diagnostics.take().len(), 0);
    let errors = Checker::check(parser.statements.take());
    assert_eq!(errors.len(), 0)
}
