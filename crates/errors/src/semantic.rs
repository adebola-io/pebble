use std::fmt::Display;

use ast::Operator;

#[derive(Debug, PartialEq)]
pub enum SemanticError<T>
where
    T: Display,
{
    UnsupportedBinaryOperation(Operator, T, T),
    UnsupportedLogicalOperation(Operator, T, T),
    ComparisionBetweenDifferentTypes(Operator, T, T),
    Undeclared(String),
    AlreadyDeclared(String),
    InvalidTernaryTest(T),
    InvalidIndex(T),
    InvalidIndexer(T),
    InconsistentTernarySides(T, T),
    UnsupportedNegation(T),
    InconsistentAssignment(T, T),
    InconsistentInitializer,
    InvalidRangeBoundaries,
    UnknownAssignment,
    UnusedVariable,
    IllegalTestBlock,
    AliasUsedAsValue(String),
    ValueUsedAsAlias(String),
    AssigningToNil,
    HeterogenousArray(T, T),
}

impl<T> Display for SemanticError<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.",
            match self {

                SemanticError::UnsupportedBinaryOperation(op, x, y) => format!(
                    "The operation '{}' is not defined for types '{}' and '{}'",
                    op, x, y
                ),
                SemanticError::UnsupportedLogicalOperation(op, x, y) => format!(
                    "Cannot perform the logical operation '{}' on types '{}' and '{}'",
                    op, x, y
                ),
                SemanticError::ComparisionBetweenDifferentTypes(op, x, y) => format!(
                    "This operation will always return {} because there is no overlap between the types '{}' and '{}'",
                    match op {
                        Operator::Equals => "false",
                        Operator::NotEquals=> "true",
                        _ => unreachable!()
                    }
                        , x, y
                    ),
                SemanticError::InvalidTernaryTest(x) => format!(
                    "Expected Boolean type for ternary operation, got '{}'", x
                ),
                SemanticError::InconsistentTernarySides(x, y) => format!(
                    "Expected '{}' for alternate expression, got '{}'. Both sides of a ternary expression must have the same type", x, y
                ),
                SemanticError::InvalidIndex(x) => format!("The type '{}' is not an indexable type", x),
                SemanticError::InvalidIndexer(x) => format!("The type '{}' cannot be used as an index", x),
                SemanticError::InvalidRangeBoundaries => format!("Invalid range. The boundaries of a range must be both be either characters or numbers"),
                SemanticError::Undeclared(x) => format!("'{}' is not defined", x),
                SemanticError::AlreadyDeclared(x) => format!("'{}' has already been declared", x),
                SemanticError::UnknownAssignment => format!("Cannot infer value type from usage"),
                SemanticError::UnsupportedNegation(_) => todo!(),
                SemanticError::IllegalTestBlock => format!("Invalid @tests block. Test blocks can only be used in the global scope of a module or file"),
                SemanticError::InconsistentAssignment(x, y) => format!("Type '{}' cannot be assigned to type '{}'", y, x),
                SemanticError::InconsistentInitializer => todo!(),
                SemanticError::HeterogenousArray(x, y) => format!(
                    "Elements of type '{}' and '{}' cannot be put in the same array. Arrays can only contain elements of the same type", x, y
                ),
                SemanticError::AliasUsedAsValue(x) => format!("'{}' is a type, but it is being used as a value", x),
                SemanticError::ValueUsedAsAlias(x) => format!("'{}' is a value, but it is being used as a type", x),
                SemanticError::UnusedVariable => todo!(),
                SemanticError::AssigningToNil => todo!(),
            }
        )
    }
}
