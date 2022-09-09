use crate::{
    identifier::{Literal, LiteralKind},
    BracketKind, Comment, CommentKind, Injunction, Keyword, Operator, Punctuation, TextSpan,
    TokenIdentifier,
};

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Operator(Operator),
    Punctuation(Punctuation),
    Keyword(Keyword),
    Comment(Comment),
    Literal(Literal),
    Identifier(TokenIdentifier),
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
    pub fn is_colon(&self) -> bool {
        matches!(
            self,
            Token {
                kind: TokenKind::Operator(Operator::Colon),
                ..
            }
        )
    }
    pub fn is_bracket(&self, bracket_kind: &BracketKind) -> bool {
        if let Token {
            kind: TokenKind::Punctuation(Punctuation::Bracket(b)),
            ..
        } = self
        {
            b == bracket_kind
        } else {
            false
        }
    }
    pub fn is_keyword(&self, keyword: &Keyword) -> bool {
        if let Token {
            kind: TokenKind::Keyword(keyword2),
            ..
        } = self
        {
            keyword2 == keyword
        } else {
            false
        }
    }
    pub fn is_operator(&self, op: &Operator) -> bool {
        if let Token {
            kind: TokenKind::Operator(operator),
            ..
        } = self
        {
            op == operator
        } else {
            false
        }
    }
    pub fn is_identifier(&self) -> bool {
        matches!(
            self,
            Token {
                kind: TokenKind::Identifier(_),
                ..
            }
        )
    }
    pub fn is_comma(&self) -> bool {
        matches!(
            self,
            Token {
                kind: TokenKind::Punctuation(Punctuation::Comma),
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
                "class" => Injunction::Class,
                "const" => Injunction::Const,
                "enum" => Injunction::Enum,
                "record" => Injunction::Record,
                "interface" => Injunction::Interface,
                "type" => Injunction::Type,
                "implement" => Injunction::Implement,
                "use" => Injunction::Use,
                "module" => Injunction::Module,
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
            kind: TokenKind::Identifier(TokenIdentifier { value }),
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
                "in" => Keyword::In,
                "loop" => Keyword::Loop,
                "from" => Keyword::From,
                "break" => Keyword::Break,
                "while" => Keyword::While,
                "implements" => Keyword::Implements,
                "continue" => Keyword::Continue,
                "return" => Keyword::Return,
                "crash" => Keyword::Crash,
                "try" => Keyword::Try,
                "recover" => Keyword::Recover,
                "println" => Keyword::Println,
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
                "||" => Operator::LogicalOr,
                "&&" => Operator::LogicalAnd,
                "!" => Operator::LogicalNot,
                "|" => Operator::BitwiseOr,
                "&" => Operator::BitwiseAnd,
                "~" => Operator::BitWiseNot,
                "<<" => Operator::BitwiseLeftShift,
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
