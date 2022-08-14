use crate::TextRange;

#[derive(Debug, PartialEq, Clone)]
pub struct Comment {
    kind: CommentKind,
    range: TextRange,
    value: String,
}
#[derive(Debug, PartialEq, Clone)]
pub enum CommentKind {
    Block,
    Doc,
    Line,
}
