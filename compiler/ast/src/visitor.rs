use crate::{
    ArrayExpression, AssignmentExpression, Attribute, BinaryExpression, Block, Boolean, Break,
    CallExpression, Character, Class, ConcreteType, Continue, CrashStatement, DotExpression, Enum,
    Expression, ExpressionStatement, FnExpression, ForLoop, Function, FunctionType,
    GenericArgument, Identifier, IfStatement, Implement, IndexExpression, Interface,
    LogicalExpression, Loop, Mapping, Method, Module, NamespaceExpression, Number, Parameter,
    PrependStatement, PrintLnStatement, Property, PublicModifier, RangeExpression, Record,
    ReturnStatement, SelfExpression, Statement, TernaryExpression, TestBlock, TextString, TryBlock,
    Type, TypeAlias, UnaryExpression, UseImport, VariableDeclaration, Variant, WhileStatement,
};

/// Recursively visits every node in the Abstract syntax tree using the visitor pattern.
pub trait ASTVisitor<'a> {
    type Output: Default;

    fn visit_expression(&'a self, exp: &Expression<'a>) -> Self::Output {
        match exp {
            Expression::IdentifierExpression(identifier) => self.visit_identifier(identifier),
            Expression::StringExpression(string) => self.visit_string(string),
            Expression::NumericExpression(number) => self.visit_number(number),
            Expression::BooleanExpression(boolean) => self.visit_boolean(boolean),
            Expression::CharacterExpression(character) => self.visit_character(character),
            Expression::SelfExpression(self_expression) => {
                self.visit_self_expression(self_expression)
            }
            Expression::BinaryExpression(binary_expression) => {
                self.visit_binary_expression(binary_expression)
            }
            Expression::LogicalExpression(logical_expression) => {
                self.visit_logical_expression(logical_expression)
            }
            Expression::UnaryExpression(unary_expression) => {
                self.visit_unary_expression(unary_expression)
            }
            Expression::CallExpression(call_expression) => {
                self.visit_call_expression(call_expression)
            }
            Expression::ArrayExpression(array_expression) => {
                self.visit_array_expression(array_expression)
            }
            Expression::IndexExpression(index_expression) => {
                self.visit_index_expression(index_expression)
            }
            Expression::DotExpression(dot_expression) => self.visit_dot_expression(dot_expression),
            Expression::NamespaceExpression(namespace_expression) => {
                self.visit_namespace_expression(namespace_expression)
            }
            Expression::RangeExpression(range_expression) => {
                self.visit_range_expression(range_expression)
            }
            Expression::TernaryExpression(ternary_expression) => {
                self.visit_ternary_expression(ternary_expression)
            }
            Expression::AssignmentExpression(assignment_expression) => {
                self.visit_assignment_expression(assignment_expression)
            }
            Expression::FnExpression(fn_expression) => self.visit_fn_expression(fn_expression),
        }
    }
    fn visit_identifier(&'a self, ident: &Identifier<'a>) -> Self::Output;
    fn visit_string(&'a self, string: &TextString<'a>) -> Self::Output;
    fn visit_number(&'a self, number: &Number<'a>) -> Self::Output;
    fn visit_character(&'a self, charac: &Character<'a>) -> Self::Output;
    fn visit_boolean(&'a self, boolean: &Boolean<'a>) -> Self::Output;
    fn visit_self_expression(&'a self, self_: &SelfExpression) -> Self::Output;
    fn visit_binary_expression(&'a self, bin_exp: &BinaryExpression<'a>) -> Self::Output {
        self.visit_expression(&bin_exp.operands[0]);
        self.visit_expression(&bin_exp.operands[1]);
        Default::default()
    }
    fn visit_logical_expression(&'a self, log_exp: &LogicalExpression<'a>) -> Self::Output {
        self.visit_expression(&log_exp.operands[0]);
        self.visit_expression(&log_exp.operands[1]);
        Default::default()
    }
    fn visit_dot_expression(&'a self, dot_exp: &DotExpression<'a>) -> Self::Output;
    fn visit_unary_expression(&'a self, unary_exp: &UnaryExpression<'a>) -> Self::Output;
    fn visit_namespace_expression(
        &'a self,
        namespace_exp: &NamespaceExpression<'a>,
    ) -> Self::Output;
    fn visit_assignment_expression(&'a self, assign_exp: &AssignmentExpression<'a>)
        -> Self::Output;
    fn visit_index_expression(&'a self, index_exp: &IndexExpression<'a>) -> Self::Output;
    fn visit_call_expression(&'a self, call_exp: &CallExpression<'a>) -> Self::Output;
    fn visit_array_expression(&'a self, array_exp: &ArrayExpression<'a>) -> Self::Output;
    fn visit_ternary_expression(&'a self, tern_exp: &TernaryExpression<'a>) -> Self::Output;
    fn visit_range_expression(&'a self, rang_exp: &RangeExpression<'a>) -> Self::Output;
    fn visit_fn_expression(&'a self, fn_exp: &FnExpression<'a>) -> Self::Output;
    fn visit_statement(&'a self, statement: &Statement<'a>) {
        match statement {
            Statement::IfStatement(if_stmnt) => self.visit_if_statement(if_stmnt),
            Statement::PrintLnStatement(println_stmnt) => {
                self.visit_println_statement(println_stmnt)
            }
            Statement::PrependStatement(prepend_stmnt) => {
                self.visit_prepend_statement(prepend_stmnt)
            }
            Statement::VariableDeclaration(var_decl) => self.visit_variable_declaration(var_decl),
            Statement::Break(break_) => self.visit_break(break_),
            Statement::Continue(continue_) => self.visit_continue(continue_),
            Statement::TestBlock(test_block) => self.visit_test_block(test_block),
            Statement::LoopStmnt(loop_statement) => self.visit_loop_statement(loop_statement),
            Statement::ForLoop(for_loop) => self.visit_for_loop(for_loop),
            Statement::WhileLoop(while_loop) => self.visit_while_loop(while_loop),
            Statement::PublicModifier(public_mod) => self.visit_public_statement(public_mod),
            Statement::ExpressionStatement(exp_stmnt) => self.visit_expression_statement(exp_stmnt),
            Statement::BlockStatement(block) => self.visit_block(block),
            Statement::UseImport(use_import) => self.visit_use_import(use_import),
            Statement::ReturnStatement(return_stmnt) => self.visit_return_statement(return_stmnt),
            Statement::CrashStmnt(crash) => self.visit_crash(crash),
            Statement::EmptyStatement(_) => self.visit_empty_statement(statement),
            Statement::TryBlock(try_block) => self.visit_try_block(try_block),
            Statement::Function(function) => self.visit_function(function),
            Statement::TypeAlias(type_alias) => self.visit_type_alias(type_alias),
            Statement::Interface(interface) => self.visit_interface(interface),
            Statement::Enum(enum_) => self.visit_enum_declaration(enum_),
            Statement::Class(class) => self.visit_class_declaration(class),
            Statement::Module(module) => self.visit_module(module),
            Statement::Record(record) => self.visit_record_declaration(record),
        }
    }
    fn visit_if_statement(&'a self, if_stmnt: &IfStatement<'a>);
    fn visit_println_statement(&'a self, println_stmnt: &PrintLnStatement<'a>) {
        self.visit_expression(&println_stmnt.argument);
    }
    fn visit_prepend_statement(&'a self, prepend_stmnt: &PrependStatement<'a>);
    fn visit_variable_declaration(&'a self, var_decl: &VariableDeclaration<'a>);
    fn visit_break(&'a self, break_: &Break);
    fn visit_continue(&'a self, continue_: &Continue<'a>);
    fn visit_test_block(&'a self, test_block: &TestBlock<'a>);
    fn visit_loop_statement(&'a self, loop_stmnt: &Loop<'a>);
    fn visit_for_loop(&'a self, for_loop: &ForLoop<'a>);
    fn visit_while_loop(&'a self, while_stmnt: &WhileStatement<'a>);
    fn visit_public_statement(&'a self, public_mod: &PublicModifier<'a>);
    fn visit_expression_statement(&'a self, exp_stmnt: &ExpressionStatement<'a>) {
        self.visit_expression(&exp_stmnt.expression);
    }
    fn visit_block(&'a self, block: &Block<'a>);
    fn visit_use_import(&'a self, use_stmnt: &UseImport<'a>);
    fn visit_return_statement(&'a self, return_stmnt: &ReturnStatement<'a>);
    fn visit_crash(&'a self, crash: &CrashStatement<'a>);
    fn visit_empty_statement(&'a self, _empty: &Statement<'a>) {}
    fn visit_try_block(&'a self, try_block: &TryBlock<'a>);
    fn visit_function(&'a self, function: &Function<'a>);
    fn visit_module(&'a self, module: &Module<'a>);
    fn visit_enum_declaration(&'a self, enum_: &Enum<'a>);
    fn visit_record_declaration(&'a self, record: &Record<'a>);
    fn visit_class_declaration(&'a self, class: &Class<'a>);
    fn visit_property(&'a self, property: &Property<'a>) -> Self::Output;
    fn visit_method(&'a self, method: &Method<'a>) -> Self::Output;
    fn visit_constructor(&'a self, method: &Method<'a>) -> Self::Output;
    fn visit_attribute(&'a self, attrib: &Attribute<'a>) -> Self::Output;
    fn visit_implement(&'a self, implement: &Implement<'a>) -> Self::Output;
    fn visit_record_mapping(&'a self, map: &Mapping<'a>);
    fn visit_variant(&'a self, variant: &Variant<'a>);
    fn visit_parameter(&'a self, param: &Parameter<'a>) -> Self::Output;
    fn visit_type_alias(&'a self, type_alias: &TypeAlias<'a>);
    fn visit_type_label(&'a self, label: &Type<'a>) -> Self::Output;
    fn visit_concrete_type(&'a self, concrete_type: &ConcreteType<'a>) -> Self::Output;
    fn visit_functional_type(&'a self, functional_type: &FunctionType<'a>) -> Self::Output;
    fn visit_interface(&'a self, interface: &Interface<'a>);
    fn visit_generic_argument(&'a self, argument: &GenericArgument);
}
