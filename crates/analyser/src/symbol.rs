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
    pub fn primitives() -> Vec<(&'static str, Self)> {
        vec![
            (
                "Number",
                Symbol {
                    _type: SymbolType::Alias(TypeAlias {
                        actual_symbol: Box::new(Symbol {
                            _type: SymbolType::Number,
                            span: [[0, 0], [0, 0]],
                        }),
                        arguments: Vec::new(),
                    }),
                    span: [[0, 0], [0, 0]],
                },
            ),
            (
                "String",
                Symbol {
                    _type: SymbolType::Alias(TypeAlias {
                        actual_symbol: Box::new(Symbol {
                            _type: SymbolType::String,
                            span: [[0, 0], [0, 0]],
                        }),
                        arguments: Vec::new(),
                    }),
                    span: [[0, 0], [0, 0]],
                },
            ),
            (
                "Boolean",
                Symbol {
                    _type: SymbolType::Alias(TypeAlias {
                        actual_symbol: Box::new(Symbol {
                            _type: SymbolType::Boolean,
                            span: [[0, 0], [0, 0]],
                        }),
                        arguments: Vec::new(),
                    }),
                    span: [[0, 0], [0, 0]],
                },
            ),
            (
                "Character",
                Symbol {
                    _type: SymbolType::Alias(TypeAlias {
                        actual_symbol: Box::new(Symbol {
                            _type: SymbolType::Character,
                            span: [[0, 0], [0, 0]],
                        }),
                        arguments: Vec::new(),
                    }),
                    span: [[0, 0], [0, 0]],
                },
            ),
        ]
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
    pub fn array(inner_type: Symbol, span: TextSpan) -> Self {
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
    Generic(Generic),
    Number,
    Character,
    Array(Box<Symbol>),
    Boolean,
    Alias(TypeAlias),
    Class(ClassType),
    Function(FunctionType),
    Instance { class: Rc<ClassType> },
}
#[derive(Debug, Clone, PartialEq)]
pub struct ClassType {
    pub name: String,
    pub arguments: Vec<Box<Symbol>>,
    pub properties: Vec<Box<Symbol>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeAlias {
    pub actual_symbol: Box<Symbol>,
    pub arguments: Vec<Symbol>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionType {
    pub parameters: Vec<ParameterSymbol>,
    pub return_type: Box<Symbol>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParameterSymbol {
    pub name: String,
    pub _type: SymbolType,
    pub span: TextSpan,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Generic {
    pub name: String,
    pub implements: Box<Symbol>,
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
                SymbolType::Alias(TypeAlias { actual_symbol, .. }) =>
                    actual_symbol._type.to_string(),
                SymbolType::Array(s) => format!("Array<{}>", s._type),
                SymbolType::Class(ClassType { name, .. }) => name.to_string(),
                SymbolType::Function(FunctionType {
                    parameters,
                    return_type,
                }) => format!(
                    "({}) -> {}",
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
                SymbolType::Generic(Generic { .. }) => todo!(),
            }
        )
    }
}

impl Display for ParameterSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{}: {}", self.name, self._type))
    }
}

impl Symbol {
    pub fn mul(&self, rhs: Self) -> SymbolOrError {
        let span = [self.span[0], rhs.span[0]];
        let mut right_type = &rhs._type;
        let mut left_type = &self._type;
        // Unwrap aliased types.
        while let SymbolType::Alias(a) = left_type {
            left_type = &a.actual_symbol._type
        }
        while let SymbolType::Alias(a) = right_type {
            right_type = &a.actual_symbol._type
        }
        match (left_type, right_type) {
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
        let mut right_type = &rhs._type;
        let mut left_type = &self._type;
        // Unwrap aliased types.
        while let SymbolType::Alias(a) = left_type {
            left_type = &a.actual_symbol._type
        }
        while let SymbolType::Alias(a) = right_type {
            right_type = &a.actual_symbol._type
        }
        match (left_type, right_type) {
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
        let mut right_type = &rhs._type;
        let mut left_type = &self._type;
        // Unwrap aliased types.
        while let SymbolType::Alias(a) = left_type {
            left_type = &a.actual_symbol._type
        }
        while let SymbolType::Alias(a) = right_type {
            right_type = &a.actual_symbol._type
        }
        if let (SymbolType::Number, SymbolType::Number) = (left_type, right_type) {
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
        let mut _type = &self._type;
        // Unwrap aliased types.
        while let SymbolType::Alias(a) = _type {
            _type = &a.actual_symbol._type
        }
        if let SymbolType::Boolean = _type {
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
        let mut right_type = &rhs._type;
        let mut left_type = &self._type;
        // Unwrap aliased types.
        while let SymbolType::Alias(a) = left_type {
            left_type = &a.actual_symbol._type
        }
        while let SymbolType::Alias(a) = right_type {
            right_type = &a.actual_symbol._type
        }
        if left_type == right_type {
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
        let mut right_type = &rhs._type;
        let mut left_type = &self._type;
        // Unwrap aliased types.
        while let SymbolType::Alias(a) = left_type {
            left_type = &a.actual_symbol._type
        }
        while let SymbolType::Alias(a) = right_type {
            right_type = &a.actual_symbol._type
        }
        // a: Number x b: Number = c: Number
        if let (SymbolType::Number, SymbolType::Number) = (left_type, right_type) {
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
        let mut right_type = &rhs._type;
        let mut left_type = &self._type;
        // Unwrap aliased types.
        while let SymbolType::Alias(a) = left_type {
            left_type = &a.actual_symbol._type
        }
        while let SymbolType::Alias(a) = right_type {
            right_type = &a.actual_symbol._type
        }
        // a: Boolean && b: Boolean = c: Boolean
        // a: Boolean || b: Boolean = c: Boolean
        if let (SymbolType::Boolean, SymbolType::Boolean) = (left_type, right_type) {
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
