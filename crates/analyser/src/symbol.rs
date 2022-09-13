use std::{fmt::Display, rc::Rc};

use ast::{Operator, TextSpan};
use errors::SemanticError;

pub type ResolveError = (SemanticError<SymbolType>, TextSpan);
pub type SymbolOrError = Result<Symbol, ResolveError>;

/// An item refers to a single value in the program, either a function, a class, a constant, etc.
#[derive(Debug, Clone, PartialEq)]
pub struct Symbol {
    pub _type: SymbolType,
    pub span: TextSpan,
}

impl Default for Symbol {
    fn default() -> Self {
        Symbol {
            _type: SymbolType::Nil,
            span: [[0, 0], [0, 0]],
        }
    }
}

impl Symbol {
    pub fn nil(span: TextSpan) -> Self {
        Self {
            _type: SymbolType::Nil,
            span,
        }
    }
    pub fn unknown(span: TextSpan) -> Self {
        Self {
            _type: SymbolType::Unknown,
            span,
        }
    }
    pub fn array(inner_type: SymbolType, span: TextSpan) -> Self {
        Self {
            _type: SymbolType::Array(Box::new(inner_type)),
            span,
        }
    }
    pub fn is_nil(&self) -> bool {
        matches!(
            self,
            Symbol {
                _type: SymbolType::Nil,
                ..
            }
        )
    }
    pub fn is_unknown(&self) -> bool {
        matches!(
            self,
            Symbol {
                _type: SymbolType::Unknown,
                ..
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolType {
    Nil,
    Unknown,
    Module,
    String,
    Number,
    Character,
    Array(Box<SymbolType>),
    Boolean,
    Custom {},
    Class(ClassType),
    Function(FunctionType),
    Instance { class: Rc<ClassType> },
}
#[derive(Debug, Clone, PartialEq)]
pub struct ClassType {
    name: String,
    arguments: Box<Symbol>,
    properties: Vec<Box<Symbol>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionType {
    pub parameters: Vec<ParameterType>,
    pub return_type: Rc<Symbol>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParameterType {
    pub name: String,
    pub _type: SymbolType,
    pub span: TextSpan,
}

impl Display for SymbolType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SymbolType::Nil => String::from("Nil"),
                SymbolType::Unknown => String::from("Unknown"),
                SymbolType::Module => String::from("Module"),
                SymbolType::String => String::from("String"),
                SymbolType::Number => String::from("Number"),
                SymbolType::Character => String::from("Character"),
                SymbolType::Boolean => String::from("Boolean"),
                SymbolType::Custom {} => String::from("Custom"),
                SymbolType::Array(s) => format!("[{}]", s),
                SymbolType::Class(ClassType { name, .. }) => name.to_string(),
                SymbolType::Function(FunctionType {
                    parameters,
                    return_type,
                }) => format!(
                    "({})-> {}",
                    {
                        let mut output = String::new();
                        let mut i = 0;
                        while i < parameters.len() {
                            output.push_str(&format!("{}", parameters[i]));
                            if i < parameters.len() - 1 {
                                output.push_str(", ")
                            }
                            i += 1;
                        }
                        output
                    },
                    return_type._type
                ),
                SymbolType::Instance { class } => class.name.to_string(),
            }
        )
    }
}

impl Display for ParameterType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{}: {}", self.name, self._type))
    }
}

impl Symbol {
    pub fn mul(&self, rhs: Self) -> SymbolOrError {
        let span = [self.span[0], rhs.span[0]];
        match (&self._type, &rhs._type) {
            // a: String * b: Number = c: String
            (SymbolType::String, SymbolType::Number) => Ok(Symbol {
                _type: SymbolType::String,
                span,
            }),
            // a: Number * b: Number = c: Number
            (SymbolType::Number, SymbolType::Number) => Ok(Symbol {
                _type: SymbolType::Number,
                span,
            }),
            _ => Err((
                SemanticError::UnsupportedBinaryOperation(
                    Operator::Multiply,
                    self._type.clone(),
                    rhs._type,
                ),
                span,
            )),
        }
    }
    pub fn add(&self, rhs: Self) -> SymbolOrError {
        let span = [self.span[0], rhs.span[0]];
        match (&self._type, &rhs._type) {
            // a: String + b: String = c: String
            (SymbolType::String, SymbolType::String) => Ok(Symbol {
                _type: SymbolType::String,
                span,
            }),
            // a: Number + b: Number = c: Number
            (SymbolType::Number, SymbolType::Number) => Ok(Symbol {
                _type: SymbolType::Number,
                span,
            }),
            _ => Err((
                SemanticError::UnsupportedBinaryOperation(
                    Operator::Add,
                    self._type.clone(),
                    rhs._type,
                ),
                span,
            )),
        }
    }
    pub fn operate(&self, rhs: Self, operator: &Operator) -> SymbolOrError {
        let span = [self.span[0], rhs.span[0]];
        if let (SymbolType::Number, SymbolType::Number) = (&self._type, &rhs._type) {
            Ok(Symbol {
                _type: SymbolType::Number,
                span,
            })
        } else {
            Err((
                SemanticError::UnsupportedBinaryOperation(
                    operator.clone(),
                    self._type.clone(),
                    rhs._type,
                ),
                span,
            ))
        }
    }
    pub fn negate(&self) -> SymbolOrError {
        if let SymbolType::Boolean = self._type {
            Ok(self.clone())
        } else {
            Err((
                SemanticError::UnsupportedNegation(self._type.clone()),
                self.span,
            ))
        }
    }
    pub fn equate(&self, rhs: Self, operator: &Operator) -> SymbolOrError {
        let span = [self.span[0], rhs.span[0]];
        if self._type == rhs._type {
            Ok(Symbol {
                _type: SymbolType::Boolean,
                span,
            })
        } else {
            Err((
                SemanticError::ComparisionBetweenDifferentTypes(
                    operator.clone(),
                    self._type.clone(),
                    rhs._type,
                ),
                span,
            ))
        }
    }
    pub fn compare(&self, rhs: Self, operator: &Operator) -> SymbolOrError {
        let span = [self.span[0], rhs.span[0]];
        // a: Number x b: Number = c: Number
        if let (SymbolType::Number, SymbolType::Number) = (&self._type, &rhs._type) {
            Ok(Symbol {
                _type: SymbolType::Number,
                span,
            })
        } else {
            Err((
                SemanticError::UnsupportedBinaryOperation(
                    operator.clone(),
                    self._type.clone(),
                    rhs._type,
                ),
                span,
            ))
        }
    }
    pub fn andor(&self, rhs: Self, operator: &Operator) -> SymbolOrError {
        let span = [self.span[0], rhs.span[0]];
        // a: Boolean && b: Boolean = c: Boolean
        // a: Boolean || b: Boolean = c: Boolean
        if let (SymbolType::Boolean, SymbolType::Boolean) = (&self._type, &rhs._type) {
            Ok(Symbol {
                _type: SymbolType::Boolean,
                span,
            })
        } else {
            Err((
                SemanticError::UnsupportedLogicalOperation(
                    operator.clone(),
                    self._type.clone(),
                    rhs._type,
                ),
                span,
            ))
        }
    }
}
