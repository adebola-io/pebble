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
    pub fn is_nil(&self) -> bool {
        matches!(
            self,
            Symbol {
                _type: SymbolType::Nil,
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
    Boolean,
    Custom {},
    Class(ClassType),
    Function {
        arguments: Box<Symbol>,
        parameters: Vec<Symbol>,
        return_type: Rc<Symbol>,
    },
    Instance {
        class: Rc<ClassType>,
    },
}
#[derive(Debug, Clone, PartialEq)]
pub struct ClassType {
    name: String,
    arguments: Box<Symbol>,
    properties: Vec<Box<Symbol>>,
}

impl Display for SymbolType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SymbolType::Nil => "Nil",
                SymbolType::Unknown => "Unknown",
                SymbolType::Module => "Module",
                SymbolType::String => "String",
                SymbolType::Number => "Number",
                SymbolType::Character => "Character",
                SymbolType::Boolean => "Boolean",
                SymbolType::Custom {} => "Custom",
                SymbolType::Class(ClassType { name, .. }) => &name,
                SymbolType::Function { .. } => "Function",
                SymbolType::Instance { class } => &class.name,
            }
        )
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
        match (&self._type, &rhs._type) {
            // a: Number x b: Number = c: Number
            (SymbolType::Number, SymbolType::Number) => Ok(Symbol {
                _type: SymbolType::Number,
                span,
            }),
            _ => Err((
                SemanticError::UnsupportedBinaryOperation(
                    operator.clone(),
                    self._type.clone(),
                    rhs._type,
                ),
                span,
            )),
        }
    }
    pub fn negate(&self) -> SymbolOrError {
        match self._type {
            SymbolType::Boolean => Ok(self.clone()),
            _ => Err((
                SemanticError::UnsupportedNegation(self._type.clone()),
                self.span,
            )),
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
        match (&self._type, &rhs._type) {
            // a: Number x b: Number = c: Number
            (SymbolType::Number, SymbolType::Number) => Ok(Symbol {
                _type: SymbolType::Number,
                span,
            }),
            _ => Err((
                SemanticError::UnsupportedBinaryOperation(
                    operator.clone(),
                    self._type.clone(),
                    rhs._type,
                ),
                span,
            )),
        }
    }
    pub fn andor(&self, rhs: Self, operator: &Operator) -> SymbolOrError {
        let span = [self.span[0], rhs.span[0]];
        match (&self._type, &rhs._type) {
            // a: Boolean && b: Boolean = c: Boolean
            // a: Boolean || b: Boolean = c: Boolean
            (SymbolType::Boolean, SymbolType::Boolean) => Ok(Symbol {
                _type: SymbolType::Number,
                span,
            }),
            _ => Err((
                SemanticError::UnsupportedLogicalOperation(
                    operator.clone(),
                    self._type.clone(),
                    rhs._type,
                ),
                span,
            )),
        }
    }
}
