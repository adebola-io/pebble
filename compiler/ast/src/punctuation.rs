/// Punctuation markers in Pebble.
#[derive(Debug, PartialEq, Clone)]
pub enum Punctuation {
    Bracket(BracketKind),
    Comma,
    SemiColon,
}

#[derive(Debug, PartialEq, Clone)]
pub enum BracketKind {
    LeftCurly,        // {
    RightCurly,       // }
    LeftParenthesis,  // (
    RightParenthesis, // )
    LeftSquare,       // [
    RightSquare,      // ]
}
