use super::{expression, Expression, Location, NodeRange};
#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Statement {
    EmptyStatement {
        range: NodeRange,
    },
    ExpressionStatement {
        expression: Expression,
        range: NodeRange,
    },
}

impl Location for Statement {
    fn get_range(&self) -> NodeRange {
        match self {
            Self::ExpressionStatement { range, .. } | Self::EmptyStatement { range } => *range,
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
