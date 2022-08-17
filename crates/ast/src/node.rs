use crate::{Expression, TextSpan};

#[derive(Debug, Clone, PartialEq)]
pub enum NodeData {
    Expression { kind: Expression },
    Program { statements: Vec<Box<Node>> },
}

/// A single element on the syntax tree.
#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    /// The range of a node is the width of the text from which the node was created.
    /// It is represented by a two multidemsional array. The first array contains the line and column where the node text starts, while the second array contains the line and column where it ends.
    range: TextSpan,
    /// The node data is its node's actual type and data, e.g. A Binary Expression, An If Statement, etc.
    data: NodeData,
}

impl Node {
    /// The entry node to an AST tree.
    pub fn program() -> Self {
        Node {
            range: [[0, 0], [0, 0]],
            data: NodeData::Program { statements: vec![] },
        }
    }
}
