use std::fmt::Display;

use ast::Operator;

#[derive(Debug, PartialEq)]
pub enum SemanticError<T>
where
    T: Display,
{
    Unaddable(T, T),
    UnsupportedOperation(Operator, T, T),
    UnsupportedUnaryOperation(Operator, T),
    Undeclared(String),
    AssignmentToConst,
    Uninitialized(String),
    AlreadyDeclared(String),
    InvalidTernaryTest(T),
    InvalidIndex(T),
    InvalidIndexer(T),
    InconsistentTernarySides(T, T),
    Unassignable(T, T),
    InvalidRangeBoundaries,
    UnknownAssignment(String),
    UnequalGenericArgs(String, usize, usize),
    UnexpectedGenerics(String),
    Uncallable(T),
    UnequalArgs(usize, usize),
    ParameterMismatch(T, T),
    IllegalTestBlock,
    AssigningToNil,
    OperationOnNil,
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
                SemanticError::UnequalGenericArgs(x, y, z) => format!("Unequal generic arguments. Expected {y} arguments for '{x}' and got {z}"),
                SemanticError::AssignmentToConst => format!("Assignment to constant variable or parameter"),
                SemanticError::UnexpectedGenerics(x) => format!("Unexpected arguments for type '{x}'. '{x}' has no generic arguments"),
                SemanticError::Unaddable(x, y) => format!("Cannot add types '{x}' and '{y}'"),
                SemanticError::OperationOnNil => format!("Cannot perform operation on possibly nil values"),
                SemanticError::UnsupportedOperation(op, x, y) => format!("The operation '{op}' is not defined for types '{x}' and '{y}'"),
                SemanticError::UnsupportedUnaryOperation(op, x) => format!("The operation {op} is not defined for '{x}'"),
                SemanticError::InvalidTernaryTest(x) => format!("Expected Boolean type for ternary operation test, got '{x}'"),
                SemanticError::InconsistentTernarySides(x, y) => format!(
                    "Expected '{x}' for alternate expression, got '{y}'. Both sides of a ternary expression must have the same type"
                ),
                SemanticError::Uninitialized(x) => format!("'{x}' is being used before it is initialized"),
                SemanticError::InvalidIndex(x) => format!("The type '{x}' is not an indexable type"),
                SemanticError::InvalidIndexer(x) => format!("The type '{x}' cannot be used as an index"),
                SemanticError::InvalidRangeBoundaries => format!("Invalid range. The boundaries of a range must be both be either characters or numbers"),
                SemanticError::Undeclared(x) => format!("'{x}' is not defined"),
                SemanticError::AlreadyDeclared(x) => format!("'{x}' has either already been declared or just initialized in this scope"),
                SemanticError::UnknownAssignment(x) => format!("Cannot infer the type of '{x}' from its usage."),
                SemanticError::IllegalTestBlock => format!("Invalid @tests block. Test blocks can only be used in the global scope of a module or file"),
                SemanticError::Unassignable(x, y) => format!("Type '{y}' cannot be assigned to type '{x}'"),
                SemanticError::Uncallable(x) => format!("'{x}' is not a callable type"),
                SemanticError::UnequalArgs(x, y) => format!("Function or Constructor required {x} arguments but got {y}"),
                SemanticError::ParameterMismatch(x, y) => format!("Invalid argument. Expected type '{x}' and got '{y}'"),
                SemanticError::HeterogenousArray(x, y) => format!(
                    "Elements of type '{x}' and '{y}' cannot be put in the same array. Arrays can only contain elements of the same type",
                ),
                SemanticError::AssigningToNil => format!("Cannot assign nil value to variable or constant"),
            }
        )
    }
}
