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
    CallExpression {
        callee: Box<Expression>,
        arguments: Vec<Expression>,
        range: NodeRange,
    },
    BinaryExpression {
        operator: String,
        left: Box<Expression>,
        right: Box<Expression>,
        range: NodeRange,
    },
    LogicalExpression {
        operator: String,
        left: Box<Expression>,
        right: Box<Expression>,
        range: NodeRange,
    },
    RangeExpression {
        lower_boundary: Box<Expression>,
        upper_boundary: Box<Expression>,
        range: NodeRange,
    },
    AssignmentExpression {
        operator: String,
        left: Box<Expression>,
        right: Box<Expression>,
        range: NodeRange,
    },
    UpdateExpression {
        variable: Box<Expression>,
        operator: String,
        range: NodeRange,
    },
    AccessExpression {
        array: Box<Expression>,
        element: Box<Expression>,
        range: NodeRange,
    },
    SelfExpression {
        range: NodeRange,
    },
    NothingExpression {
        range: NodeRange,
    },
    Boolean {
        value: String,
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
    pub fn logical_expression(left_node: Self, operator: String, right_node: Self) -> Self {
        let left_range = left_node.get_range();
        let right_range = right_node.get_range();
        Expression::LogicalExpression {
            operator,
            left: Box::new(left_node),
            right: Box::new(right_node),
            range: [left_range[0], left_range[1], right_range[2], right_range[3]],
        }
    }
    pub fn call_expression(callee: Self, arguments: Vec<Self>, end: NodeRange) -> Self {
        let callee_range = callee.get_range();
        Expression::CallExpression {
            callee: Box::new(callee),
            arguments,
            range: [callee_range[0], callee_range[1], end[2], end[3]],
        }
    }
    pub fn member_expression(object: Self, property: Self) -> Self {
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
    pub fn access_expression(array: Self, element: Self, end: NodeRange) -> Self {
        let array_range = array.get_range();
        Expression::AccessExpression {
            array: Box::new(array),
            element: Box::new(element),
            range: [array_range[0], array_range[1], end[2], end[3]],
        }
    }
    pub fn range_expression(lower_boundary: Self, upper_boundary: Self) -> Self {
        let lower_range = lower_boundary.get_range();
        let upper_range = upper_boundary.get_range();
        Expression::RangeExpression {
            lower_boundary: Box::new(lower_boundary),
            upper_boundary: Box::new(upper_boundary),
            range: [
                lower_range[0],
                lower_range[1],
                upper_range[2],
                upper_range[3],
            ],
        }
    }
    pub fn assignment_expression(left_node: Self, operator: String, right_node: Self) -> Self {
        let left_range = left_node.get_range();
        let right_range = right_node.get_range();
        Expression::AssignmentExpression {
            operator,
            left: Box::new(left_node),
            right: Box::new(right_node),
            range: [left_range[0], left_range[1], right_range[2], right_range[3]],
        }
    }
    pub fn update_expression(variable: Self, operator: Token) -> Self {
        let value_range = variable.get_range();
        if let Token::Operator { value, loc } = operator {
            Expression::UpdateExpression {
                variable: Box::new(variable),
                operator: value,
                range: [value_range[0], value_range[1], loc[2], loc[3]],
            }
        } else {
            panic!("Cannot construct node. Expected an update token.")
        }
    }
    pub fn self_expression(loc: NodeRange) -> Self {
        Self::SelfExpression { range: loc }
    }
    pub fn boolean(value: String, loc: NodeRange) -> Self {
        Self::Boolean { value, range: loc }
    }
    pub fn nil_expression(loc: NodeRange) -> Self {
        Self::NothingExpression { range: loc }
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
            | Self::Boolean { range, .. }
            | Self::SelfExpression { range }
            | Self::NothingExpression { range }
            | Self::BinaryExpression { range, .. }
            | Self::LogicalExpression { range, .. }
            | Self::CallExpression { range, .. }
            | Self::UpdateExpression { range, .. }
            | Self::MemberExpression { range, .. }
            | Self::AccessExpression { range, .. }
            | Self::RangeExpression { range, .. }
            | Self::AssignmentExpression { range, .. } => *range,
        }
    }
}
