use crate::compiler::scanner::token::{NumericKind, Token};

use super::{Location, NodeRange};

#[derive(Debug, PartialEq)]
pub enum Expression {
    Null,
    Identifier {
        name: String,
        range: NodeRange,
    },
    Number {
        kind: NumericKind,
        value: String,
        raw: String,
        range: NodeRange,
    },
    MemberExpression {
        object: Box<Expression>,
        property: Box<Expression>,
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
        left: Box<Expression>,
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
    pub fn member_expression(object: Expression, property: Expression) -> Self {
        let object_range = object.get_range();
        let property_range = property.get_range();
        Expression::MemberExpression {
            object: Box::new(object),
            property: Box::new(property),
            range: [
                object_range[0],
                object_range[1],
                property_range[2],
                property_range[3],
            ],
        }
    }
    pub fn identifier(token: Token) -> Self {
        if let Token::Identifier { value, loc } = token {
            Expression::Identifier {
                name: value,
                range: loc,
            }
        } else {
            panic!("Cannot construct node. Expected an identifier token.")
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
            Self::Identifier { range, .. }
            | Self::Number { range, .. }
            | Self::BinaryExpression { range, .. }
            | Self::MemberExpression { range, .. }
            | Self::AssignmentExpression { range, .. } => *range,
        }
    }
}
