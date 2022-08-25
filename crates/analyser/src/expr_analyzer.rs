use ast::Expression;

pub trait Analyzer {}

impl<'a> Analyzer for Expression<'a> {}
