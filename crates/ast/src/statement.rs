use crate::{Expression, Location, TextSpan};

#[derive(Debug, Clone, PartialEq)]
pub enum Statement<'a> {
    /// A generic if statement, as it is in other C derived languages. e.g. `if (a) {b} else {c}`. The blocks are optional.
    IfStmnt {
        test: Expression<'a>,
        body: &'a Statement<'a>,
        alternate: Option<Vec<Statement<'a>>>,
        span: TextSpan,
    },
    LoopStmnt {
        constraint: Expression<'a>,
        body: &'a Statement<'a>,
        span: TextSpan,
    },
    WhileStmnt {
        test: Expression<'a>,
        body: &'a Statement<'a>,
        span: TextSpan,
    },
    ExprStmnt {
        expression: Expression<'a>,
        span: TextSpan,
    },
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
