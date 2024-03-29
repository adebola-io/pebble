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
///     @implement PartialOrd,
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

/// A template-like structure for creating instances and objects in Pebble. e.g.
/// ```pebble
/// @class Person {
///     Person(name: String, age: String) {
///         self.name = name;
///         self.age = age;
///         self.id = generateID();
///     }
///     name: String,
///     age: UnsignedInt,
///     id: String,
/// }
/// ```
#[derive(Location, Clone, Debug, PartialEq)]
pub struct Class<'a> {
    pub name: Identifier<'a>,
    pub generic_arguments: Option<Vec<GenericArgument<'a>>>,
    pub properties: Vec<Property<'a>>,
    pub span: TextSpan,
}

/// The property of a class or interface.
#[derive(Clone, Debug, PartialEq)]
pub enum Property<'a> {
    Method(Method<'a>),
    Attribute(Attribute<'a>),
    Implement(Implement<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Method<'a> {
    pub name: Identifier<'a>,
    pub generic_arguments: Option<Vec<GenericArgument<'a>>>,
    pub is_static: bool,
    pub parameters: Vec<Parameter<'a>>,
    pub return_type: Option<Type<'a>>,
    pub body: Block<'a>,
    pub span: TextSpan,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Attribute<'a> {
    pub key: Identifier<'a>,
    pub type_label: Option<Type<'a>>,
    pub is_static: bool,
    pub is_readonly: bool,
    pub value: Option<Expression<'a>>,
    pub span: TextSpan,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Implement<'a> {
    pub interface: Identifier<'a>,
    pub span: TextSpan,
}

impl<'a> Location for Property<'a> {
    fn get_range(&self) -> TextSpan {
        match self {
            Self::Method(Method { span, .. })
            | Self::Attribute(Attribute { span, .. })
            | Self::Implement(Implement { span, .. }) => *span,
        }
    }
}

/// An enumerated value, which could be any of its defined variants. e.g.
/// ```pebble
///     @enum Directions {
///         Up,
///         Down,
///         Left,
///         Right
///     }
/// ```
#[derive(Location, Debug, Clone, PartialEq)]
pub struct Enum<'a> {
    pub name: Identifier<'a>,
    pub generic_arguments: Option<Vec<GenericArgument<'a>>>,
    pub variants: Vec<Variant<'a>>,
    pub span: TextSpan,
}

/// Any of the values an enum could take.
#[derive(Debug, Clone, PartialEq)]
pub enum Variant<'a> {
    Tuple {
        name: Identifier<'a>,
        elements: Vec<Type<'a>>,
        span: TextSpan,
    },
    Concrete {
        name: Identifier<'a>,
        span: TextSpan,
    },
}

impl Location for Variant<'_> {
    fn get_range(&self) -> TextSpan {
        match self {
            Self::Tuple { span, .. } | Self::Concrete { span, .. } => *span,
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

impl VarKind {
    /// Returns `true` if the var kind is [`Const`].
    ///
    /// [`Const`]: VarKind::Const
    pub fn is_const(&self) -> bool {
        matches!(self, Self::Const)
    }

    /// Returns `true` if the var kind is [`Let`].
    ///
    /// [`Let`]: VarKind::Let
    pub fn is_let(&self) -> bool {
        matches!(self, Self::Let)
    }
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

/// An immutable store of values that can be accessed by any part of the program in O(1) time.
/// ```pebble
///     @record NAMES {
///         1 -> "Akomolafe",
///         2 -> "Jonathan"
///     }
/// ```
#[derive(Location, Debug, Clone, PartialEq)]
pub struct Record<'a> {
    pub name: Identifier<'a>,
    pub mappings: Vec<Mapping<'a>>,
    pub span: TextSpan,
}

#[derive(Location, Debug, Clone, PartialEq)]
pub struct Mapping<'a> {
    pub key: Expression<'a>,
    pub value: Expression<'a>,
    pub span: TextSpan,
}
