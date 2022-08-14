use crate::{Node, Operator};

/// A string literal in Pebble. e.g. `"John Doe", "One does not simply walk into Mordor"`
#[derive(Debug, Clone, PartialEq)]
pub struct StringExpression<'a> {
    pub value: &'a str,
}

/// A number literal in Pebble. e.g. `1, 3.5, 4e9, 0x03, Ob22, 007`
#[derive(Debug, Clone, PartialEq)]
pub struct NumericExpression<'a> {
    pub value: &'a str,
}

/// An operation that occurs on only one operand. e.g. `!a, ~b`
#[derive(Debug, Clone, PartialEq)]
pub struct UnaryExpression<'a> {
    pub operator: Operator,
    pub operand: Box<Node<'a>>,
}

/// An operation with two operands that yields a new value. e.g `a + b`
#[derive(Debug, Clone, PartialEq)]
pub struct BinaryExpression<'a> {
    pub operator: Operator,
    pub left: Box<Node<'a>>,
    pub right: Box<Node<'a>>,
}

/// A member or dot access of a model. e.g. `a.b`
#[derive(Debug, Clone, PartialEq)]
pub struct DotExpression<'a> {
    pub model: Box<Node<'a>>,
    pub property: Box<Node<'a>>,
}

/// An array in Pebble. e.g. `[a, b, c]`
#[derive(Debug, PartialEq, Clone)]
pub struct ArrayExpression<'a> {
    pub elements: Vec<Box<Node<'a>>>,
}

/// An expression that access an index of an array. e.g `a[b]`.
#[derive(Debug, PartialEq, Clone)]
pub struct AccessExpression<'a> {
    pub sequence: Box<Node<'a>>,
    pub index: Box<Node<'a>>,
}
