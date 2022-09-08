use macros::Location;

use crate::{Block, Expression, Identifier, Location, Statement, TextSpan, Type};

/// A function declaration. e.g.
/// ```pebble
/// @function sayHello() {
///     println "Hello, world!";
/// }
/// ```
#[derive(Location, Debug, Clone, PartialEq)]
pub struct Function<'a> {
    pub name: Identifier<'a>,
    pub label: Option<Type<'a>>,
    pub parameters: Vec<Parameter<'a>>,
    pub return_type: Option<Type<'a>>,
    pub body: Block<'a>,
    pub span: TextSpan,
}

#[derive(Location, Clone, Debug, PartialEq)]
pub struct Parameter<'a> {
    pub name: Identifier<'a>,
    pub label: Option<Type<'a>>,
    pub span: TextSpan,
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
    pub body: Block<'a>,
    pub span: TextSpan,
}

/// A variable declaration.
/// ```pebble
/// @let name: String = "johnny";
/// ```
#[derive(Location, Clone, Debug, PartialEq)]
pub struct LetDeclaration<'a> {
    pub identifier: Identifier<'a>,
    pub initializer: Option<Expression<'a>>,
    pub type_label: Option<Type<'a>>,
    pub span: TextSpan,
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
