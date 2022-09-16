use std::fmt::Display;

/// All valid Operators in Pebble.
#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Add,                 // a + b
    Multiply,            // a * b
    Subtract,            // a - b
    Divide,              // a / b
    Remainder,           // a % b
    PowerOf,             // a ** b
    LogicalAnd,          // a && b
    LogicalOr,           // a || b
    LogicalNot,          // !a
    BitwiseOr,           // a | b
    BitwiseAnd,          // a & b
    BitWiseNot,          // ~a
    BitwiseLeftShift,    // a << b
    BitwiseRightShift,   // a >> b
    RangeBetween,        // a..b
    Assign,              // a = b
    AddAssign,           // a += b
    SubtractAssign,      // a -= b
    DivideAssign,        // a /= b
    MultiplyAssign,      // a *= b
    LogicalAndAssign,    // a &&= b
    LogicalOrAssign,     // a ||= b
    Equals,              // a == b
    NotEquals,           // a != b
    GreaterThan,         // a > b
    LessThan,            // a < b
    GreaterThanOrEquals, // a >= b
    LessThanOrEquals,    // a <= b
    Confirm,             // a ? b
    Colon,               // a:b
    RestOf,              // ...a
    Namespace,           // a::b
    Dot,                 // a.b
    Arrow,               // a => b
    Returns,             // a -> b
    Increment,           // a++
    Decrement,           // a--
    Call,                // a(b)
    Index,               // a[b]
    Temp,                // A pseudo operator.
}

// The operator representations. This array is sorted in descending order by the length of the operators.
pub const OPERATORS: &'static [&'static str; 36] = &[
    "...", "&&=", "||=", "*=", "/=", "+=", "-=", "==", "!=", ">=", "<=", "=>", "->", "++", "--",
    "..", "::", "||", "&&", "<<", "^", "*", "/", "+", "-", "%", "&", "|", "!", "~", ".", "?", ":",
    ">", "<", "=",
];

pub fn precedence_of(operator: &Operator) -> i32 {
    match operator {
        Operator::Dot => 20,
        Operator::Namespace => 19,
        Operator::Index => 18,
        Operator::Call => 17,
        Operator::RangeBetween => 16,
        Operator::LogicalNot | Operator::BitWiseNot => 15,
        Operator::PowerOf => 14,
        Operator::Multiply | Operator::Remainder | Operator::Divide => 13,
        Operator::Add | Operator::Subtract => 12,
        Operator::BitwiseLeftShift | Operator::BitwiseRightShift => 11,
        Operator::LessThan
        | Operator::GreaterThan
        | Operator::LessThanOrEquals
        | Operator::GreaterThanOrEquals => 10,
        Operator::Equals | Operator::NotEquals => 11,
        Operator::BitwiseAnd => 10,
        Operator::BitwiseOr => 9,
        Operator::LogicalAnd => 8,
        Operator::LogicalOr => 7,
        Operator::Confirm => 6,
        Operator::Colon => 5,
        Operator::Assign
        | Operator::AddAssign
        | Operator::DivideAssign
        | Operator::MultiplyAssign
        | Operator::SubtractAssign
        | Operator::LogicalOrAssign
        | Operator::LogicalAndAssign => 4,
        Operator::Temp => 0,
        _ => todo!(),
    }
}

impl Operator {
    pub fn from(op: &str) -> Self {
        match op {
            "+" => Operator::Add,
            "*" => Operator::Multiply,
            "-" => Operator::Subtract,
            "/" => Operator::Divide,
            "%" => Operator::Remainder,
            "^" => Operator::PowerOf,
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
        }
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Operator::Add => "+",
                Operator::Multiply => "*",
                Operator::Subtract => "-",
                Operator::Divide => "/",
                Operator::Remainder => "*",
                Operator::PowerOf => "^",
                Operator::LogicalAnd => "&&",
                Operator::LogicalOr => "||",
                Operator::LogicalNot => "!",
                Operator::BitwiseOr => "|",
                Operator::BitwiseAnd => "&",
                Operator::BitWiseNot => "~",
                Operator::BitwiseLeftShift => "<<",
                Operator::BitwiseRightShift => ">>",
                Operator::RangeBetween => "..",
                Operator::Assign => "=",
                Operator::AddAssign => "+=",
                Operator::SubtractAssign => "-=",
                Operator::DivideAssign => "/=",
                Operator::MultiplyAssign => "*=",
                Operator::LogicalAndAssign => "&&=",
                Operator::LogicalOrAssign => "||=",
                Operator::Equals => "==",
                Operator::NotEquals => "!=",
                Operator::GreaterThan => ">",
                Operator::LessThan => "<",
                Operator::GreaterThanOrEquals => ">=",
                Operator::LessThanOrEquals => "<=",
                Operator::Confirm => "?",
                Operator::Colon => ":",
                Operator::RestOf => "...",
                Operator::Namespace => "::",
                Operator::Dot => ".",
                Operator::Arrow => "=>",
                Operator::Returns => "->",
                Operator::Increment => "++",
                Operator::Decrement => "--",
                Operator::Call => "()",
                Operator::Index => "[]",
                Operator::Temp => "0",
            }
        )
    }
}
