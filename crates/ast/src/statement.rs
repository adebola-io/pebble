use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    ExpressionStatement(ExpressionStatement),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExpressionStatement {
    pub expression: Box<Node>,
}
