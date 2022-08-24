use crate::{Location, Operator, TextSpan};

/// The base node for an expression.
#[derive(Debug, Clone, PartialEq)]
pub enum Expression<'a> {
    /// An expression consisting of a single identifier.
    IdentifierExpr {
        value: &'a str,
        span: TextSpan,
    },
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
        operator: &'a Operator,
        left: Box<Self>,
        right: Box<Self>,
        span: TextSpan,
    },
    /// An operation that occurs on only one operand. e.g. `!a, ~b`
    UnaryExpr {
        operator: Operator,
        operand: &'a Self,
        span: TextSpan,
    },
    /// A function call expression. e.g. `a(b)`.
    CallExpr {
        callee: Box<Self>,
        arguments: Vec<Self>,
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
    /// A member or dot access of a class. e.g. `a.b`
    DotExpr {
        class: &'a Self,
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
    /// Creates am identifier expression node.
    pub fn create_ident_expr(value: &'a str, span: TextSpan) -> Self {
        Expression::IdentifierExpr { value, span }
    }
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
    /// Creates a binary expression.
    pub fn create_bin_expr(left: Self, operator: &'a Operator, right: Self) -> Self {
        let span = [left.get_range()[0], right.get_range()[1]];
        Expression::BinaryExpr {
            operator,
            left: Box::new(left),
            right: Box::new(right),
            span,
        }
    }
    /// Creates a call expression.
    pub fn create_call_expr(
        callee: Expression<'a>,
        arguments: Vec<Expression<'a>>,
        end: [u64; 2],
    ) -> Self {
        let start = callee.get_range()[0];
        Expression::CallExpr {
            callee: Box::new(callee),
            arguments,
            span: [start, end],
        }
    }
}

impl Location for Expression<'_> {
    fn get_range(&self) -> TextSpan {
        match self {
            Self::IdentifierExpr { span, .. }
            | Self::StringExpr { span, .. }
            | Self::NumericExpr { span, .. }
            | Self::BooleanExpr { span, .. }
            | Self::CharacterExpr { span, .. }
            | Self::BinaryExpr { span, .. }
            | Self::UnaryExpr { span, .. }
            | Self::CallExpr { span, .. }
            | Self::ArrayExpr { span, .. }
            | Self::AccessExpr { span, .. }
            | Self::DotExpr { span, .. }
            | Self::RangeExpr { span, .. } => *span,
        }
    }
}
