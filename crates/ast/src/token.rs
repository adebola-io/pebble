use crate::{
    identifier::Literal, Comment, CommentKind, Identifier, Keyword, Operator, Punctuation, TextSpan,
};

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind<'a> {
    Operator(Operator),
    Punctuation(Punctuation),
    Keyword(Keyword),
    Comment(Comment),
    Literal(Literal<'a>),
    Identifier(Identifier<'a>),
    Invalid { value: &'a str },
}

/// A piece of code collected when scanning the input source file.
#[derive(Debug, PartialEq, Clone)]
pub struct Token<'a> {
    pub kind: TokenKind<'a>,
    pub span: TextSpan,
}

impl<'a> Token<'a> {
    pub fn create_line_comment(content: String, span: TextSpan) -> Self {
        Token {
            kind: TokenKind::Comment(Comment {
                kind: CommentKind::Line,
                content,
            }),
            span,
        }
    }
}
