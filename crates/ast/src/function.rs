use crate::{Location, Parameter, Statement, TextSpan, Type};

/// The name, parameters, generic label, parameters, return type and body of a function or a method.
/// ```pebble
/// doStuff<T>(name: T) -> String {
///     return name.toString();
/// }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionalSignature<'a> {
    pub name: Option<String>,
    pub generic_label: Option<Type<'a>>,
    pub parameters: Vec<Parameter<'a>>,
    pub return_type: Option<Type<'a>>,
    pub body: Statement<'a>,
    pub span: TextSpan,
}

impl Location for FunctionalSignature<'_> {
    fn get_range(&self) -> crate::TextSpan {
        self.span
    }
}
