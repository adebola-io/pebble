use crate::compiler::scanner::token::{NumericKind, Token};

use super::{Location, NodeRange};
#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Expression {
    Null,
    Identifier {
        name: String,
        range: NodeRange,
    },
    String {
        range: NodeRange,
        value: String,
    },
    TemplateString {
        range: NodeRange,
        sequences: Vec<Expression>,
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
    ArrayExpression {
        elements: Vec<Expression>,
        range: NodeRange,
    },
    NewExpression {
        construct: Box<Expression>,
        arguments: Vec<Expression>,
        range: NodeRange,
    },
    UpdateExpression {
        variable: Box<Expression>,
        operator: String,
        range: NodeRange,
    },
    UnaryExpression {
        variable: Box<Expression>,
        operator: String,
        range: NodeRange,
    },
    AccessExpression {
        array: Box<Expression>,
        element: Box<Expression>,
        range: NodeRange,
    },
    TernaryExpression {
        test: Box<Expression>,
        consequent: Box<Expression>,
        alternate: Box<Expression>,
        range: NodeRange,
    },
    NamespaceExpression {
        left: Box<Expression>,
        right: Box<Expression>,
        range: NodeRange,
    },
    SelfExpression {
        range: NodeRange,
    },
    CoreExpression {
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
    pub fn ternary_expression(test: Self, consequent: Self, alternate: Self) -> Self {
        let test_range = test.get_range();
        let alternate_range = alternate.get_range();
        Expression::TernaryExpression {
            test: Box::new(test),
            consequent: Box::new(consequent),
            alternate: Box::new(alternate),
            range: [
                test_range[0],
                test_range[1],
                alternate_range[2],
                alternate_range[3],
            ],
        }
    }
    pub fn namespace_expression(left: Self, right: Self) -> Self {
        let left_range = left.get_range();
        let right_range = right.get_range();
        Expression::NamespaceExpression {
            left: Box::new(left),
            right: Box::new(right),
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
    pub fn new_expression(
        start: [usize; 4],
        construct: Self,
        arguments: Vec<Self>,
        end: [usize; 4],
    ) -> Self {
        Expression::NewExpression {
            construct: Box::new(construct),
            arguments,
            range: [start[0], start[1], end[2], end[3]],
        }
    }
    pub fn array_expression(start: [usize; 4], elements: Vec<Self>, end: [usize; 4]) -> Self {
        Expression::ArrayExpression {
            elements,
            range: [start[0], start[1], end[2], end[3]],
        }
    }
    pub fn unary_expression(variable: Self, operator: Token) -> Self {
        let value_range = variable.get_range();
        if let Token::Operator { value, loc } = operator {
            Expression::UnaryExpression {
                variable: Box::new(variable),
                operator: value,
                range: [loc[0], loc[1], value_range[2], value_range[3]],
            }
        } else {
            panic!("Cannot construct node. Expected an update token.")
        }
    }
    pub fn self_expression(loc: NodeRange) -> Self {
        Self::SelfExpression { range: loc }
    }
    pub fn core_expression(loc: NodeRange) -> Self {
        Self::CoreExpression { range: loc }
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
            | Self::TemplateString { range, .. }
            | Self::String { range, .. }
            | Self::Number { range, .. }
            | Self::Boolean { range, .. }
            | Self::SelfExpression { range }
            | Self::CoreExpression { range, .. }
            | Self::NothingExpression { range }
            | Self::BinaryExpression { range, .. }
            | Self::LogicalExpression { range, .. }
            | Self::NewExpression { range, .. }
            | Self::CallExpression { range, .. }
            | Self::UpdateExpression { range, .. }
            | Self::UnaryExpression { range, .. }
            | Self::MemberExpression { range, .. }
            | Self::AccessExpression { range, .. }
            | Self::ArrayExpression { range, .. }
            | Self::RangeExpression { range, .. }
            | Self::NamespaceExpression { range, .. }
            | Self::TernaryExpression { range, .. }
            | Self::AssignmentExpression { range, .. } => *range,
        }
    }
}
