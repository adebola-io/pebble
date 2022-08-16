use crate::{
    identifier::{Literal, LiteralKind},
    Comment, CommentKind, Identifier, Injunction, Keyword, Operator, Punctuation, TextSpan,
};

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind<'a> {
    Operator(Operator),
    Punctuation(Punctuation),
    Keyword(Keyword),
    Comment(Comment),
    Literal(Literal),
    Identifier(Identifier<'a>),
    Invalid { value: &'a str },
}

/// A piece of code collected when scanning the input source file.
#[derive(Debug, PartialEq, Clone)]
pub struct Token<'a> {
    pub span: TextSpan,
    pub kind: TokenKind<'a>,
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
    pub fn create_block_comment(content: String, span: TextSpan) -> Self {
        Token {
            kind: TokenKind::Comment(Comment {
                kind: CommentKind::Block,
                content,
            }),
            span,
        }
    }
    pub fn create_doc_comment(content: String, span: TextSpan) -> Self {
        Token {
            span,
            kind: TokenKind::Comment(Comment {
                kind: CommentKind::Doc,
                content,
            }),
        }
    }
    pub fn create_literal(literal_type: &str, value: String, span: TextSpan) -> Self {
        Token {
            span,
            kind: TokenKind::Literal(Literal {
                value,
                kind: match literal_type {
                    "string" => LiteralKind::StringLiteral,
                    "boolean" => LiteralKind::BooleanLiteral,
                    "number" => LiteralKind::NumericLiteral,
                    "character" => LiteralKind::CharacterLiteral,
                    _ => unreachable!(),
                },
            }),
        }
    }
    pub fn create_injunction(value: String, span: TextSpan) -> Self {
        let injunction = match value.as_str() {
            "public" => Injunction::Public,
            "function" => Injunction::Function,
            "let" => Injunction::Let,
            "tests" => Injunction::Test,
            "model" => Injunction::Model,
            "const" => Injunction::Const,
            "enum" => Injunction::Enum,
            "record" => Injunction::Record,
            "specify" => Injunction::Specify,
            "interface" => Injunction::Interface,
            "type" => Injunction::Type,
            "implement" => Injunction::Implement,
            "use" => Injunction::Use,
            "prepend" => Injunction::Prepend,
            _ => Injunction::Unknown(value),
        };
        Token {
            span,
            kind: TokenKind::Keyword(Keyword::Injunction(injunction)),
        }
    }
}
