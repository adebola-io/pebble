use crate::compiler::scanner::token::{NumericKind, Token};

use super::{Identifier, Location, NodeRange};

#[derive(Debug, PartialEq)]
pub enum Expression {
    Null,
    Identifier(Identifier),
    Number {
        kind: NumericKind,
        value: String,
        raw: String,
        range: NodeRange,
    },
    BinaryExpression {
        operator: String,
        left: Box<Expression>,
        right: Box<Expression>,
        range: NodeRange,
    },
    AssignmentExpression {
        operator: String,
        left: Identifier,
        right: Box<Expression>,
        range: NodeRange,
    },
}

impl Expression {
    pub fn binary_expression(left_node: Self, operator: String, right_node: Self) -> Self {
        let left_range = left_node.get_range();
        let right_range = right_node.get_range();
        Expression::BinaryExpression {
            operator,
            left: Box::new(left_node),
            right: Box::new(right_node),
            range: [left_range[0], left_range[1], right_range[2], right_range[3]],
        }
    }
    pub fn number(token: Token) -> Self {
        if let Token::Number { kind, raw, loc } = token {
            Expression::Number {
                kind,
                value: raw.clone(),
                raw,
                range: loc,
            }
        } else {
            panic!("Cannot construct node. Expected a number token.")
        }
    }
}

impl Location for Expression {
    fn get_range(&self) -> NodeRange {
        match self {
            Self::Null => [0, 0, 0, 0],
            Self::Identifier(i) => i.get_range(),
            Self::Number { range, .. } => *range,
            Self::BinaryExpression { range, .. } => *range,
            Self::AssignmentExpression { range, .. } => *range,
        }
    }
}
