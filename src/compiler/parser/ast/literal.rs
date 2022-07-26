use crate::compiler::scanner::token::NumericKind;

use super::{Expression, Location, NodeRange};

#[derive(Debug, PartialEq)]
pub enum Literal {
    Boolean {
        value: String,
        range: NodeRange,
    },
    Number {
        kind: NumericKind,
        value: String,
        raw: String,
        range: NodeRange,
    },
    String {
        expressions: Vec<StringExpressions>,
        range: NodeRange,
    },
}
impl Location for Literal {
    fn get_range(&self) -> NodeRange {
        match self {
            Self::Boolean { range, .. }
            | Self::Number { range, .. }
            | Self::String { range, .. } => range.clone(),
        }
    }
}
#[derive(Debug, PartialEq)]
pub enum StringExpressions {
    Sequence,
    Expression(Expression),
}
