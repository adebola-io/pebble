use macros::Location;

use crate::{Block, GenericArgument, Location, Operator, Parameter, TextSpan, Type};

/// The base node for an expression.
#[derive(Debug, Clone, PartialEq)]
pub enum Expression<'a> {
    IdentifierExpression(Identifier<'a>),
    StringExpression(TextString<'a>),
    NumericExpression(Number<'a>),
    BooleanExpression(Boolean<'a>),
    CharacterExpression(Character<'a>),
    SelfExpression(SelfExpression),
    BinaryExpression(BinaryExpression<'a>),
    LogicalExpression(LogicalExpression<'a>),
    UnaryExpression(UnaryExpression<'a>),
    CallExpression(CallExpression<'a>),
    ArrayExpression(ArrayExpression<'a>),
    IndexExpression(IndexExpression<'a>),
    DotExpression(DotExpression<'a>),
    NamespaceExpression(NamespaceExpression<'a>),
    RangeExpression(RangeExpression<'a>),
    TernaryExpression(TernaryExpression<'a>),
    AssignmentExpression(AssignmentExpression<'a>),
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

// A boolean literal, i.e. `true` or `false`
#[derive(Location, Debug, Clone, PartialEq)]
pub struct Boolean<'a> {
    pub value: &'a str,
    pub span: TextSpan,
}

/// An operation that occurs on two operands e.g. `a + b`
#[derive(Location, Debug, Clone, PartialEq)]
pub struct BinaryExpression<'a> {
    pub operator: &'a Operator,
    pub operands: Vec<Expression<'a>>,
    pub span: TextSpan,
}

/// An operation expressiong a logical operation, e.g. `a || b`
#[derive(Location, Debug, Clone, PartialEq)]
pub struct LogicalExpression<'a> {
    pub operator: &'a Operator,
    pub operands: Vec<Expression<'a>>,
    pub span: TextSpan,
}

/// An operation that occurs on only one operand. e.g. `!a, ~b`
#[derive(Location, Debug, Clone, PartialEq)]
pub struct UnaryExpression<'a> {
    pub operator: &'a Operator,
    pub operand: Box<Expression<'a>>,
    pub span: TextSpan,
}

/// A reference value to the current class instance.
#[derive(Debug, Clone, PartialEq)]
pub struct SelfExpression {
    pub span: TextSpan,
}

impl Location for SelfExpression {
    fn get_range(&self) -> TextSpan {
        self.span
    }
}

/// A function call expression. e.g. `a(b)`.
#[derive(Location, Debug, Clone, PartialEq)]
pub struct CallExpression<'a> {
    pub callee: Box<Expression<'a>>,
    pub arguments: Vec<Expression<'a>>,
    pub span: TextSpan,
}

/// A member or dot access of a class. e.g. `a.b`
#[derive(Location, Debug, Clone, PartialEq)]
pub struct DotExpression<'a> {
    pub object: Box<Expression<'a>>,
    pub property: Box<Expression<'a>>,
    pub span: TextSpan,
}

#[derive(Location, Debug, Clone, PartialEq)]
pub struct NamespaceExpression<'a> {
    pub object: Box<Expression<'a>>,
    pub property: Box<Expression<'a>>,
    pub span: TextSpan,
}

/// An expression that expresses a numeric or alphabetic range. e.g. `a..b`
#[derive(Location, Debug, Clone, PartialEq)]
pub struct RangeExpression<'a> {
    pub boundaries: Vec<Expression<'a>>,
    pub span: TextSpan,
}

#[derive(Location, Debug, Clone, PartialEq)]
pub struct TernaryExpression<'a> {
    pub test: Box<Expression<'a>>,
    pub consequent: Box<Expression<'a>>,
    pub alternate: Box<Expression<'a>>,
    pub span: TextSpan,
}

/// An array of expression. e.g. `[a, b, c]`
#[derive(Location, Debug, Clone, PartialEq)]
pub struct ArrayExpression<'a> {
    pub elements: Vec<Expression<'a>>,
    pub span: TextSpan,
}

#[derive(Location, Debug, Clone, PartialEq)]
pub struct AssignmentExpression<'a> {
    pub operands: Vec<Expression<'a>>,
    pub operator: &'a Operator,
    pub span: TextSpan,
}

/// An expression that access an index of an array. e.g `a[b]`.
#[derive(Location, Debug, Clone, PartialEq)]
pub struct IndexExpression<'a> {
    pub accessor_and_property: Vec<Expression<'a>>,
    pub span: TextSpan,
}

/// A functional expression.
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

    /// Returns true if the expression is a literal.
    pub fn is_literal(&self) -> bool {
        if let Self::StringExpression(_)
        | Self::BooleanExpression(_)
        | Self::NumericExpression(_)
        | Self::CharacterExpression(_) = self
        {
            true
        } else {
            false
        }
    }
    /// Check if an expression is a valid left hand side assignment target.
    pub fn is_valid_assignment_target(&self) -> bool {
        match self {
            Expression::IdentifierExpression(_) => true,
            Expression::SelfExpression(_) => true,
            Expression::IndexExpression(indexexpr) => {
                indexexpr.accessor_and_property[0].is_valid_assignment_target()
                    && (indexexpr.accessor_and_property[1].is_numeric_expr()
                        || indexexpr.accessor_and_property[1].is_valid_assignment_target())
            }
            Expression::DotExpression(dotexpr) => {
                dotexpr.object.is_valid_assignment_target()
                    && dotexpr.property.is_valid_assignment_target()
            }
            _ => false,
        }
    }
}

impl<'a> Expression<'a> {
    /// Creates an identifier expression node.
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
        Expression::DotExpression(DotExpression {
            object: Box::new(object),
            property: Box::new(property),
            span,
        })
    }
    /// Creates a namespace expression.
    pub fn create_namespace_expr(object: Self, property: Self) -> Self {
        let span = [object.get_range()[0], property.get_range()[1]];
        Expression::NamespaceExpression(NamespaceExpression {
            object: Box::new(object),
            property: Box::new(property),
            span,
        })
    }
    /// Creates a binary expression.
    pub fn create_bin_expr(left: Self, operator: &'a Operator, right: Self) -> Self {
        let span = [left.get_range()[0], right.get_range()[1]];
        Expression::BinaryExpression(BinaryExpression {
            operator,
            operands: vec![left, right],
            span,
        })
    }
    /// Creates a call expression.
    pub fn create_call_expr(callee: Self, arguments: Vec<Self>, end: [u64; 2]) -> Self {
        let start = callee.get_range()[0];
        Expression::CallExpression(CallExpression {
            callee: Box::new(callee),
            arguments,
            span: [start, end],
        })
    }
    /// Creates a index expression.
    pub fn create_index_expr(accessor: Self, property: Self, end: [u64; 2]) -> Self {
        let span = [accessor.get_range()[0], end];
        Expression::IndexExpression(IndexExpression {
            accessor_and_property: vec![accessor, property],
            span,
        })
    }
    /// Creates a unary expression.
    pub fn create_unary_expr(start: [u64; 2], operator: &'a Operator, operand: Self) -> Self {
        let span = [start, operand.get_range()[1]];
        Expression::UnaryExpression(UnaryExpression {
            operator,
            operand: Box::new(operand),
            span,
        })
    }
    /// Creates a range expression.
    pub fn create_range_expr(top: Self, bottom: Self) -> Self {
        let span = [top.get_range()[0], bottom.get_range()[1]];
        Expression::RangeExpression(RangeExpression {
            boundaries: vec![top, bottom],
            span,
        })
    }
    /// Creates a logical expression.
    pub fn create_logical_expr(left: Self, operator: &'a Operator, right: Self) -> Self {
        let span = [left.get_range()[0], right.get_range()[1]];
        Expression::LogicalExpression(LogicalExpression {
            operator,
            operands: vec![left, right],
            span,
        })
    }
    /// Creates a ternary expression.
    pub fn create_ternary_expr(test: Self, consequent: Self, alternate: Self) -> Self {
        let span = [test.get_range()[0], alternate.get_range()[1]];
        Expression::TernaryExpression(TernaryExpression {
            test: Box::new(test),
            consequent: Box::new(consequent),
            alternate: Box::new(alternate),
            span,
        })
    }
    /// Creates an assignment expression.
    pub fn create_assign_expr(left: Self, operator: &'a Operator, right: Self) -> Self {
        let span = [left.get_range()[0], right.get_range()[1]];
        Expression::AssignmentExpression(AssignmentExpression {
            operator,
            operands: vec![left, right],
            span,
        })
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
            | Self::SelfExpression(SelfExpression { span })
            | Self::BinaryExpression(BinaryExpression { span, .. })
            | Self::LogicalExpression(LogicalExpression { span, .. })
            | Self::UnaryExpression(UnaryExpression { span, .. })
            | Self::CallExpression(CallExpression { span, .. })
            | Self::ArrayExpression(ArrayExpression { span, .. })
            | Self::IndexExpression(IndexExpression { span, .. })
            | Self::DotExpression(DotExpression { span, .. })
            | Self::NamespaceExpression(NamespaceExpression { span, .. })
            | Self::RangeExpression(RangeExpression { span, .. })
            | Self::TernaryExpression(TernaryExpression { span, .. })
            | Self::AssignmentExpression(AssignmentExpression { span, .. })
            | Self::FnExpression(FnExpression { span, .. }) => *span,
        }
    }
}
