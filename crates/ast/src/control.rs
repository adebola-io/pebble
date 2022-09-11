use std::marker::PhantomData;

use macros::Location;

use crate::{Expression, Identifier, Location, Parameter, Statement, TextSpan};

#[derive(Location, Debug, Clone, PartialEq)]
pub struct TryBlock<'a> {
    pub body: Block<'a>,
    pub span: TextSpan,
    pub recover: Option<RecoverBlock<'a>>,
}

#[derive(Location, Clone, Debug, PartialEq)]
pub struct RecoverBlock<'a> {
    pub params: Vec<Parameter<'a>>,
    pub span: TextSpan,
    pub body: Block<'a>,
}

/// A block statement. e.g.
/// ```pebble
/// {
///     print "This is a block statement.";
/// }
/// ```
#[derive(Location, Clone, Debug, PartialEq)]
pub struct Block<'a> {
    pub body: Vec<Statement<'a>>,
    pub span: TextSpan,
}

/// A loop statement, with the form:
/// ```pebble
/// loop (10) {
///     doStuff();
/// }
/// ```
/// The above loop runs the function `doStuff()` 10 times.
/// To create an infinite loop the constraint can be omitted.
#[derive(Location, Debug, Clone, PartialEq)]
pub struct Loop<'a> {
    pub constraint: Option<Expression<'a>>,
    pub body: Block<'a>,
    pub span: TextSpan,
}

/// A for loop statement, with the form:
/// ```pebble
/// for (item in iterator) {
///     doStuffWithItem(item);
/// }
/// ```
/// The block can be replaced with a single statement.
#[derive(Location, Debug, Clone, PartialEq)]
pub struct ForLoop<'a> {
    pub item: Identifier<'a>,
    pub iterator: Expression<'a>,
    pub span: TextSpan,
}

/// A generic if statement, as it is in other C derived languages. e.g.
/// ```pebble
/// if (is_true) {
///     doStuff();
/// } else {
///     doOtherStuff();
/// }
/// ```
/// As with Javascript, the blocks can be replaced with a single statement, and the else is optional.
#[derive(Location, Debug, Clone, PartialEq)]
pub struct IfStatement<'a> {
    pub test: Expression<'a>,
    pub body: Box<Statement<'a>>,
    pub alternate: Option<Box<Statement<'a>>>,
    pub span: TextSpan,
}
/// A while statement, with the form:
/// ```pebble
/// while (is_true) {
///     doStuff();
/// }
/// ```
#[derive(Location, Debug, Clone, PartialEq)]
pub struct WhileStatement<'a> {
    pub test: Expression<'a>,
    pub body: Box<Statement<'a>>,
    pub span: TextSpan,
}

/// A statement that prints to the standard output. e.g.
/// ```pebble
/// println "Hello, world!";
/// ```
#[derive(Location, Debug, Clone, PartialEq)]
pub struct PrintLnStatement<'a> {
    pub argument: Expression<'a>,
    pub span: TextSpan,
}

/// A statement that halts execution of the current code context and rolls back the stack trace to the last try block.
/// It is useful for error handling in debugging, or preventing program crashes in production.
/// ```pebble
/// try {
///     if (procedure.isValid) {
///         doStuff();
///     } else {
///         crash Error("This is an invalid procedure!");
///     }
/// } recover(error) {
///     core.io.printErr(error.message)
/// }
/// ```
#[derive(Location, Debug, Clone, PartialEq)]
pub struct CrashStatement<'a> {
    pub argument: Expression<'a>,
    pub span: TextSpan,
}

/// A break statement that halts a loop.
#[derive(Location, Debug, Clone, PartialEq)]
pub struct Break<'a> {
    pub span: TextSpan,
    pub phantom: PhantomData<&'a i32>,
}

/// A continue statement that skips over the next iteration in the loop.
#[derive(Location, Debug, Clone, PartialEq)]
pub struct Continue<'a> {
    pub span: TextSpan,
    pub phantom: PhantomData<&'a i32>,
}

/// Any expression statement.
#[derive(Location, Debug, Clone, PartialEq)]
pub struct ExpressionStatement<'a> {
    pub expression: Expression<'a>,
    pub span: TextSpan,
}

/// A return statement.
/// ```pebble
/// @function getX () {
///     return x;
/// }
/// ```
#[derive(Location, Clone, Debug, PartialEq)]
pub struct ReturnStatement<'a> {
    pub argument: Option<Expression<'a>>,
    pub span: TextSpan,
}
