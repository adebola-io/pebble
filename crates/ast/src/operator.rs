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
}

// The operator representations. This array is sorted in descending order by the length of the operators.
pub const OPERATORS: &'static [&'static str; 34] = &[
    "...", "&&=", "||=", "*=", "/=", "+=", "-=", "==", ">=", "<=", "=>", "->", "++", "--", "..",
    "::", "||", "&&", ">>", "<<", "**", "*", "/", "+", "-", "%", "&", "|", "!", "~", ".", "?", ":",
    "=",
];

pub fn precedence_of(operator: &Operator) -> i32 {
    match operator {
        Operator::Dot => 14,
        Operator::Index => 13,
        Operator::Call => 12,
        Operator::PowerOf => 11,
        Operator::Multiply => 10,
        Operator::Remainder => 10,
        Operator::Divide => 10,
        Operator::Add => 9,
        Operator::Subtract => 9,
        Operator::Assign => 8,
        _ => todo!(),
    }
}
