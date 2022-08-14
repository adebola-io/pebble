use crate::{BinaryExpression, DotExpression, NumericExpression, StringExpression};

pub type NodeRange = [usize; 4];

#[derive(Debug, Clone, PartialEq)]
pub enum NodeData<'a> {
    StringExpression(StringExpression<'a>),
    NumericExpression(NumericExpression<'a>),
    BinaryExpression(BinaryExpression<'a>),
    DotExpression(DotExpression<'a>),
}

/// A single element on the syntax tree.
#[derive(Debug, Clone, PartialEq)]
pub struct Node<'a> {
    /// The range of a node is the width of the text from which the node was created.
    /// It is represented by an array of 4 numbers. The first two are the line and column where the node text starts, while the last two are the line and column where it ends.
    range: NodeRange,
    /// The node data is its node's actual type and data, e.g. A Binary Expression, An If Statement, etc.
    data: NodeData<'a>,
}
