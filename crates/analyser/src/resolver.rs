#![allow(unused_variables)]
#![allow(dead_code)]

use ast::{Expression, Location, Operator, Statement, Visitor};
use parser::{Parser, ParserError};
use std::cell::RefCell;
use utils::Stage;

use crate::{ResolveError, Symbol, SymbolOrError, SymbolType};

pub struct Resolver<'a> {
    parser: &'a Parser<'a>,
    stage: RefCell<Stage<&'a str, Symbol>>,
    pub diagnostics: RefCell<Vec<ResolveError>>,
    pub warnings: RefCell<Vec<ResolveError>>,
}

impl<'a> Resolver<'a> {
    pub fn new(parser: &'a Parser<'a>) -> Self {
        Resolver {
            diagnostics: RefCell::new(Vec::new()),
            stage: RefCell::new(Stage::new()),
            warnings: RefCell::new(Vec::new()),
            parser,
        }
    }
    pub fn resolve(&'a self) -> Result<Stage<&'a str, Symbol>, Vec<ParserError>> {
        if self.parser.diagnostics.borrow().len() > 0 {
            Err(self.parser.diagnostics.take())
        } else {
            for statement in self.parser.statements.take() {
                self.statement(&statement);
            }
            Ok(self.stage.take())
        }
    }
}

impl<'a> Visitor<'a, SymbolOrError> for Resolver<'a> {
    fn expression(&'a self, exp: &ast::Expression<'a>) -> SymbolOrError {
        match exp {
            Expression::IdentifierExpression(_) => todo!(),
            Expression::StringExpression(s) => self.string(s),
            Expression::NumericExpression(n) => self.number(n),
            Expression::BooleanExpression(b) => self.boolean(b),
            Expression::CharacterExpression(c) => self.character(c),
            Expression::SelfExpression(_) => todo!(),
            Expression::BinaryExpression(b) => self.binary_exp(b),
            Expression::LogicalExpression(l) => self.logical_exp(l),
            Expression::UnaryExpression(_) => todo!(),
            Expression::CallExpression(_) => todo!(),
            Expression::ArrayExpression(_) => todo!(),
            Expression::IndexExpression(_) => todo!(),
            Expression::DotExpression(_) => todo!(),
            Expression::NamespaceExpression(_) => todo!(),
            Expression::RangeExpression(_) => todo!(),
            Expression::TernaryExpression(_) => todo!(),
            Expression::AssignmentExpression(_) => todo!(),
            Expression::FnExpression(_) => todo!(),
        }
    }

    fn ident(&'a self, ident: &ast::Identifier<'a>) -> SymbolOrError {
        todo!()
    }

    fn string(&'a self, string: &ast::TextString<'a>) -> SymbolOrError {
        Ok(Symbol {
            _type: SymbolType::String,
            span: string.get_range(),
        })
    }

    fn number(&'a self, number: &ast::Number<'a>) -> SymbolOrError {
        Ok(Symbol {
            _type: SymbolType::Number,
            span: number.get_range(),
        })
    }

    fn character(&'a self, charac: &ast::Character<'a>) -> SymbolOrError {
        Ok(Symbol {
            _type: SymbolType::Character,
            span: charac.get_range(),
        })
    }

    fn boolean(&'a self, boolean: &ast::Boolean<'a>) -> SymbolOrError {
        Ok(Symbol {
            _type: SymbolType::Boolean,
            span: boolean.get_range(),
        })
    }

    fn self_exp(&'a self, self_: &ast::SelfExpression) -> SymbolOrError {
        todo!()
    }

    fn binary_exp(&'a self, bin_exp: &ast::BinaryExpression<'a>) -> SymbolOrError {
        let [left, right] = [
            self.expression(&bin_exp.operands[0])?,
            self.expression(&bin_exp.operands[1])?,
        ];
        match bin_exp.operator {
            Operator::Add => left.add(right),
            Operator::Multiply => left.mul(right),
            Operator::BitwiseOr
            | Operator::BitwiseAnd
            | Operator::PowerOf
            | Operator::Remainder
            | Operator::Divide
            | Operator::Subtract
            | Operator::BitwiseLeftShift
            | Operator::BitwiseRightShift => left.operate(right, &bin_exp.operator),
            Operator::Equals | ast::Operator::NotEquals => left.equate(right, &bin_exp.operator),
            Operator::LessThan
            | Operator::GreaterThan
            | Operator::GreaterThanOrEquals
            | Operator::LessThanOrEquals => left.compare(right, &bin_exp.operator),
            _ => unreachable!(),
        }
    }

    fn logical_exp(&'a self, log_exp: &ast::LogicalExpression<'a>) -> SymbolOrError {
        let [left, right] = [
            self.expression(&log_exp.operands[0])?,
            self.expression(&log_exp.operands[1])?,
        ];
        match log_exp.operator {
            Operator::LogicalAnd | Operator::LogicalOr => left.andor(right, &log_exp.operator),
            _ => unreachable!(),
        }
    }

    fn dot_exp(&'a self, dot_exp: &ast::DotExpression<'a>) -> SymbolOrError {
        todo!()
    }

    fn unary_exp(&'a self, unary_exp: &ast::UnaryExpression<'a>) -> SymbolOrError {
        match unary_exp.operator {
            Operator::LogicalNot => self.expression(&unary_exp.operand)?.negate(),
            Operator::BitWiseNot => Ok(Symbol {
                _type: SymbolType::Number,
                span: unary_exp.operand.get_range(),
            }),
            _ => unreachable!(),
        }
    }

    fn namespace_exp(&'a self, namespace_exp: &ast::NamespaceExpression<'a>) -> SymbolOrError {
        todo!()
    }

    fn assign_exp(&'a self, assign_exp: &ast::AssignmentExpression<'a>) -> SymbolOrError {
        todo!()
    }

    fn index_exp(&'a self, index_exp: &ast::IndexExpression<'a>) -> SymbolOrError {
        todo!()
    }

    fn call_exp(&'a self, call_exp: &ast::CallExpression<'a>) -> SymbolOrError {
        todo!()
    }

    fn array_exp(&'a self, array_exp: &ast::ArrayExpression<'a>) -> SymbolOrError {
        todo!()
    }

    fn tern_exp(&'a self, tern_exp: &ast::TernaryExpression<'a>) -> SymbolOrError {
        todo!()
    }

    fn range_exp(&'a self, rang_exp: &ast::RangeExpression<'a>) -> SymbolOrError {
        todo!()
    }

    fn fn_exp(&'a self, fn_exp: &ast::FnExpression<'a>) -> SymbolOrError {
        todo!()
    }

    fn statement(&'a self, statement: &ast::Statement<'a>) {
        match statement {
            Statement::IfStatement(_) => todo!(),
            Statement::PrintLnStatement(_) => todo!(),
            Statement::PrependStatement(_) => todo!(),
            Statement::VariableDeclaration(v) => self.var_decl(v),
            Statement::Break(_) => todo!(),
            Statement::Continue(_) => todo!(),
            Statement::TestBlock(_) => todo!(),
            Statement::LoopStmnt(_) => todo!(),
            Statement::ForStatement(_) => todo!(),
            Statement::WhileStatement(_) => todo!(),
            Statement::PublicModifier(_) => todo!(),
            Statement::ExpressionStatement(e) => self.exp_statement(e),
            Statement::BlockStatement(_) => todo!(),
            Statement::UseImport(_) => todo!(),
            Statement::ReturnStatement(_) => todo!(),
            Statement::CrashStmnt(_) => todo!(),
            Statement::EmptyStatement(_) => todo!(),
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

    fn if_statement(&'a self, if_stmnt: &ast::IfStatement<'a>) {
        todo!()
    }

    fn println_statement(&'a self, println_stmnt: &ast::PrintLnStatement<'a>) {
        todo!()
    }

    fn prepend_statement(&'a self, prepend_stmnt: &ast::PrependStatement<'a>) {
        todo!()
    }

    fn var_decl(&'a self, var_decl: &ast::VariableDeclaration<'a>) {}

    fn breack(&'a self, brk: &ast::Break<'a>) {
        todo!()
    }

    fn kontinue(&'a self, cont: &ast::Continue<'a>) {
        todo!()
    }

    fn test_block(&'a self, test_block: &ast::TestBlock<'a>) {
        todo!()
    }

    fn loop_statement(&'a self, loop_stmnt: &ast::Loop<'a>) {
        todo!()
    }

    fn for_statement(&'a self, for_loop: &ast::ForLoop<'a>) {
        todo!()
    }

    fn while_statement(&'a self, while_stmnt: &ast::WhileStatement<'a>) {
        todo!()
    }

    fn public_mod(&'a self, public_mod: &ast::PublicModifier<'a>) {
        todo!()
    }

    fn exp_statement(&'a self, exp_stmnt: &ast::ExpressionStatement<'a>) {
        match self.expression(&exp_stmnt.expression) {
            Ok(_) => {}
            Err(err) => self.diagnostics.borrow_mut().push(err),
        }
    }

    fn block(&'a self, block: &ast::Block<'a>) {
        todo!()
    }

    fn use_import(&'a self, use_stmnt: &ast::UseImport<'a>) {
        todo!()
    }

    fn return_statement(&'a self, return_stmnt: &ast::ReturnStatement<'a>) {
        todo!()
    }

    fn crash(&'a self, crash: &ast::CrashStatement<'a>) {
        todo!()
    }

    fn empty_statement(&'a self, empty: &ast::ExpressionStatement<'a>) {
        todo!()
    }

    fn try_block(&'a self, try_block: &ast::TryBlock<'a>) {
        todo!()
    }

    fn function(&'a self, function: &ast::Function<'a>) {
        todo!()
    }

    fn enum_declaration(&'a self, enum_: ast::Enum<'a>) {
        todo!()
    }

    fn record_declaration(&'a self, record: ast::Record<'a>) {
        todo!()
    }

    fn mapping(&'a self, map: ast::Mapping<'a>) {
        todo!()
    }

    fn variant(&'a self, variant: ast::Variant<'a>) {
        todo!()
    }

    fn parameter(&'a self, param: ast::Parameter<'a>) {
        todo!()
    }

    fn type_alias(&'a self, type_alias: &ast::TypeAlias<'a>) {
        todo!()
    }

    fn interface(&'a self, interface: &ast::Interface<'a>) {
        todo!()
    }

    fn gen_arg(&'a self, argument: ast::GenericArgument) {
        todo!()
    }
}
