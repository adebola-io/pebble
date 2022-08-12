use super::{Block, Expression, Location, NodeRange};
#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Statement {
    EmptyStatement {
        range: NodeRange,
    },
    BlockStatement {
        statements: Vec<Statement>,
        range: NodeRange,
    },
    IfStatement {
        test: Expression,
        body: Box<Statement>,
        alternate: Box<Option<Statement>>,
        range: NodeRange,
    },
    WhileStatement {
        test: Expression,
        body: Box<Statement>,
        range: NodeRange,
    },
    DoWhileStatement {
        test: Expression,
        body: Box<Statement>,
        range: NodeRange,
    },
    PrintLnStatement {
        argument: Expression,
        range: NodeRange,
    },
    MatchStatement {
        discriminant: Expression,
        cases: Vec<MatchCase>,
        range: NodeRange,
    },
    ExpressionStatement {
        expression: Expression,
        range: NodeRange,
    },
}
#[derive(Debug, PartialEq)]
pub struct MatchCase {
    pattern: Pattern,
    consequent: Statement,
}
#[derive(Debug, PartialEq)]
pub struct Pattern {}

impl Location for Statement {
    fn get_range(&self) -> NodeRange {
        match self {
            Self::ExpressionStatement { range, .. }
            | Self::BlockStatement { range, .. }
            | Self::WhileStatement { range, .. }
            | Self::DoWhileStatement { range, .. }
            | Self::PrintLnStatement { range, .. }
            | Self::MatchStatement { range, .. }
            | Self::EmptyStatement { range }
            | Self::IfStatement { range, .. } => *range,
        }
    }
}
impl Statement {
    /// Create an expression statement node.
    pub fn expression_statement(expression: Expression, end: NodeRange) -> Self {
        let range = expression.get_range();
        Self::ExpressionStatement {
            expression,
            range: [range[0], range[1], end[2], end[3]],
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Declaration {
    RecordDeclaration,
    ConstantDeclaration,
    FunctionDeclaration,
    PrependDeclaration,
    UseDeclaration,
    EnumDeclaration,
    StructDeclaration,
    InterfaceDeclaration,
    TypeDeclaration,
    VariableDeclaration,
}
