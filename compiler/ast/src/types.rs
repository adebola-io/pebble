use macros::Location;

use crate::{GenericArgument, Identifier, Location, Parameter, TextSpan};

#[derive(Clone, Debug, PartialEq)]
pub enum Type<'a> {
    Concrete(ConcreteType<'a>),
    Function(FunctionType<'a>),
    Dot(DotType<'a>),
}

impl Type<'_> {
    /// Returns `true` if the type is [`Dot`].
    ///
    /// [`Dot`]: Type::Dot
    pub fn is_dot(&self) -> bool {
        matches!(self, Self::Dot(..))
    }

    /// Returns `true` if the type is [`Concrete`].
    ///
    /// [`Concrete`]: Type::Concrete
    pub fn is_concrete(&self) -> bool {
        matches!(self, Self::Concrete(..))
    }

    /// Returns `true` if the type is [`Function`].
    ///
    /// [`Function`]: Type::Function
    pub fn is_function(&self) -> bool {
        matches!(self, Self::Function(..))
    }
}

impl<'a> Type<'a> {
    pub fn create_dot_type(object: Self, property: Self) -> Self {
        let span = [object.get_range()[0], property.get_range()[1]];
        Type::Dot(DotType {
            levels: vec![object, property],
            span,
        })
    }
}

impl Location for Type<'_> {
    fn get_range(&self) -> TextSpan {
        match self {
            Type::Concrete(ConcreteType { span, .. })
            | Type::Function(FunctionType { span, .. })
            | Type::Dot(DotType { span, .. }) => *span,
        }
    }
}

/// Types that imply classes or enums. e.g. `a: String` or `b: Stack<UnsignedInt>`
#[derive(Location, Clone, Debug, PartialEq)]
pub struct ConcreteType<'a> {
    pub name: Identifier<'a>,
    pub arguments: Vec<Type<'a>>,
    pub span: TextSpan,
}

/// Types that imply functions. e.g. `a: () -> Nil` or `b: <T>(a: T) -> T`
#[derive(Location, Clone, Debug, PartialEq)]
pub struct FunctionType<'a> {
    pub parameters: Vec<Parameter<'a>>,
    pub return_type: Box<Type<'a>>,
    pub generic_arguments: Option<Vec<GenericArgument<'a>>>,
    pub span: TextSpan,
}

/// Types that are children of external modules or files. e.g. `a: core.prelude.String`
#[derive(Location, Clone, Debug, PartialEq)]
pub struct DotType<'a> {
    pub levels: Vec<Type<'a>>,
    pub span: TextSpan,
}
