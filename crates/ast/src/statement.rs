use crate::{Expression, Location, TextSpan};

#[derive(Debug, Clone, PartialEq)]
pub enum Statement<'a> {
    /// A generic if statement, as it is in other C derived languages. e.g.
    /// ```pebble
    /// if (is_true) {
    ///     doStuff();
    /// } else {
    ///     doOtherStuff();
    /// }
    /// ```
    /// As with javascript, the blocks can be replaced with a single statement, and the else is optional.
    IfStmnt {
        test: Expression<'a>,
        body: &'a Statement<'a>,
        alternate: Option<Vec<Statement<'a>>>,
        span: TextSpan,
    },
    /// A loop statement, with the form:
    /// ```pebble
    /// loop (10) {
    ///     doStuff();
    /// }
    /// ```
    /// The above loop runs the function `doStuff()` 10 times.<br>
    /// To create an infinite loop the constraint can be omitted.
    LoopStmnt {
        constraint: Expression<'a>,
        body: &'a Statement<'a>,
        span: TextSpan,
    },
    /// A while statement, with the form:
    /// ```pebble
    /// while (is_true) {
    ///     doStuff();
    /// }
    /// ```
    WhileStmnt {
        test: Expression<'a>,
        body: &'a Statement<'a>,
        span: TextSpan,
    },
    /// Any expression statement.
    ExprStmnt {
        expression: Expression<'a>,
        span: TextSpan,
    },
    /// A block statement.
    BlockStmnt {
        statements: Vec<Statement<'a>>,
        span: TextSpan,
    },
}

impl<'a> Statement<'a> {
    pub fn create_expression_statement(expression: Expression<'a>) -> Self {
        let span = expression.get_range();
        Statement::ExprStmnt { expression, span }
    }
}

impl<'a> Location for Statement<'a> {
    fn get_range(&self) -> TextSpan {
        match self {
            Statement::IfStmnt { span, .. }
            | Statement::WhileStmnt { span, .. }
            | Statement::LoopStmnt { span, .. }
            | Statement::ExprStmnt { span, .. }
            | Statement::BlockStmnt { span, .. } => *span,
        }
    }
}
