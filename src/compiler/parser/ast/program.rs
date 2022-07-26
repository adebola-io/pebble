use super::{statement, Location, NodeRange, Statement};

#[derive(Debug, PartialEq)]
pub struct Program {
    pub range: NodeRange,
    pub body: Block,
}

impl Program {
    pub fn new() -> Self {
        Program {
            body: Block::new(),
            range: [1, 1, 0, 0],
        }
    }
    pub fn append(&mut self, statement: Statement) {
        let statement_range = statement.get_range();
        self.range[2] = statement_range[2];
        self.range[3] = statement_range[3];
        self.body.push(statement);
    }
}

#[derive(Debug, PartialEq)]
pub struct Block {
    pub range: [usize; 4],
    pub statements: Vec<Statement>,
}

impl Block {
    pub fn new() -> Self {
        Block {
            range: [1, 1, 0, 0],
            statements: vec![],
        }
    }
    pub fn push(&mut self, statement: Statement) {
        let statement_range = statement.get_range();
        self.range[2] = statement_range[2];
        self.range[3] = statement_range[3];
        self.statements.push(statement)
    }
}
