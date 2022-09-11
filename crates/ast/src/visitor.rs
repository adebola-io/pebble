use crate::{
    ArrayExpression, AssignmentExpression, BinaryExpression, Block, Boolean, Break, CallExpression,
    Character, Continue, CrashStatement, DotExpression, Expression, ExpressionStatement,
    FnExpression, Function, Identifier, IfStatement, IndexExpression, Interface, LogicalExpression,
    Loop, NamespaceExpression, Number, PrependStatement, PrintLnStatement, PublicModifier,
    RangeExpression, ReturnStatement, SelfExpression, Statement, TernaryExpression, TestBlock,
    TextString, TryBlock, TypeAlias, UnaryExpression, UseImport, VariableDeclaration,
    WhileStatement,
};

pub trait Visitor<'a, T = ()> {
    fn expression(&'a self, exp: &Expression<'a>) -> T;
    fn ident(&'a self, ident: &Identifier<'a>) -> T;
    fn string(&'a self, string: &TextString<'a>) -> T;
    fn number(&'a self, number: &Number<'a>) -> T;
    fn character(&'a self, charac: &Character<'a>) -> T;
    fn boolean(&'a self, boolean: &Boolean<'a>) -> T;
    fn self_exp(&'a self, self_: &SelfExpression) -> T;
    fn binary_exp(&'a self, bin_exp: &BinaryExpression<'a>) -> T;
    fn logical_exp(&'a self, log_exp: &LogicalExpression<'a>) -> T;
    fn dot_exp(&'a self, dot_exp: &DotExpression<'a>) -> T;
    fn unary_exp(&'a self, unary_exp: &UnaryExpression<'a>) -> T;
    fn namespace_exp(&'a self, namespace_exp: &NamespaceExpression<'a>) -> T;
    fn assign_exp(&'a self, assign_exp: &AssignmentExpression<'a>) -> T;
    fn index_exp(&'a self, index_exp: &IndexExpression<'a>) -> T;
    fn call_exp(&'a self, call_exp: &CallExpression<'a>) -> T;
    fn array_exp(&'a self, array_exp: &ArrayExpression<'a>) -> T;
    fn tern_exp(&'a self, tern_exp: &TernaryExpression<'a>) -> T;
    fn range_exp(&'a self, rang_exp: &RangeExpression<'a>) -> T;
    fn fn_exp(&'a self, fn_exp: &FnExpression<'a>) -> T;
    fn statement(&'a self, statement: &Statement<'a>);
    fn if_statement(&'a self, if_stmnt: &IfStatement<'a>);
    fn println_statement(&'a self, println_stmnt: &PrintLnStatement<'a>);
    fn prepend_statement(&'a self, prepend_stmnt: &PrependStatement<'a>);
    fn var_decl(&'a self, var_decl: &VariableDeclaration<'a>);
    fn breack(&'a self, brk: &Break<'a>);
    fn kontinue(&'a self, cont: &Continue<'a>);
    fn test_block(&'a self, test_block: &TestBlock<'a>);
    fn loop_statement(&'a self, loop_stmnt: &Loop<'a>);
    fn while_statement(&'a self, while_stmnt: &WhileStatement<'a>);
    fn public_mod(&'a self, public_mod: &PublicModifier<'a>);
    fn exp_statement(&'a self, exp_stmnt: &ExpressionStatement<'a>);
    fn block(&'a self, block: &Block<'a>);
    fn use_import(&'a self, use_stmnt: &UseImport<'a>);
    fn return_statement(&'a self, return_stmnt: &ReturnStatement<'a>);
    fn crash(&'a self, crash: &CrashStatement<'a>);
    fn empty_statement(&'a self, empty: &ExpressionStatement<'a>);
    fn try_block(&'a self, try_block: &TryBlock<'a>);
    fn function(&'a self, function: &Function<'a>);
    fn type_alias(&'a self, type_alias: &TypeAlias<'a>);
    fn interface(&'a self, interface: &Interface<'a>);
}
