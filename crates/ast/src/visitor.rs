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
    fn visit_expression(&'a self, exp: &Expression<'a>) -> T {
        match exp {
            Expression::IdentifierExpression(ident) => self.visit_ident(ident),
            Expression::StringExpression(string) => self.visit_string(string),
            Expression::NumericExpression(number) => self.visit_number(number),
            Expression::BooleanExpression(boolean) => self.visit_boolean(boolean),
            Expression::CharacterExpression(charac) => self.visit_character(charac),
            Expression::SelfExpression(self_) => self.visit_self_expression(self_),
            Expression::BinaryExpression(bin_exp) => self.visit_binary_expression(bin_exp),
            Expression::LogicalExpression(log_exp) => self.visit_logical_expression(log_exp),
            Expression::UnaryExpression(unary_exp) => self.visit_unary_expression(unary_exp),
            Expression::CallExpression(call_exp) => self.visit_call_expression(call_exp),
            Expression::ArrayExpression(array_exp) => self.visit_array_expression(array_exp),
            Expression::IndexExpression(index_exp) => self.visit_index_expression(index_exp),
            Expression::DotExpression(dot_exp) => self.visit_dot_expression(dot_exp),
            Expression::NamespaceExpression(namespace_exp) => {
                self.visit_namespace_exp(namespace_exp)
            }
            Expression::RangeExpression(rang_exp) => self.visit_range_expression(rang_exp),
            Expression::TernaryExpression(tern_exp) => self.visit_ternary_expression(tern_exp),
            Expression::AssignmentExpression(assign_exp) => {
                self.visit_assign_expression(assign_exp)
            }
            Expression::FnExpression(fn_exp) => self.visit_function_expression(fn_exp),
        }
    }
    fn visit_ident(&'a self, ident: &Identifier<'a>) -> T;
    fn visit_string(&'a self, string: &TextString<'a>) -> T;
    fn visit_number(&'a self, number: &Number<'a>) -> T;
    fn visit_character(&'a self, charac: &Character<'a>) -> T;
    fn visit_boolean(&'a self, boolean: &Boolean<'a>) -> T;
    fn visit_self_expression(&'a self, self_: &SelfExpression) -> T;
    fn visit_binary_expression(&'a self, bin_exp: &BinaryExpression<'a>) -> T;
    fn visit_logical_expression(&'a self, log_exp: &LogicalExpression<'a>) -> T;
    fn visit_dot_expression(&'a self, dot_exp: &DotExpression<'a>) -> T;
    fn visit_unary_expression(&'a self, unary_exp: &UnaryExpression<'a>) -> T;
    fn visit_namespace_exp(&'a self, namespace_exp: &NamespaceExpression<'a>) -> T;
    fn visit_assign_expression(&'a self, assign_exp: &AssignmentExpression<'a>) -> T;
    fn visit_index_expression(&'a self, index_exp: &IndexExpression<'a>) -> T;
    fn visit_call_expression(&'a self, call_exp: &CallExpression<'a>) -> T;
    fn visit_array_expression(&'a self, array_exp: &ArrayExpression<'a>) -> T;
    fn visit_ternary_expression(&'a self, tern_exp: &TernaryExpression<'a>) -> T;
    fn visit_range_expression(&'a self, rang_exp: &RangeExpression<'a>) -> T;
    fn visit_function_expression(&'a self, fn_exp: &FnExpression<'a>) -> T;
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
            Statement::LoopStmnt(_) => todo!(),
            Statement::ForStatement(_) => todo!(),
            Statement::WhileStatement(_) => todo!(),
            Statement::PublicModifier(_) => todo!(),
            Statement::ExpressionStatement(exp_stmnt) => self.visit_expression_statement(exp_stmnt),
            Statement::BlockStatement(block) => self.visit_block(block),
            Statement::UseImport(_) => todo!(),
            Statement::ReturnStatement(_) => todo!(),
            Statement::CrashStmnt(_) => todo!(),
            Statement::EmptyStatement(_) => self.visit_empty_statement(statement),
            Statement::TryBlock(_) => todo!(),
            Statement::Function(_) => todo!(),
            Statement::TypeAlias(_) => todo!(),
            Statement::Interface(_) => todo!(),
            Statement::Enum(_) => todo!(),
            Statement::Class(_) => todo!(),
            Statement::Module(_) => todo!(),
            Statement::Record(_) => todo!(),
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
    fn visit_for_statement(&'a self, for_loop: &ForLoop<'a>);
    fn visit_while_statement(&'a self, while_stmnt: &WhileStatement<'a>);
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
    fn visit_enum_declaration(&'a self, enum_: &Enum<'a>);
    fn visit_record_declaration(&'a self, record: &Record<'a>);
    fn visit_class_declaration(&'a self, class: &Class<'a>);
    fn visit_property(&'a self, property: &Property<'a>) -> T;
    fn visit_method(&'a self, method: &Method<'a>) -> T;
    fn visit_constructor(&'a self, method: &Method<'a>) -> T;
    fn visit_attribute(&'a self, attrib: &Attribute<'a>) -> T;
    fn visit_implement(&'a self, implement: &Implement<'a>) -> T;
    fn visit_record_mapping(&'a self, map: &Mapping<'a>);
    fn visit_variant(&'a self, variant: &Variant<'a>);
    fn visit_parameter(&'a self, param: &Parameter<'a>) -> T;
    fn visit_type_alias(&'a self, type_alias: &TypeAlias<'a>);
    fn visit_type_label(&'a self, label: &Type<'a>) -> T;
    fn visit_concrete_type(&'a self, concrete_type: &ConcreteType<'a>) -> T;
    fn visit_functional_type(&'a self, functional_type: &FunctionType<'a>) -> T;
    fn visit_interface(&'a self, interface: &Interface<'a>);
    fn visit_generic_argument(&'a self, argument: &GenericArgument);
}
