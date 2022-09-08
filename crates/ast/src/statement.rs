use crate::{
    Block, Break, Continue, CrashStatement, Expression, ExpressionStatement, Function, IfStatement,
    LetDeclaration, Location, Loop, Module, PrependStatement, PrintLnStatement, PublicModifier,
    ReturnStatement, TestBlock, TextSpan, TryBlock, UseImport, WhileStatement,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Statement<'a> {
    IfStatement(IfStatement<'a>),
    PrintLnStatement(PrintLnStatement<'a>),
    PrependStatement(PrependStatement<'a>),
    LetDeclaration(LetDeclaration<'a>),
    Break(Break<'a>),
    Continue(Continue<'a>),
    TestBlock(TestBlock<'a>),
    LoopStmnt(Loop<'a>),
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
    Module(Module<'a>),
}

impl<'a> Statement<'a> {
    pub fn create_expr_stmnt(expression: Expression<'a>) -> Self {
        let span = expression.get_range();
        Statement::ExpressionStatement(ExpressionStatement { expression, span })
    }
}

impl<'a> Location for Statement<'a> {
    fn get_range(&self) -> TextSpan {
        match self {
            Self::IfStatement(IfStatement { span, .. })
            | Self::WhileStatement(WhileStatement { span, .. })
            | Self::LoopStmnt(Loop { span, .. })
            | Self::PrependStatement(PrependStatement { span, .. })
            | Self::PrintLnStatement(PrintLnStatement { span, .. })
            | Self::ExpressionStatement(ExpressionStatement { span, .. })
            | Self::UseImport(UseImport { span, .. })
            | Self::LetDeclaration(LetDeclaration { span, .. })
            | Self::Break(Break { span, .. })
            | Self::Continue(Continue { span, .. })
            | Self::TestBlock(TestBlock { span, .. })
            | Self::BlockStatement(Block { span, .. })
            | Self::ReturnStatement(ReturnStatement { span, .. })
            | Self::CrashStmnt(CrashStatement { span, .. })
            | Self::TryBlock(TryBlock { span, .. })
            | Self::PublicModifier(PublicModifier { span, .. })
            | Self::Function(Function { span, .. })
            | Self::Module(Module { span, .. })
            | Self::EmptyStatement(span) => *span,
        }
    }
}
