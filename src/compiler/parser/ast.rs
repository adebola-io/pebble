use crate::compiler::scanner::token::Token;
type NodeRange = [usize; 4];
#[derive(Debug)]
pub struct Program {
    pub body: Block,
}

impl Program {
    pub fn new() -> Self {
        Program { body: Block::new() }
    }
}

#[derive(Debug)]
pub struct Block {
    pub range: [usize; 4],
    pub statements: Vec<Statement>,
}
impl Block {
    pub fn new() -> Self {
        Block {
            range: [0, 0, 0, 0],
            statements: vec![],
        }
    }
}
#[derive(Debug)]
pub enum Statement {
    IfStatement,
    ForStatement,
    ExpressionStatement {
        expression: Expression,
        range: NodeRange,
    },
    Declaration(Declaration),
}
#[derive(Debug)]
pub enum Declaration {
    RecordDeclaration,
    ConstantDeclaration,
    FunctionDeclaration,
    PrependDeclaration,
    UseDeclaration,
    EnumDeclaration,
    StructDeclaration,
    InterfaceDeclaration,
    TypeDeclaration,
    VariableDeclaration,
}
#[derive(Debug)]
pub enum Literal {
    Boolean {
        value: String,
        range: NodeRange,
    },
    Number {
        value: String,
        raw: String,
        range: NodeRange,
    },
    String {
        expressions: Vec<StringExpressions>,
    },
}
#[derive(Debug)]
pub enum StringExpressions {
    Sequence,
    Expression(Expression),
}
#[derive(Debug)]
pub struct Identifier {
    name: String,
    range: NodeRange,
}
#[derive(Debug)]
pub enum Expression {
    Literal(Literal),
    Identifier(Identifier),
    BinaryExpression {
        operator: String,
        left: Box<Expression>,
        right: Box<Expression>,
        range: NodeRange,
    },
    AssignmentExpression {
        operator: String,
        left: Identifier,
        right: Box<Expression>,
    },
}
