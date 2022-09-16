#[derive(Debug, PartialEq, Clone)]
pub struct Comment {
    pub kind: CommentKind,
    pub content: String,
}
#[derive(Debug, PartialEq, Clone)]
pub enum CommentKind {
    Block,
    Doc,
    Line,
}
