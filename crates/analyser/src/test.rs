#![cfg(test)]

use ast::Operator;
use errors::SemanticError;
use parser::{Parser, Provider, Scanner};

use crate::checker::{Type, TypeChecker};

#[test]
fn it_validates_number_plus_number() {
    let mut scanner = Scanner::new("2 + 2;");
    scanner.run();
    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    assert_eq!(parser.diagnostics.take().len(), 0);
    let errors = TypeChecker::check(parser.statements.take());
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
    let errors = TypeChecker::check(parser.statements.take());
    assert_eq!(
        errors,
        vec![(
            SemanticError::UnsupportedOperation(Operator::Add, Type::number(), Type::string(),),
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
    let errors = TypeChecker::check(parser.statements.take());
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
    let errors = TypeChecker::check(parser.statements.take());
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
    let errors = TypeChecker::check(parser.statements.take());
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
    let errors = TypeChecker::check(parser.statements.take());
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
    let errors = TypeChecker::check(parser.statements.take());
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
    let errors = TypeChecker::check(parser.statements.take());
    assert_eq!(
        errors,
        vec![(
            SemanticError::Unassignable(Type::string(), Type::boolean(),),
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
    let errors = TypeChecker::check(parser.statements.take());
    assert_eq!(
        errors,
        vec![
            (
                SemanticError::UnsupportedOperation(Operator::Add, Type::boolean(), Type::number(),),
                [[1, 30], [1, 40]]
            ),
            (
                SemanticError::Unassignable(Type::number(), Type::Uninferrable),
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
    let errors = TypeChecker::check(parser.statements.take());
    assert_eq!(errors.len(), 0)
}

#[test]
fn it_validates_array_expression() {
    let mut scanner = Scanner::new(
        "
    @let array: Array<String> = [\"This\", \"is\", \"an\", \"array\"];
    @let array_ref = array;
    @let array_ref2: Array<String> = array;
    @let array2 = [1, 3, 4, 5, 6];
    @let a: Array<Number> = array2;
    @let array3: Array<String> = [];
     ",
    );
    scanner.run();
    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    assert_eq!(parser.diagnostics.take().len(), 0);
    let errors = TypeChecker::check(parser.statements.take());
    assert_eq!(errors, vec![])
}

#[test]
fn it_validates_index_expression() {
    let mut scanner = Scanner::new(
        "
    @let array: Array<String> = [\"This\", \"is\", \"an\", \"array\"];
    @let element = array[0];
    @let element_ref: String = element;

    @let array2 = [[1,1], [0, 0], [0, 0], [2, 3]];
    @let element2: Number = array2[1][2];
     ",
    );
    scanner.run();
    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    assert_eq!(parser.diagnostics.take().len(), 0);
    let errors = TypeChecker::check(parser.statements.take());
    assert_eq!(errors, vec![])
}

#[test]
fn it_validates_ternary_expression() {
    let mut scanner = Scanner::new(
        "
    @let is_ready = false;
    @let loaded: String = is_ready ? \"Loading successful.\" : \"Loading...\";
     ",
    );
    scanner.run();
    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    assert_eq!(parser.diagnostics.take().len(), 0);
    let errors = TypeChecker::check(parser.statements.take());
    assert_eq!(errors, vec![])
}

#[test]
fn it_faults_invalid_ternary_expression() {
    let mut scanner = Scanner::new(
        "
    @let loader = 9 ? 4 : true;
     ",
    );
    scanner.run();
    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    assert_eq!(parser.diagnostics.take().len(), 0);
    let errors = TypeChecker::check(parser.statements.take());
    assert_eq!(
        errors,
        vec![
            (
                SemanticError::InvalidTernaryTest(Type::number()),
                [[1, 20], [1, 21]]
            ),
            (
                SemanticError::InconsistentTernarySides(Type::number(), Type::boolean()),
                [[1, 20], [1, 32]]
            ),
            (
                SemanticError::Unassignable(Type::Unknown, Type::Uninferrable),
                [[1, 6], [1, 32]]
            )
        ]
    )
}

#[test]
fn it_validates_range_boundaries() {
    let mut scanner = Scanner::new(
        "
    @let decimal = 3..4;
    @let decimal_array: Array<Number> = [decimal];
    @let decimal_number: Number = decimal_array[1];

    @let char: Character = 'a'..'z'; 
     ",
    );
    scanner.run();
    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    assert_eq!(parser.diagnostics.take().len(), 0);
    let errors = TypeChecker::check(parser.statements.take());
    assert_eq!(errors, vec![])
}

#[test]
fn it_validates_unary_expression() {
    let mut scanner = Scanner::new(
        "
    @let boolean_value = !true;
    @let boolean_value_2: Boolean = !!boolean_value;
    
    @let numeric_value = 101;
    @let num_value_2 = ~numeric_value;
     ",
    );
    scanner.run();
    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    assert_eq!(parser.diagnostics.take().len(), 0);
    let errors = TypeChecker::check(parser.statements.take());
    assert_eq!(errors, vec![])
}

#[test]
fn it_validates_assignment_expression() {
    let mut scanner = Scanner::new(
        "
    @let empty_variable: String;
    empty_variable = \"hello, \";

    @let s: String = empty_variable;
    s += \"world! \";

    @let empty_array = [];
    empty_array = [1, 2, 3];
    empty_array = [];
    empty_array = [true, false];
    @let a: Array<Number> = empty_array;
     ",
    );
    scanner.run();
    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    assert_eq!(parser.diagnostics.take().len(), 0);
    let errors = TypeChecker::check(parser.statements.take());
    assert_eq!(errors, vec![])
}
