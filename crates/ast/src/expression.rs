use crate::{Location, Operator, TextSpan};

/// The base node for an expression.
#[derive(Debug, Clone, PartialEq)]
pub enum Expression<'a> {
    /// A string literal in Pebble. e.g. `"John Doe", "One does not simply walk into Mordor"`
    StringExpr {
        value: &'a str,
        span: TextSpan,
    },
    /// A number literal in Pebble. e.g. `1, 3.5, 4e9, 0x03, Ob22, 007`
    NumericExpr {
        value: &'a str,
        span: TextSpan,
    },
    // A boolean literal, i.e. `true` or `false`
    BooleanExpr {
        value: bool,
        span: TextSpan,
    },
    CharacterExpr {
        value: &'a str,
        span: TextSpan,
    },
    /// An operation that occurs on two operands e.g. `a + b`
    BinaryExpr {
        operator: Operator,
        left: &'a Self,
        right: &'a Self,
        span: TextSpan,
    },
    /// An operation that occurs on only one operand. e.g. `!a, ~b`
    UnaryExpr {
        operator: Operator,
        operand: &'a Self,
        span: TextSpan,
    },
    /// An array of expression. e.g. `[a, b, c]`
    ArrayExpr {
        elements: Vec<&'a Self>,
        span: TextSpan,
    },
    /// An expression that access an index of an array. e.g `a[b]`.
    AccessExpr {
        accessor: &'a Self,
        property: &'a Self,
        span: TextSpan,
    },
    /// A member or dot access of a model. e.g. `a.b`
    DotExpr {
        model: &'a Self,
        property: &'a Self,
        span: TextSpan,
    },
    /// An expression that expresses a numeric or alphabetic range. e.g. `a..b`
    RangeExpr {
        top: &'a Self,
        bottom: &'a Self,
        span: TextSpan,
    },
}

impl<'a> Expression<'a> {
    /// Creates a string expression node.
    pub fn create_str_expr(value: &'a str, span: TextSpan) -> Self {
        Expression::StringExpr { value, span }
    }
    /// Creates a numeric expression node.
    pub fn create_num_expr(value: &'a str, span: TextSpan) -> Self {
        Expression::NumericExpr { value, span }
    }
    /// Creates a boolean expression node.
    pub fn create_bool_expr(value: &'a str, span: TextSpan) -> Self {
        Expression::BooleanExpr {
            value: if value == "true" { true } else { false },
            span,
        }
    }
    /// Creates a character expression node.
    pub fn create_char_expr(value: &'a str, span: TextSpan) -> Self {
        Expression::CharacterExpr { value, span }
    }
}

impl Location for Expression<'_> {
    fn get_range(&self) -> TextSpan {
        match self {
            Expression::StringExpr { span, .. }
            | Expression::NumericExpr { span, .. }
            | Expression::BooleanExpr { span, .. }
            | Expression::CharacterExpr { span, .. }
            | Expression::BinaryExpr { span, .. }
            | Expression::UnaryExpr { span, .. }
            | Expression::ArrayExpr { span, .. }
            | Expression::AccessExpr { span, .. }
            | Expression::DotExpr { span, .. }
            | Expression::RangeExpr { span, .. } => *span,
        }
    }
}
