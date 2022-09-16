use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum TypeError<T, U>
where
    T: Display,
    U: Display,
{
    Unaddable(T, T),
    UnsupportedOperation(U, T, T),
    UnsupportedUnaryOperation(U, T),
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
    UnsatisfiedGenericConstraint(T, T),
    Uncallable(T),
    UnequalArgs(usize, usize),
    ParameterMismatch(T, T),
    IllegalTestBlock,
    AssigningToNil,
    OperationOnNil,
    HeterogenousArray(T, T),
}

impl<T, U> Display for TypeError<T, U>
where
    T: Display,
    U: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.",
            match self {
                TypeError::UnequalGenericArgs(x, y, z) => format!("Unequal generic arguments. Expected {y} arguments for '{x}' and got {z}"),
                TypeError::AssignmentToConst => format!("Assignment to constant variable or parameter"),
                TypeError::UnexpectedGenerics(x) => format!("Unexpected arguments for type '{x}'. '{x}' has no generic arguments"),
                TypeError::Unaddable(x, y) => format!("Cannot add types '{x}' and '{y}'"),
                TypeError::OperationOnNil => format!("Cannot perform operation on possibly nil values"),
                TypeError::UnsupportedOperation(op, x, y) => format!("The operation '{op}' is not defined for types '{x}' and '{y}'"),
                TypeError::UnsupportedUnaryOperation(op, x) => format!("The operation {op} is not defined for '{x}'"),
                TypeError::InvalidTernaryTest(x) => format!("Expected Boolean type for ternary operation test, got '{x}'"),
                TypeError::InconsistentTernarySides(x, y) => format!(
                    "Expected '{x}' for alternate expression, got '{y}'. Both sides of a ternary expression must have the same type"
                ),
                TypeError::Uninitialized(x) => format!("'{x}' is being used before it is initialized"),
                TypeError::InvalidIndex(x) => format!("The type '{x}' is not an indexable type"),
                TypeError::InvalidIndexer(x) => format!("The type '{x}' cannot be used as an index"),
                TypeError::InvalidRangeBoundaries => format!("Invalid range. The boundaries of a range must be both be either characters or numbers"),
                TypeError::Undeclared(x) => format!("'{x}' is not defined"),
                TypeError::AlreadyDeclared(x) => format!("'{x}' has  already been declared in this scope"),
                TypeError::UnknownAssignment(x) => format!("Cannot infer the type of '{x}' from its usage."),
                TypeError::IllegalTestBlock => format!("Invalid @tests block. Test blocks can only be used in the global scope of a module or file"),
                TypeError::Unassignable(x, y) => format!("Type '{y}' cannot be assigned to type '{x}'"),
                TypeError::Uncallable(x) => format!("'{x}' is not a callable type"),
                TypeError::UnsatisfiedGenericConstraint(x, y) => format!("{x} does not satisfy the generic constraint because it does not implement '{y}'"),
                TypeError::UnequalArgs(x, y) => format!("Function or Constructor required {x} arguments but got {y}"),
                TypeError::ParameterMismatch(x, y) => format!("Invalid argument. Expected type '{x}' and got '{y}'"),
                TypeError::HeterogenousArray(x, y) => format!(
                    "Elements of type '{x}' and '{y}' cannot be put in the same array. Arrays can only contain elements of the same type",
                ),
                TypeError::AssigningToNil => format!("Cannot assign nil value to variable or constant"),
            }
        )
    }
}
