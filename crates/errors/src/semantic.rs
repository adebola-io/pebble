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
    UndeclaredVariable(String),
    AlreadyDeclared(String),
    UnsupportedNegation(T),
    InconsistentAssignment(String, String),
    InconsistentInitializer,
    UnusedVariable,
    AssigningToNil,
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
                    "The operation '{}' is not defined for '{}' and '{}'",
                    op, x, y
                ),
                SemanticError::UnsupportedLogicalOperation(op, x, y) => format!(
                    "Cannot perform the operation '{}' on a '{}' and a '{}'",
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
                SemanticError::UndeclaredVariable(x) => format!("Variable '{}' is not defined", x),
                SemanticError::AlreadyDeclared(x) => format!("'{}' has already been declared", x),
                SemanticError::UnsupportedNegation(_) => todo!(),
                SemanticError::InconsistentAssignment(_, _) => todo!(),
                SemanticError::InconsistentInitializer => todo!(),
                SemanticError::UnusedVariable => todo!(),
                SemanticError::AssigningToNil => todo!(),
            }
        )
    }
}
