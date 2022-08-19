use crate::{Node, Operator};

/// The base node for an expression.
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    StringExpr(StringExpr),
    NumericExpr(NumericExpr),
    BooleanExpr(BooleanExpr),
    UnaryExpr(UnaryExpr),
    ArrayExpr(ArrayExpr),
    AccessExpr(AccessExpr),
    DotExpr(DotExpr),
    RangeExpr(RangeExpr),
    BinaryExpr(BinaryExpr),
}

/// A string literal in Pebble. e.g. `"John Doe", "One does not simply walk into Mordor"`
#[derive(Debug, Clone, PartialEq)]
pub struct StringExpr {
    pub value: String,
}

/// A number literal in Pebble. e.g. `1, 3.5, 4e9, 0x03, Ob22, 007`
#[derive(Debug, Clone, PartialEq)]
pub struct NumericExpr {
    pub value: String,
}

// A boolean literal, i.e. `true` or `false`
#[derive(Debug, Clone, PartialEq)]
pub struct BooleanExpr {
    pub value: bool,
}
/// An operation that occurs on only one operand. e.g. `!a, ~b`
#[derive(Debug, Clone, PartialEq)]
pub struct UnaryExpr {
    pub operator: Operator,
    pub operand: Box<Node>,
}

/// An operation with two operands that yields a new value. e.g `a + b`
#[derive(Debug, Clone, PartialEq)]
pub struct BinaryExpr {
    pub operator: Operator,
    pub left: Box<Node>,
    pub right: Box<Node>,
}

/// A member or dot access of a model. e.g. `a.b`
#[derive(Debug, Clone, PartialEq)]
pub struct DotExpr {
    pub model: Box<Node>,
    pub property: Box<Node>,
}

/// An array in Pebble. e.g. `[a, b, c]`
#[derive(Debug, PartialEq, Clone)]
pub struct ArrayExpr {
    pub elements: Vec<Box<Node>>,
}

/// An expression that access an index of an array. e.g `a[b]`.
#[derive(Debug, PartialEq, Clone)]
pub struct AccessExpr {
    pub sequence: Box<Node>,
    pub index: Box<Node>,
}

/// An expression that expresses a numeric or alphabetic range. e.g. a..b
#[derive(Debug, PartialEq, Clone)]
pub struct RangeExpr {
    pub upper_limit: String,
    pub lower_limit: String,
}
