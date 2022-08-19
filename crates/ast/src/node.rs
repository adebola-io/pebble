use crate::{
    BinaryExpr, BooleanExpr, Expression, ExpressionStatement, NumericExpr, Operator, Statement,
    StringExpr, TextSpan,
};

#[derive(Debug, Clone, PartialEq)]
pub enum NodeData {
    Expression { kind: Expression },
    Statement { kind: Statement },
    Program { statements: Vec<Box<Node>> },
}

/// A single element on the syntax tree.
#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    /// The range of a node is the width of the text from which the node was created.
    /// It is represented by a two multidemsional array. The first array contains the line and column where the node text starts, while the second array contains the line and column where it ends.
    pub range: TextSpan,
    /// The node data is its node's actual type and data, e.g. A Binary Expression, An If Statement, etc.
    pub data: NodeData,
}

impl Node {
    /// The entry node to an AST tree.
    pub fn program() -> Self {
        Node {
            range: [[0, 0], [0, 0]],
            data: NodeData::Program { statements: vec![] },
        }
    }
    /// Creates an expression statement.
    pub fn expression_statement(range: TextSpan, expression: Self) -> Self {
        Node {
            range,
            data: NodeData::Statement {
                kind: Statement::ExpressionStatement(ExpressionStatement {
                    expression: Box::new(expression),
                }),
            },
        }
    }
    /// Creates a string expression.
    pub fn string_expression(range: TextSpan, value: String) -> Self {
        Node {
            range,
            data: NodeData::Expression {
                kind: Expression::StringExpr(StringExpr { value }),
            },
        }
    }
    /// Creates a number expression.
    pub fn number_expression(range: TextSpan, value: String) -> Self {
        Node {
            range,
            data: NodeData::Expression {
                kind: Expression::NumericExpr(NumericExpr { value }),
            },
        }
    }
    /// Creates a boolean expression.
    pub fn boolean_expression(range: TextSpan, value: bool) -> Self {
        Node {
            range,
            data: NodeData::Expression {
                kind: Expression::BooleanExpr(BooleanExpr { value }),
            },
        }
    }
    // Creates a binary expression.
    pub fn binary_expression(range: TextSpan, left: Self, right: Self, operator: Operator) -> Self {
        Node {
            range,
            data: NodeData::Expression {
                kind: Expression::BinaryExpr(BinaryExpr {
                    left: Box::new(left),
                    right: Box::new(right),
                    operator,
                }),
            },
        }
    }
}
