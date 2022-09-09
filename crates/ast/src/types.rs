use crate::{GenericArgument, Identifier, Location, Parameter, TextSpan};

#[derive(Clone, Debug, PartialEq)]
pub struct Type<'a> {
    pub kind: TypeKind<'a>,
    pub span: TextSpan,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TypeKind<'a> {
    /// Types that imply classes or enums. e.g. `a: String` or `b: UnsignedInt`
    Concrete {
        name: Identifier<'a>,
        objects: Vec<Identifier<'a>>,
        arguments: Vec<Type<'a>>,
    },
    /// Types that imply functions. e.g. `a: () -> Nil` or `b: <T>(a: T) -> T`
    Functional {
        parameters: Vec<Parameter<'a>>,
        return_type: Box<Type<'a>>,
        generic_arguments: Option<Vec<GenericArgument<'a>>>,
    },
}

impl Type<'_> {
    fn is_functional_type(&self) -> bool {
        matches!(
            self,
            Type {
                kind: TypeKind::Functional { .. },
                ..
            }
        )
    }
    fn is_concrete_type(&self) -> bool {
        matches!(
            self,
            Type {
                kind: TypeKind::Concrete { .. },
                ..
            }
        )
    }
}

impl Location for Type<'_> {
    fn get_range(&self) -> TextSpan {
        self.span
    }
}
