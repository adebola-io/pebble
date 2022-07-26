mod expression;
mod identifier;
mod literal;
mod program;
mod statement;

pub use expression::Expression;
pub use identifier::Identifier;
pub use literal::Literal;
pub use literal::StringExpressions;
pub use program::Block;
pub use program::Program;
pub use statement::Declaration;
pub use statement::Statement;

pub type NodeRange = [usize; 4];

pub trait Location {
    fn get_range(&self) -> NodeRange;
}
