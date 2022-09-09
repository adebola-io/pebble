use macros::Location;

use crate::{Block, GenericArgument, Location, Operator, Parameter, TextSpan, Type};

/// The base node for an expression.
#[derive(Debug, Clone, PartialEq)]
pub enum Expression<'a> {
    IdentifierExpression(Identifier<'a>),
    StringExpression(TextString<'a>),
    NumericExpression(Number<'a>),
    // A boolean literal, i.e. `true` or `false`
    BooleanExpression(Boolean<'a>),
    CharacterExpression(Character<'a>),
    /// An operation that occurs on two operands e.g. `a + b`
    BinaryExpr {
        operator: &'a Operator,
        left: Box<Self>,
        right: Box<Self>,
        span: TextSpan,
    },
    /// An operation expressiong a logical operation, e.g. `a || b`
    LogicalExpr {
        operator: &'a Operator,
        left: Box<Self>,
        right: Box<Self>,
        span: TextSpan,
    },
    /// An operation that occurs on only one operand. e.g. `!a, ~b`
    UnaryExpr {
        operator: &'a Operator,
        operand: Box<Self>,
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
        elements: Vec<Self>,
        span: TextSpan,
    },
    /// An expression that access an index of an array. e.g `a[b]`.
    IndexExpr {
        accessor: Box<Self>,
        property: Box<Self>,
        span: TextSpan,
    },
    /// A member or dot access of a class. e.g. `a.b`
    DotExpr {
        object: Box<Self>,
        property: Box<Self>,
        span: TextSpan,
    },
    NamespaceExpr {
        object: Box<Self>,
        property: Box<Self>,
        span: TextSpan,
    },
    /// An expression that expresses a numeric or alphabetic range. e.g. `a..b`
    RangeExpr {
        top: Box<Self>,
        bottom: Box<Self>,
        span: TextSpan,
    },
    TernaryExpr {
        test: Box<Self>,
        consequent: Box<Self>,
        alternate: Box<Self>,
        span: TextSpan,
    },
    AssignmentExpr {
        left: Box<Self>,
        right: Box<Self>,
        operator: &'a Operator,
        span: TextSpan,
    },
    /// A functional expression.
    FnExpression(FnExpression<'a>),
}

/// An expression consisting of a single identifier.
#[derive(Location, Debug, Clone, PartialEq)]
pub struct Identifier<'a> {
    pub value: &'a str,
    pub span: TextSpan,
}

/// A string literal in Pebble. e.g. `"John Doe", "One does not simply walk into Mordor"`
#[derive(Location, Debug, Clone, PartialEq)]
pub struct TextString<'a> {
    pub value: &'a str,
    pub span: TextSpan,
}

/// A number literal in Pebble. e.g. `1, 3.5, 4e9, 0x03, Ob22, 007`
#[derive(Location, Debug, Clone, PartialEq)]
pub struct Number<'a> {
    pub value: &'a str,
    pub span: TextSpan,
}

/// A number literal in Pebble. e.g. `1, 3.5, 4e9, 0x03, Ob22, 007`
#[derive(Location, Debug, Clone, PartialEq)]
pub struct Boolean<'a> {
    pub value: &'a str,
    pub span: TextSpan,
}

#[derive(Location, Debug, Clone, PartialEq)]
pub struct FnExpression<'a> {
    pub labels: Option<Vec<GenericArgument<'a>>>,
    pub parameters: Vec<Parameter<'a>>,
    pub return_type: Option<Type<'a>>,
    pub body: Option<Block<'a>>,
    /// A functional expression may consist only of its return expression.
    pub implicit_return: Option<Box<Expression<'a>>>,
    pub span: TextSpan,
}

#[derive(Location, Debug, Clone, PartialEq)]
pub struct Character<'a> {
    pub value: &'a str,
    pub span: TextSpan,
}

impl<'a> Expression<'a> {
    /// Returns `true` if the expression is [`IdentifierExpr`].
    ///
    /// [`IdentifierExpr`]: Expression::IdentifierExpr
    pub fn is_identifier_expr(&self) -> bool {
        matches!(self, Self::IdentifierExpression(_))
    }

    /// Returns `true` if the expression is [`StringExpr`].
    ///
    /// [`StringExpr`]: Expression::StringExpr
    pub fn is_string_expr(&self) -> bool {
        matches!(self, Self::StringExpression(_))
    }

    /// Returns `true` if the expression is [`NumericExpr`].
    ///
    /// [`NumericExpr`]: Expression::NumericExpr
    pub fn is_numeric_expr(&self) -> bool {
        matches!(self, Self::NumericExpression(_))
    }

    /// Returns `true` if the expression is [`BooleanExpr`].
    ///
    /// [`BooleanExpr`]: Expression::BooleanExpr
    pub fn is_boolean_expr(&self) -> bool {
        matches!(self, Self::BooleanExpression(_))
    }

    /// Returns `true` if the expression is [`CharacterExpr`].
    ///
    /// [`CharacterExpr`]: Expression::CharacterExpr
    pub fn is_character_expr(&self) -> bool {
        matches!(self, Self::CharacterExpression(_))
    }
}

impl<'a> Expression<'a> {
    /// Creates am identifier expression node.
    pub fn create_ident_expr(value: &'a str, span: TextSpan) -> Self {
        Expression::IdentifierExpression(Identifier { value, span })
    }
    /// Creates a string expression node.
    pub fn create_str_expr(value: &'a str, span: TextSpan) -> Self {
        Expression::StringExpression(TextString { value, span })
    }
    /// Creates a numeric expression node.
    pub fn create_num_expr(value: &'a str, span: TextSpan) -> Self {
        Expression::NumericExpression(Number { value, span })
    }
    /// Creates a boolean expression node.
    pub fn create_bool_expr(value: &'a str, span: TextSpan) -> Self {
        Expression::BooleanExpression(Boolean { value, span })
    }
    /// Creates a character expression node.
    pub fn create_char_expr(value: &'a str, span: TextSpan) -> Self {
        Expression::CharacterExpression(Character { value, span })
    }
    /// Creates a dot expression.
    pub fn create_dot_expr(object: Self, property: Self) -> Self {
        let span = [object.get_range()[0], property.get_range()[1]];
        Expression::DotExpr {
            object: Box::new(object),
            property: Box::new(property),
            span,
        }
    }
    /// Creates a namespace expression.
    pub fn create_namespace_expr(object: Self, property: Self) -> Self {
        let span = [object.get_range()[0], property.get_range()[1]];
        Expression::NamespaceExpr {
            object: Box::new(object),
            property: Box::new(property),
            span,
        }
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
    pub fn create_call_expr(callee: Self, arguments: Vec<Self>, end: [u64; 2]) -> Self {
        let start = callee.get_range()[0];
        Expression::CallExpr {
            callee: Box::new(callee),
            arguments,
            span: [start, end],
        }
    }
    /// Creates a index expression.
    pub fn create_index_expr(accessor: Self, property: Self, end: [u64; 2]) -> Self {
        let span = [accessor.get_range()[0], end];
        Expression::IndexExpr {
            accessor: Box::new(accessor),
            property: Box::new(property),
            span,
        }
    }
    /// Creates a unary expression.
    pub fn create_unary_expr(start: [u64; 2], operator: &'a Operator, operand: Self) -> Self {
        let span = [start, operand.get_range()[1]];
        Expression::UnaryExpr {
            operator,
            operand: Box::new(operand),
            span,
        }
    }
    /// Creates a range expression.
    pub fn create_range_expr(top: Self, bottom: Self) -> Self {
        let span = [top.get_range()[0], bottom.get_range()[1]];
        Expression::RangeExpr {
            top: Box::new(top),
            bottom: Box::new(bottom),
            span,
        }
    }
    /// Creates a logical expression.
    pub fn create_logical_expr(left: Self, operator: &'a Operator, right: Self) -> Self {
        let span = [left.get_range()[0], right.get_range()[1]];
        Expression::LogicalExpr {
            operator,
            left: Box::new(left),
            right: Box::new(right),

            span,
        }
    }
    /// Creates a ternary expression.
    pub fn create_ternary_expr(test: Self, consequent: Self, alternate: Self) -> Self {
        let span = [test.get_range()[0], alternate.get_range()[1]];
        Expression::TernaryExpr {
            test: Box::new(test),
            consequent: Box::new(consequent),
            alternate: Box::new(alternate),
            span,
        }
    }
    /// Creates an assignment expression.
    pub fn create_assign_expr(left: Self, operator: &'a Operator, right: Self) -> Self {
        let span = [left.get_range()[0], right.get_range()[1]];
        Expression::AssignmentExpr {
            operator,
            left: Box::new(left),
            right: Box::new(right),
            span,
        }
    }
}

impl Location for Expression<'_> {
    fn get_range(&self) -> TextSpan {
        match self {
            Self::IdentifierExpression(Identifier { span, .. })
            | Self::StringExpression(TextString { span, .. })
            | Self::NumericExpression(Number { span, .. })
            | Self::BooleanExpression(Boolean { span, .. })
            | Self::CharacterExpression(Character { span, .. })
            | Self::BinaryExpr { span, .. }
            | Self::LogicalExpr { span, .. }
            | Self::UnaryExpr { span, .. }
            | Self::CallExpr { span, .. }
            | Self::ArrayExpr { span, .. }
            | Self::IndexExpr { span, .. }
            | Self::DotExpr { span, .. }
            | Self::NamespaceExpr { span, .. }
            | Self::RangeExpr { span, .. }
            | Self::TernaryExpr { span, .. }
            | Self::AssignmentExpr { span, .. }
            | Self::FnExpression(FnExpression { span, .. }) => *span,
        }
    }
}
