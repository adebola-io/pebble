use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Character(char),
    Boolean(bool),
}
pub type ValueOrError = Result<Value, String>;

impl Value {
    pub fn get_type(&self) -> &str {
        match self {
            Value::Number(_) => "Number",
            Value::String(_) => "String",
            Value::Character(_) => "Character",
            Value::Boolean(_) => "Boolean",
        }
    }
    pub fn add(self, rhs: Self) -> ValueOrError {
        match (&self, &rhs) {
            (Value::Number(n1), Value::Number(n2)) => Ok(Value::Number(n1 + n2)),
            (Value::String(s1), Value::String(s2)) => Ok(Value::String(s1.clone() + &s2)),
            _ => Err(format!(
                "Cannot add {} to {}.",
                self.get_type(),
                rhs.get_type()
            )),
        }
    }
    pub fn sub(self, rhs: Self) -> ValueOrError {
        match (&self, &rhs) {
            (Value::Number(n1), Value::Number(n2)) => Ok(Value::Number(n1 - n2)),
            _ => Err(format!(
                "Cannot subtract {} from {}.",
                rhs.get_type(),
                self.get_type()
            )),
        }
    }
    pub fn mul(self, rhs: Self) -> ValueOrError {
        match (&self, &rhs) {
            (Value::Number(n1), Value::Number(n2)) => Ok(Value::Number(n1 * n2)),
            _ => Err(format!(
                "Cannot multiply {} and {}.",
                rhs.get_type(),
                self.get_type()
            )),
        }
    }
    pub fn div(self, rhs: Self) -> ValueOrError {
        match (&self, &rhs) {
            (Value::Number(n1), Value::Number(n2)) => Ok(Value::Number(n1 / n2)),
            _ => Err(format!(
                "Cannot divide {} by {}.",
                self.get_type(),
                rhs.get_type()
            )),
        }
    }
    pub fn inverse(self) -> ValueOrError {
        match self {
            Value::Boolean(b) => Ok(Value::Boolean(!b)),
            _ => Err(format!(
                "The ! operator cannot be applied to a {}.",
                self.get_type()
            )),
        }
    }
    pub fn negate(self) -> ValueOrError {
        match &self {
            Value::Number(n) => Ok(Value::Number(-n)),
            _ => Err(format!(
                "The - operator cannot be applied to a {}.",
                self.get_type(),
            )),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "\u{001b}[33m{}\u{001b}[0m", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Character(c) => write!(f, "{}", c),
            Value::Boolean(b) => write!(f, "\u{001b}[33m{}\u{001b}[0m", b),
        }
    }
}
