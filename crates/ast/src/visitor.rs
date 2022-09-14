use crate::{
    ArrayExpression, AssignmentExpression, Attribute, BinaryExpression, Block, Boolean, Break,
    CallExpression, Character, Class, ConcreteType, Continue, CrashStatement, DotExpression, Enum,
    Expression, ExpressionStatement, FnExpression, ForLoop, Function, FunctionType,
    GenericArgument, Identifier, IfStatement, Implement, IndexExpression, Interface,
    LogicalExpression, Loop, Mapping, Method, NamespaceExpression, Number, Parameter,
    PrependStatement, PrintLnStatement, Property, PublicModifier, RangeExpression, Record,
    ReturnStatement, SelfExpression, Statement, TernaryExpression, TestBlock, TextString, TryBlock,
    Type, TypeAlias, UnaryExpression, UseImport, VariableDeclaration, Variant, WhileStatement,
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
    fn for_statement(&'a self, for_loop: &ForLoop<'a>);
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
    fn enum_declaration(&'a self, enum_: &Enum<'a>);
    fn record_declaration(&'a self, record: &Record<'a>);
    fn class_declaration(&'a self, class: &Class<'a>);
    fn property(&'a self, property: &Property<'a>) -> T;
    fn method(&'a self, method: &Method<'a>) -> T;
    fn constructor(&'a self, method: &Method<'a>) -> T;
    fn attribute(&'a self, attrib: &Attribute<'a>) -> T;
    fn implement(&'a self, implement: &Implement<'a>) -> T;
    fn mapping(&'a self, map: &Mapping<'a>);
    fn variant(&'a self, variant: &Variant<'a>);
    fn parameter(&'a self, param: &Parameter<'a>) -> T;
    fn type_alias(&'a self, type_alias: &TypeAlias<'a>);
    fn type_label(&'a self, label: &Type<'a>) -> T;
    fn concrete_type(&'a self, concrete_type: &ConcreteType<'a>) -> T;
    fn functional_type(&'a self, functional_type: &FunctionType<'a>) -> T;
    fn interface(&'a self, interface: &Interface<'a>);
    fn gen_arg(&'a self, argument: &GenericArgument);
}
