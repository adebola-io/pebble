use crate::{Expression, Location, TextSpan, Type};

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
    /// As with Javascript, the blocks can be replaced with a single statement, and the else is optional.
    IfStmnt {
        test: Expression<'a>,
        body: Box<Self>,
        alternate: Option<Box<Self>>,
        span: TextSpan,
    },
    /// A statement that prints to the standard output. e.g.
    /// ```pebble
    /// println "Hello, world!";
    /// ```
    PrintLnStmnt {
        argument: Expression<'a>,
        span: TextSpan,
    },
    /// A statement that concatenates the content of another file to the top of a file. e.g.
    /// ```pebble
    /// @prepend "./otherfile.peb";
    /// ```
    PrependStmnt {
        source: Expression<'a>,
        span: TextSpan,
    },
    /// A variable declaration.
    /// ```pebble
    /// @let name: String = "johnny";
    /// ```
    LetStmnt {
        identifier: Expression<'a>,
        initializer: Option<Expression<'a>>,
        type_label: Option<Type>,
        span: TextSpan,
    },
    /// A break statement that halts a loop.
    BreakStmnt { span: TextSpan },
    /// A testing block, i.e. a group of functions for testing code functionality. e.g.
    /// ```pebble
    /// @tests {
    ///     @function it_adds() {
    ///         core.assert(2 + 2, 4);
    ///     }
    /// }
    /// ```
    TestBlock { body: Box<Self>, span: TextSpan },
    /// A loop statement, with the form:
    /// ```pebble
    /// loop (10) {
    ///     doStuff();
    /// }
    /// ```
    /// The above loop runs the function `doStuff()` 10 times.
    /// To create an infinite loop the constraint can be omitted.
    LoopStmnt {
        constraint: Option<Expression<'a>>,
        body: Box<Self>,
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
        body: Box<Self>,
        span: TextSpan,
    },
    /// Any expression statement.
    ExprStmnt {
        expression: Expression<'a>,
        span: TextSpan,
    },
    /// A block statement. e.g.
    /// ```pebble
    /// {
    ///     print "This is a block statement.";
    /// }
    /// ```
    BlockStmnt {
        statements: Vec<Self>,
        span: TextSpan,
    },
    /// A return statement.
    /// ```pebble
    /// return x;
    /// ```
    ReturnStmnt {
        argument: Option<Expression<'a>>,
        span: TextSpan,
    },
}

impl<'a> Statement<'a> {
    pub fn create_expr_stmnt(expression: Expression<'a>) -> Self {
        let span = expression.get_range();
        Statement::ExprStmnt { expression, span }
    }
}

impl<'a> Location for Statement<'a> {
    fn get_range(&self) -> TextSpan {
        match self {
            Self::IfStmnt { span, .. }
            | Self::WhileStmnt { span, .. }
            | Self::LoopStmnt { span, .. }
            | Self::PrependStmnt { span, .. }
            | Self::PrintLnStmnt { span, .. }
            | Self::ExprStmnt { span, .. }
            | Self::LetStmnt { span, .. }
            | Self::BreakStmnt { span, .. }
            | Self::TestBlock { span, .. }
            | Self::BlockStmnt { span, .. }
            | Self::ReturnStmnt { span, .. } => *span,
        }
    }
}
