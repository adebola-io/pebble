use macros::Location;

use crate::{Block, Expression, Identifier, Location, Statement, TextSpan, TextString, Type};

/// A function declaration. e.g.
/// ```pebble
/// @function sayHello() {
///     println "Hello, world!";
/// }
/// ```
#[derive(Location, Debug, Clone, PartialEq)]
pub struct Function<'a> {
    pub name: Identifier<'a>,
    pub generic_arguments: Option<Vec<GenericArgument<'a>>>,
    pub parameters: Vec<Parameter<'a>>,
    pub return_type: Option<Type<'a>>,
    pub body: Block<'a>,
    pub span: TextSpan,
}

#[derive(Location, Clone, Debug, PartialEq)]
pub struct GenericArgument<'a> {
    pub name: Identifier<'a>,
    pub implements: Option<Vec<Identifier<'a>>>,
    pub span: TextSpan,
}

#[derive(Location, Clone, Debug, PartialEq)]
pub struct Parameter<'a> {
    pub name: Identifier<'a>,
    pub label: Option<Type<'a>>,
    pub span: TextSpan,
}

/// Declaration of a type name that is an alias of another.
/// ```pebble
/// @type Meters = Number;
/// ```
#[derive(Location, Clone, Debug, PartialEq)]
pub struct TypeAlias<'a> {
    pub name: Identifier<'a>,
    pub generic_arguments: Option<Vec<GenericArgument<'a>>>,
    pub value: Type<'a>,
    pub span: TextSpan,
}

/// An abstract structure that allows the enforcing of properties on classes and objects. e.g.
/// ```pebble
/// @interface Equatable {
///     isGreater: (rhs: Self) -> Boolean,
///     isLesser: (rhs: Self) -> Boolean,
/// }
/// ```
#[derive(Location, Clone, Debug, PartialEq)]
pub struct Interface<'a> {
    pub name: Identifier<'a>,
    pub generic_arguments: Option<Vec<GenericArgument<'a>>>,
    pub properties: Vec<Property<'a>>,
    pub span: TextSpan,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Property<'a> {
    Method {
        name: Identifier<'a>,
        generic_arguments: Option<Vec<GenericArgument<'a>>>,
        parameters: Vec<Parameter<'a>>,
        return_type: Option<Type<'a>>,
        body: Block<'a>,
        span: TextSpan,
    },
    Attribute {
        key: Identifier<'a>,
        type_label: Option<Type<'a>>,
        value: Option<Expression<'a>>,
        span: TextSpan,
    },
    Implement {
        interface: Identifier<'a>,
        span: TextSpan,
    },
}

impl<'a> Location for Property<'a> {
    fn get_range(&self) -> TextSpan {
        match self {
            Self::Method { span, .. }
            | Self::Attribute { span, .. }
            | Self::Implement { span, .. } => *span,
        }
    }
}

/// A namespace of code that encloses related code. Every file is a module by default.
/// ```pebble
/// @module process {
///     @public @function args() -> Args {
///         // Code...
///     }
/// }
/// ```
#[derive(Location, Clone, Debug, PartialEq)]
pub struct Module<'a> {
    pub name: Identifier<'a>,
    pub body: Block<'a>,
    pub span: TextSpan,
}

/// A variable declaration.
/// ```pebble
/// @let name: String = "johnny";
/// ```
#[derive(Location, Clone, Debug, PartialEq)]
pub struct VariableDeclaration<'a> {
    pub name: Identifier<'a>,
    pub kind: VarKind,
    pub initializer: Option<Expression<'a>>,
    pub type_label: Option<Type<'a>>,
    pub span: TextSpan,
}

#[derive(Clone, Debug, PartialEq)]
pub enum VarKind {
    Let,
    Const,
}

/// A testing block, i.e. a group of functions for testing code functionality. e.g.
/// ```pebble
/// @tests {
///     @function itAdds() {
///         core.assert.isEqual(2 + 2, 4);
///     }
/// }
/// ```
#[derive(Location, Clone, Debug, PartialEq)]
pub struct TestBlock<'a> {
    pub body: Block<'a>,
    pub span: TextSpan,
}

/// A modifier that indicates that a function, variable, class, record or type is accessible from outside the file in which it was defined.
/// ```pebble
/// @public @function doStuff() {
///     // Doing stuff...
/// }
/// ```
#[derive(Location, Clone, Debug, PartialEq)]
pub struct PublicModifier<'a> {
    pub statement: Box<Statement<'a>>,
    pub span: TextSpan,
}

/// A statement that concatenates the content of another file to the top of a file. e.g.
/// ```pebble
/// @prepend "./otherfile.peb";
/// ```
#[derive(Location, Debug, Clone, PartialEq)]
pub struct PrependStatement<'a> {
    pub source: Expression<'a>,
    pub span: TextSpan,
}

/// A statement that retrieves a function, module, class, type or variable from another file, module or pile in the workspace.
/// Only public items can be imported from other files.
/// ```pebble
/// @use { colors, timer } from "utils";
/// ```
#[derive(Location, Debug, Clone, PartialEq)]
pub struct UseImport<'a> {
    pub imports: Vec<Import<'a>>,
    pub source: TextString<'a>,
    pub span: TextSpan,
}

/// An import into a module or file.
#[derive(Location, Debug, Clone, PartialEq)]
pub struct Import<'a> {
    pub imported_name: Identifier<'a>,
    pub collapsed_import: bool,
    pub local_name: Option<Identifier<'a>>,
    pub span: TextSpan,
}
