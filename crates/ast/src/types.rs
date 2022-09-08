use crate::{Location, Parameter, TextSpan};

#[derive(Clone, Debug, PartialEq)]
pub struct Type<'a> {
    name: String,
    kind: TypeKind<'a>,
    span: TextSpan,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TypeKind<'a> {
    Generic {
        arguments: Vec<Type<'a>>,
        implements: Vec<Type<'a>>,
    },
    Definite {
        arguments: Vec<Type<'a>>,
    },
    Functional {
        parameters: Vec<Parameter<'a>>,
        return_type: Box<Type<'a>>,
    },
}

impl Location for Type<'_> {
    fn get_range(&self) -> TextSpan {
        self.span
    }
}
