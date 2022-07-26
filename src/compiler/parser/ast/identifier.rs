use super::{Location, NodeRange};

#[derive(Debug, PartialEq)]
pub struct Identifier {
    name: String,
    range: NodeRange,
}

impl Location for Identifier {
    fn get_range(&self) -> NodeRange {
        self.range
    }
}
