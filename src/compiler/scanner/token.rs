#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub enum StringInnerToken {
    StringSequence { value: String, loc: [usize; 4] },
    StringExpression { tokens: Vec<Token>, loc: [usize; 4] },
}
#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub struct Comment {
    /// The kind of comment tokenized, either block, line or doc.
    pub kind: CommentKind,
    pub value: String,
    pub loc: [usize; 4],
}
#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    /// Token representing the start of the file.
    SOF,
    /// A token that begins with an @ symbol.
    Injunction {
        value: String,
        loc: [usize; 4],
    },
    Number {
        /// The kind of number tokenized, whether a decimal, hexadecimal, octal, exponent, etc.
        kind: NumericKind,
        raw: String,
        loc: [usize; 4],
    },
    Character {
        value: String,
        loc: [usize; 4],
    },
    StringToken {
        inner: Vec<StringInnerToken>,
        loc: [usize; 4],
    },
    Identifier {
        value: String,
        loc: [usize; 4],
    },
    Bracket {
        kind: BracketKind,
        loc: [usize; 4],
    },
    Keyword {
        value: String,
        loc: [usize; 4],
    },
    Comma {
        loc: [usize; 4],
    },
    SemiColon {
        loc: [usize; 4],
    },
    Colon {
        loc: [usize; 4],
    },
    Operator {
        value: String,
        loc: [usize; 4],
    },
    /// Token representing the end of the file.
    EOF,
}
impl Token {
    pub fn get_location(&self) -> [usize; 4] {
        match self {
            Self::Bracket { loc, .. }
            | Self::Colon { loc }
            | Self::Comma { loc, .. }
            | Self::Number { loc, .. }
            | Self::Character { loc, .. }
            | Self::Keyword { loc, .. }
            | Self::Identifier { loc, .. }
            | Self::StringToken { loc, .. }
            | Self::Injunction { loc, .. }
            | Self::SemiColon { loc, .. }
            | Self::Operator { loc, .. } => *loc,
            _ => [0, 0, 0, 0],
        }
    }
    pub fn is_comma(&self) -> bool {
        match self {
            Self::Comma { .. } => true,
            _ => false,
        }
    }
    pub fn is_semi_colon(&self) -> bool {
        match self {
            Self::SemiColon { .. } => true,
            _ => false,
        }
    }
    pub fn is_number(&self) -> bool {
        match self {
            Self::Number { .. } => true,
            _ => false,
        }
    }
    pub fn get_end_line(&self) -> usize {
        self.get_location()[2]
    }
    pub fn get_end_column(&self) -> usize {
        self.get_location()[3]
    }
    pub fn is_bracket(&self, _bracketkind: BracketKind) -> bool {
        match self {
            Self::Bracket {
                kind: _bracketkind, ..
            } => true,
            _ => false,
        }
    }
    pub fn is_identifier(&self) -> bool {
        match self {
            Self::Identifier { .. } => true,
            _ => false,
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub enum CommentKind {
    Block,
    Line,
    Documentation,
}
#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub enum NumericKind {
    Hexadecimal,
    Decimal,
    Octal,
    Binary,
}
#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub enum BracketKind {
    LCurly,
    RCurly,
    LSquare,
    RSquare,
    RParen,
    LParen,
}
