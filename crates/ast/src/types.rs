use crate::{Location, TextSpan};

#[derive(Clone, Debug, PartialEq)]
pub struct Type {
    name: String,
    arguments: Vec<Self>,
    span: TextSpan,
}

impl Location for Type {
    fn get_range(&self) -> TextSpan {
        self.span
    }
}
