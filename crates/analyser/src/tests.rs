#![cfg(test)]

use std::cell::RefCell;

use crate::{Resolver, Symbol, SymbolType};
use ast::Operator;
use ast::TextSpan;
use errors::SemanticError;
use parser::{Parser, Provider, Scanner};
use utils::Stage;

#[test]
fn it_tests_number_plus_number() {
    let mut scanner = Scanner::new("2 + 2;");
    scanner.run();
    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    let resolver = Resolver::new(&parser);
    resolver.resolve().unwrap();
    assert_eq!(resolver.diagnostics, RefCell::new(vec![]))
}

#[test]
fn it_tests_number_plus_string() {
    let mut scanner = Scanner::new("2 + \"hello\";");
    scanner.run();
    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    let resolver = Resolver::new(&parser);
    resolver.resolve().unwrap();
    assert_eq!(
        resolver.diagnostics.take(),
        vec![(
            SemanticError::UnsupportedBinaryOperation(
                Operator::Add,
                SymbolType::Number,
                SymbolType::String
            ),
            [[1, 1], [1, 5]]
        )]
    )
}

#[test]
fn it_tests_numeric_identifier_plus_string() {
    let mut scanner = Scanner::new(
        "
    @let a = 2; 
    a + \"hello\";",
    );
    scanner.run();
    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    let resolver = Resolver::new(&parser);
    resolver.resolve().unwrap();
    assert_eq!(
        resolver.diagnostics.take(),
        vec![(
            SemanticError::UnsupportedBinaryOperation(
                Operator::Add,
                SymbolType::Number,
                SymbolType::String
            ),
            [[1, 15], [2, 9]]
        )]
    )
}

#[test]
fn it_tests_identifiers() {
    let mut scanner = Scanner::new(
        "
    @let a = 2;
    @let b = 4;
    @let c = a + b;
    @let d = c;
    d + '\\n';",
    );
    scanner.run();
    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    let resolver = Resolver::new(&parser);
    resolver.resolve().unwrap();
    assert_eq!(
        resolver.diagnostics.take(),
        vec![(
            SemanticError::UnsupportedBinaryOperation(
                Operator::Add,
                SymbolType::Number,
                SymbolType::Character
            ),
            [[1, 15], [5, 9]]
        )]
    )
}

#[test]
fn it_tests_identifiers_across_blocks() {
    let mut scanner = Scanner::new(
        "
    @let a = 2;
    {
        @let b = 4 + a;
        @let c = true;
        println b + c;
    }
    println b;
    println a;",
    );
    scanner.run();
    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    let resolver = Resolver::new(&parser);
    resolver.resolve().unwrap();
    assert_eq!(
        resolver.diagnostics.take(),
        vec![
            (
                SemanticError::UnsupportedBinaryOperation(
                    Operator::Add,
                    SymbolType::Number,
                    SymbolType::Boolean
                ),
                [[3, 18], [4, 18]]
            ),
            (
                SemanticError::Undeclared(String::from("b")),
                [[7, 13], [7, 14]]
            )
        ]
    )
}

#[test]
fn it_faults_heterogenous_arrays() {
    let mut scanner = Scanner::new(
        "
    @let array = [1, 2, 3, \"hello\"];",
    );
    scanner.run();
    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    let resolver = Resolver::new(&parser);
    resolver.resolve().unwrap();
    assert_eq!(
        resolver.diagnostics.take(),
        vec![(
            SemanticError::HeterogenousArray(SymbolType::Number, SymbolType::String),
            [[1, 29], [1, 35]]
        )]
    )
}

#[test]
fn it_fault_inconsistent_ternaries() {
    let mut scanner = Scanner::new(
        "
    @let isTrue = true;
    @let array = isTrue ? 345: \"John Doe\";",
    );
    scanner.run();
    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    let resolver = Resolver::new(&parser);
    resolver.resolve().unwrap();
    assert_eq!(
        resolver.diagnostics.take(),
        vec![(
            SemanticError::InconsistentTernarySides(SymbolType::Number, SymbolType::String),
            [[2, 32], [2, 41]]
        )]
    )
}

#[test]
fn it_faults_invalid_index() {
    let mut scanner = Scanner::new(
        "
    @let numArray = [1, 2, 3];
    @let num = numArray['x'];",
    );
    scanner.run();
    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    let resolver = Resolver::new(&parser);
    resolver.resolve().unwrap();
    assert_eq!(
        resolver.diagnostics.take(),
        vec![(
            SemanticError::InvalidIndexer(SymbolType::Character),
            [[2, 25], [2, 27]]
        )]
    )
}

#[test]
fn it_faults_invalid_range_boundary() {
    let mut scanner = Scanner::new(
        "
    @let halftruth = true..false;",
    );
    scanner.run();
    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    let resolver = Resolver::new(&parser);
    resolver.resolve().unwrap();
    assert_eq!(
        resolver.diagnostics.take(),
        vec![(SemanticError::InvalidRangeBoundaries, [[1, 23], [1, 34]])]
    )
}

#[test]
fn it_faults_invalid_assignment() {
    let mut scanner = Scanner::new(
        "
    @let day = \"Monday\";
    @let time = 1600;
    day = time;",
    );
    scanner.run();
    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    let resolver = Resolver::new(&parser);
    resolver.resolve().unwrap();
    assert_eq!(
        resolver.diagnostics.take(),
        vec![(
            SemanticError::InconsistentAssignment(SymbolType::String, SymbolType::Number),
            [[3, 5], [3, 15]]
        )]
    )
}

#[test]
fn it_fault_nested_test_block() {
    let mut scanner = Scanner::new(
        "
    {
        @tests {
            
        }
    }",
    );
    scanner.run();
    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    let resolver = Resolver::new(&parser);
    resolver.resolve().unwrap();
    assert_eq!(
        resolver.diagnostics.take(),
        vec![(SemanticError::IllegalTestBlock, [[2, 9], [5, 0]])]
    )
}

#[test]
fn it_remembers_type_aliases() {
    let mut scanner = Scanner::new(
        "
    @type Metres = Number;
    @let distance1: Metres = 900;
    {
        @type Kilometres = Metres;
        @let distance2: Kilometres = 400;
        @let distance = distance1 + distance2;
    }
    ",
    );
    scanner.run();
    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    let resolver = Resolver::new(&parser);
    resolver.resolve().unwrap();
    assert_eq!(resolver.diagnostics.take(), vec![])
}

#[test]
fn it_tests_boolean_or_number() {
    let mut scanner = Scanner::new("true || 9;");
    scanner.run();
    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    let resolver = Resolver::new(&parser);
    resolver.resolve().unwrap();
    assert_eq!(
        resolver.diagnostics,
        RefCell::new(vec![(
            SemanticError::UnsupportedLogicalOperation(
                Operator::LogicalOr,
                SymbolType::Boolean,
                SymbolType::Number
            ),
            [[1, 1], [1, 9]]
        )])
    )
}

const DEFAULT_SPAN: TextSpan = [[0, 0], [0, 0]];
#[test]
fn it_stores_variables_and_symbols() {
    let mut stage = Stage::new();
    stage.set(
        "name",
        Symbol {
            _type: SymbolType::Boolean,
            span: DEFAULT_SPAN,
        },
    );
    assert_eq!(
        stage.get("name"),
        Some(&Symbol {
            _type: SymbolType::Boolean,
            span: DEFAULT_SPAN,
        }),
    );
}

#[test]
fn it_retrieves_outer_variable_from_inner_scope() {
    let mut stage = Stage::new();
    stage.set(
        "age",
        Symbol {
            _type: SymbolType::Number,
            span: DEFAULT_SPAN,
        },
    );
    stage.create_inner();

    assert_eq!(
        stage.lookup("age"),
        Some(&Symbol {
            _type: SymbolType::Number,
            span: DEFAULT_SPAN,
        }),
    );
}

#[test]
fn it_retrieves_outer_variable_from_nested_inner_scope() {
    let mut stage = Stage::new();
    stage.set(
        "age",
        Symbol {
            _type: SymbolType::Number,
            span: DEFAULT_SPAN,
        },
    );
    stage.create_inner();

    stage.set(
        "dateCreated",
        Symbol {
            _type: SymbolType::String,
            span: DEFAULT_SPAN,
        },
    );

    stage.create_inner();

    stage.create_inner();

    assert_eq!(
        stage.lookup("age"),
        Some(&Symbol {
            _type: SymbolType::Number,
            span: DEFAULT_SPAN,
        }),
    );

    assert_eq!(
        stage.lookup("dateCreated"),
        Some(&Symbol {
            _type: SymbolType::String,
            span: DEFAULT_SPAN,
        }),
    );
}
