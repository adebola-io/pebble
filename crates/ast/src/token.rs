use crate::{
    identifier::{Literal, LiteralKind},
    BracketKind, Comment, CommentKind, Identifier, Injunction, Keyword, Operator, Punctuation,
    TextSpan,
};

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Operator(Operator),
    Punctuation(Punctuation),
    Keyword(Keyword),
    Comment(Comment),
    Literal(Literal),
    Identifier(Identifier),
    Invalid(String),
    EOF,
}

/// A piece of code collected when scanning the input source file.
#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub span: TextSpan,
    pub kind: TokenKind,
}

impl Token {
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
    pub fn is_comment(&self) -> bool {
        matches!(
            self,
            Token {
                kind: TokenKind::Comment(_),
                ..
            }
        )
    }
    pub fn is_eof(&self) -> bool {
        matches!(
            self,
            Token {
                kind: TokenKind::EOF,
                ..
            }
        )
    }
    pub fn is_semi_colon(&self) -> bool {
        matches!(
            self,
            Token {
                kind: TokenKind::Punctuation(Punctuation::SemiColon),
                ..
            }
        )
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
    pub fn create_injunction(value: &str, span: TextSpan) -> Self {
        Token {
            span,
            kind: TokenKind::Keyword(Keyword::Injunction(match value {
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
                _ => Injunction::Unknown(value.to_string()),
            })),
        }
    }
    pub fn create_bracket(value: &char, span: TextSpan) -> Self {
        Token {
            span,
            kind: TokenKind::Punctuation(Punctuation::Bracket(match value {
                '[' => BracketKind::LeftSquare,
                ']' => BracketKind::RightSquare,
                '{' => BracketKind::LeftCurly,
                '}' => BracketKind::RightCurly,
                '(' => BracketKind::LeftParenthesis,
                ')' => BracketKind::RightParenthesis,
                _ => unreachable!(),
            })),
        }
    }
    pub fn create_semi_colon(span: TextSpan) -> Self {
        Token {
            span,
            kind: TokenKind::Punctuation(Punctuation::SemiColon),
        }
    }
    pub fn create_comma(span: TextSpan) -> Self {
        Token {
            span,
            kind: TokenKind::Punctuation(Punctuation::Comma),
        }
    }
    pub fn create_identifier(value: String, span: TextSpan) -> Self {
        Token {
            span,
            kind: TokenKind::Identifier(Identifier { value }),
        }
    }
    pub fn create_keyword(value: String, span: TextSpan) -> Self {
        Token {
            span,
            kind: TokenKind::Keyword(match value.as_str() {
                "as" => Keyword::As,
                "for" => Keyword::For,
                "if" => Keyword::If,
                "else" => Keyword::Else,
                "match" => Keyword::Match,
                "in" => Keyword::In,
                "loop" => Keyword::Loop,
                "case" => Keyword::Case,
                "break" => Keyword::Break,
                "do" => Keyword::Do,
                "while" => Keyword::While,
                "continue" => Keyword::Continue,
                "return" => Keyword::Return,
                "crash" => Keyword::Crash,
                "try" => Keyword::Try,
                "recover" => Keyword::Recover,
                "println" => Keyword::Println,
                "sleep" => Keyword::Sleep,
                "static" => Keyword::Static,
                _ => unreachable!(),
            }),
        }
    }
    pub fn create_operator(value: &str, span: TextSpan) -> Self {
        Token {
            span,
            kind: TokenKind::Operator(match value {
                "+" => Operator::Add,
                "*" => Operator::Multiply,
                "-" => Operator::Subtract,
                "/" => Operator::Divide,
                "%" => Operator::Remainder,
                "**" => Operator::PowerOf,
                "new" => Operator::New,
                "||" => Operator::LogicalOr,
                "&&" => Operator::LogicalAnd,
                "!" => Operator::LogicalNot,
                "|" => Operator::BitwiseOr,
                "&" => Operator::BiwiseAnd,
                "~" => Operator::BitWiseNot,
                "<<" => Operator::BitwiseLeftShift,
                ">>" => Operator::BitwiseRightShift,
                ".." => Operator::RangeBetween,
                "=" => Operator::Assign,
                "+=" => Operator::AddAssign,
                "-=" => Operator::SubtractAssign,
                "/=" => Operator::DivideAssign,
                "*=" => Operator::MultiplyAssign,
                "||=" => Operator::LogicalAndAssign,
                "==" => Operator::Equals,
                "!=" => Operator::NotEquals,
                ">" => Operator::GreaterThan,
                "<" => Operator::LessThan,
                ">=" => Operator::GreaterThanOrEquals,
                "<=" => Operator::LessThanOrEquals,
                "?" => Operator::Confirm,
                ":" => Operator::Colon,
                "..." => Operator::RestOf,
                "::" => Operator::Namespace,
                "." => Operator::Dot,
                "=>" => Operator::Arrow,
                "->" => Operator::Returns,
                "++" => Operator::Increment,
                "--" => Operator::Decrement,
                _ => unreachable!(),
            }),
        }
    }
    pub fn create_unknown(value: String, span: TextSpan) -> Self {
        Token {
            span,
            kind: TokenKind::Invalid(value),
        }
    }
    pub fn eof(span: TextSpan) -> Self {
        Token {
            span,
            kind: TokenKind::EOF,
        }
    }
    pub fn value_of(&self) -> &str {
        match self.kind {
            TokenKind::Operator(_) => todo!(),
            TokenKind::Punctuation(_) => todo!(),
            TokenKind::Keyword(_) => todo!(),
            TokenKind::Comment(_) => todo!(),
            TokenKind::Literal(_) => todo!(),
            TokenKind::Identifier(_) => todo!(),
            TokenKind::Invalid(_) => todo!(),
            TokenKind::EOF => todo!(),
        }
    }
}
