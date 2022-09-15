use crate::{
    Block, Break, Class, Continue, CrashStatement, Enum, Expression, ExpressionStatement, ForLoop,
    Function, IfStatement, Interface, Location, Loop, Module, PrependStatement, PrintLnStatement,
    PublicModifier, Record, ReturnStatement, TestBlock, TextSpan, TryBlock, TypeAlias, UseImport,
    VariableDeclaration, WhileStatement,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Statement<'a> {
    IfStatement(IfStatement<'a>),
    PrintLnStatement(PrintLnStatement<'a>),
    PrependStatement(PrependStatement<'a>),
    VariableDeclaration(VariableDeclaration<'a>),
    Break(Break),
    Continue(Continue<'a>),
    TestBlock(TestBlock<'a>),
    LoopStmnt(Loop<'a>),
    ForStatement(ForLoop<'a>),
    WhileStatement(WhileStatement<'a>),
    PublicModifier(PublicModifier<'a>),
    ExpressionStatement(ExpressionStatement<'a>),
    BlockStatement(Block<'a>),
    UseImport(UseImport<'a>),
    ReturnStatement(ReturnStatement<'a>),
    CrashStmnt(CrashStatement<'a>),
    EmptyStatement(TextSpan),
    TryBlock(TryBlock<'a>),
    Function(Function<'a>),
    TypeAlias(TypeAlias<'a>),
    Interface(Interface<'a>),
    Enum(Enum<'a>),
    Class(Class<'a>),
    Module(Module<'a>),
    Record(Record<'a>),
}

impl<'a> Statement<'a> {
    pub fn create_expr_stmnt(expression: Expression<'a>) -> Self {
        let span = expression.get_range();
        Statement::ExpressionStatement(ExpressionStatement { expression, span })
    }
    pub fn is_declaration(&self) -> bool {
        match self {
            Statement::PrependStatement(_)
            | Statement::VariableDeclaration(_)
            | Statement::TestBlock(_)
            | Statement::PublicModifier(_)
            | Statement::UseImport(_)
            | Statement::Function(_)
            | Statement::TypeAlias(_)
            | Statement::Interface(_)
            | Statement::Class(_)
            | Statement::Module(_) => true,
            _ => false,
        }
    }
}

impl<'a> Location for Statement<'a> {
    fn get_range(&self) -> TextSpan {
        match self {
            Self::IfStatement(IfStatement { span, .. })
            | Self::WhileStatement(WhileStatement { span, .. })
            | Self::LoopStmnt(Loop { span, .. })
            | Self::ForStatement(ForLoop { span, .. })
            | Self::PrependStatement(PrependStatement { span, .. })
            | Self::PrintLnStatement(PrintLnStatement { span, .. })
            | Self::ExpressionStatement(ExpressionStatement { span, .. })
            | Self::UseImport(UseImport { span, .. })
            | Self::VariableDeclaration(VariableDeclaration { span, .. })
            | Self::Break(Break { span, .. })
            | Self::Continue(Continue { span, .. })
            | Self::TestBlock(TestBlock { span, .. })
            | Self::BlockStatement(Block { span, .. })
            | Self::ReturnStatement(ReturnStatement { span, .. })
            | Self::CrashStmnt(CrashStatement { span, .. })
            | Self::TryBlock(TryBlock { span, .. })
            | Self::PublicModifier(PublicModifier { span, .. })
            | Self::Function(Function { span, .. })
            | Self::TypeAlias(TypeAlias { span, .. })
            | Self::Module(Module { span, .. })
            | Self::EmptyStatement(span)
            | Self::Interface(Interface { span, .. })
            | Self::Class(Class { span, .. })
            | Self::Record(Record { span, .. })
            | Self::Enum(Enum { span, .. }) => *span,
        }
    }
}
