#![allow(unused_variables)]
#![allow(dead_code)]

use crate::{ResolveError, Symbol, SymbolOrError, SymbolType};
use ast::{Expression, Location, Operator, Statement, Visitor};
use errors::SemanticError;
use parser::{Parser, ParserError};
use std::cell::RefCell;
use utils::Stage;

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
            Expression::IdentifierExpression(i) => self.ident(i),
            Expression::StringExpression(s) => self.string(s),
            Expression::NumericExpression(n) => self.number(n),
            Expression::BooleanExpression(b) => self.boolean(b),
            Expression::CharacterExpression(c) => self.character(c),
            Expression::SelfExpression(_) => todo!(),
            Expression::BinaryExpression(b) => self.binary_exp(b),
            Expression::LogicalExpression(l) => self.logical_exp(l),
            Expression::UnaryExpression(u) => self.unary_exp(u),
            Expression::CallExpression(_) => todo!(),
            Expression::ArrayExpression(a) => self.array_exp(a),
            Expression::IndexExpression(i) => self.index_exp(i),
            Expression::DotExpression(_) => todo!(),
            Expression::NamespaceExpression(_) => todo!(),
            Expression::RangeExpression(r) => self.range_exp(r),
            Expression::TernaryExpression(t) => self.tern_exp(t),
            Expression::AssignmentExpression(a) => self.assign_exp(a),
            Expression::FnExpression(_) => todo!(),
        }
    }
    // Retrieves the type of an identifier.
    fn ident(&'a self, ident: &ast::Identifier<'a>) -> SymbolOrError {
        match self.stage.borrow().lookup(ident.value) {
            Some(d) => Ok(d.clone()),
            // Store error if the value is undefined.
            None => Err((
                SemanticError::UndeclaredVariable(ident.value.to_string()),
                ident.span,
            )),
        }
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
    /// Type checks a binary expression.
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
    /// Type checks a logical expression.
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

    /// Type checks an assignment expression.
    fn assign_exp(&'a self, assign_exp: &ast::AssignmentExpression<'a>) -> SymbolOrError {
        let mut rhs_symbol = self.expression(&assign_exp.operands[1])?;
        let lhs_symbol = self.expression(&assign_exp.operands[0])?;
        match assign_exp.operator {
            Operator::AddAssign => rhs_symbol = lhs_symbol.add(rhs_symbol)?,
            Operator::SubtractAssign => {
                rhs_symbol = lhs_symbol.operate(rhs_symbol, &Operator::Subtract)?
            }
            Operator::DivideAssign => {
                rhs_symbol = lhs_symbol.operate(rhs_symbol, &Operator::Divide)?
            }
            Operator::MultiplyAssign => rhs_symbol = lhs_symbol.mul(rhs_symbol)?,
            Operator::LogicalAndAssign => {
                rhs_symbol = lhs_symbol.andor(rhs_symbol, &Operator::LogicalAnd)?
            }
            Operator::LogicalOrAssign => {
                rhs_symbol = lhs_symbol.andor(rhs_symbol, &Operator::LogicalOr)?
            }
            _ => {}
        }
        if lhs_symbol.is_unknown() && rhs_symbol.is_unknown() {
            Err((SemanticError::UnknownAssignment, lhs_symbol.span))
        } else if rhs_symbol.is_nil() {
            Err((SemanticError::AssigningToNil, lhs_symbol.span))
        } else if lhs_symbol._type != rhs_symbol._type {
            Err((
                SemanticError::InconsistentAssignment(lhs_symbol._type, rhs_symbol._type),
                assign_exp.span,
            ))
        } else {
            Ok(lhs_symbol)
        }
    }

    /// Type checks an index expression.
    fn index_exp(&'a self, index_exp: &ast::IndexExpression<'a>) -> SymbolOrError {
        let accessor_symbol = self.expression(&index_exp.accessor_and_property[0])?;
        let property_symbol = self.expression(&index_exp.accessor_and_property[1])?;
        let element_type;
        if let SymbolType::Array(x) = accessor_symbol._type {
            element_type = *x;
        } else {
            return Err((
                SemanticError::InvalidIndex(accessor_symbol._type),
                accessor_symbol.span,
            ));
        }
        if let SymbolType::Number = property_symbol._type {
            Ok(Symbol {
                _type: element_type,
                span: index_exp.span,
            })
        } else {
            return Err((
                SemanticError::InvalidIndexer(property_symbol._type),
                property_symbol.span,
            ));
        }
    }

    fn call_exp(&'a self, call_exp: &ast::CallExpression<'a>) -> SymbolOrError {
        todo!()
    }

    /// Type checks an array expression.
    fn array_exp(&'a self, array_exp: &ast::ArrayExpression<'a>) -> SymbolOrError {
        if array_exp.elements.len() == 0 {
            Ok(Symbol::array(SymbolType::Unknown, array_exp.span))
        } else {
            // Match the types of all elements in the array against the first element.
            let first_type = self.expression(&array_exp.elements[0])?._type;
            for child_expression in &array_exp.elements {
                let child_symbol = self.expression(child_expression)?;
                if child_symbol._type != first_type {
                    return Err((
                        SemanticError::HeterogenousArray(first_type, child_symbol._type),
                        child_symbol.span,
                    ));
                }
            }
            Ok(Symbol::array(first_type, array_exp.span))
        }
    }

    /// Type checks a ternary expression.
    fn tern_exp(&'a self, tern_exp: &ast::TernaryExpression<'a>) -> SymbolOrError {
        let test_symbol = self.expression(&tern_exp.test)?;
        if test_symbol._type != SymbolType::Boolean {
            return Err((
                SemanticError::InvalidTernaryTest(test_symbol._type),
                test_symbol.span,
            ));
        }
        let consequent_symbol = self.expression(&tern_exp.consequent)?;
        let alternate_symbol = self.expression(&tern_exp.alternate)?;

        if consequent_symbol._type != alternate_symbol._type {
            return Err((
                SemanticError::InconsistentTernarySides(
                    consequent_symbol._type,
                    alternate_symbol._type,
                ),
                alternate_symbol.span,
            ));
        }
        Ok(consequent_symbol)
    }

    /// Type checks a range expression.
    fn range_exp(&'a self, rang_exp: &ast::RangeExpression<'a>) -> SymbolOrError {
        let lower_symbol = self.expression(&rang_exp.boundaries[0])?;
        let upper_symbol = self.expression(&rang_exp.boundaries[1])?;
        match (&lower_symbol._type, &upper_symbol._type) {
            (SymbolType::Character, SymbolType::Character)
            | (SymbolType::Number, SymbolType::Number) => Ok(lower_symbol),
            _ => return Err((SemanticError::InvalidRangeBoundaries, rang_exp.span)),
        }
    }

    fn fn_exp(&'a self, fn_exp: &ast::FnExpression<'a>) -> SymbolOrError {
        todo!()
    }

    fn statement(&'a self, statement: &ast::Statement<'a>) {
        match statement {
            Statement::IfStatement(_) => todo!(),
            Statement::PrintLnStatement(p) => self.println_statement(p),
            Statement::PrependStatement(_) => todo!(),
            Statement::VariableDeclaration(v) => self.var_decl(v),
            Statement::Break(_) => todo!(),
            Statement::Continue(_) => todo!(),
            Statement::TestBlock(tb) => self.test_block(tb),
            Statement::LoopStmnt(_) => todo!(),
            Statement::ForStatement(_) => todo!(),
            Statement::WhileStatement(_) => todo!(),
            Statement::PublicModifier(_) => todo!(),
            Statement::ExpressionStatement(e) => self.exp_statement(e),
            Statement::BlockStatement(b) => self.block(b),
            Statement::UseImport(_) => todo!(),
            Statement::ReturnStatement(_) => todo!(),
            Statement::CrashStmnt(_) => todo!(),
            Statement::EmptyStatement(e) => {}
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
        match self.expression(&println_stmnt.argument) {
            Ok(_) => {}
            Err(e) => self.diagnostics.borrow_mut().push(e),
        };
    }

    fn prepend_statement(&'a self, prepend_stmnt: &ast::PrependStatement<'a>) {
        todo!()
    }

    fn var_decl(&'a self, var_decl: &ast::VariableDeclaration<'a>) {
        let name = var_decl.name.value;
        // Block double declaration of the same name in the same scope.
        if self.stage.borrow().get(name).is_some() {
            self.diagnostics.borrow_mut().push((
                SemanticError::AlreadyDeclared(name.to_string()),
                var_decl.span,
            ));
            return;
        }
        let symbol;
        // Infer type from assigned expression.
        if let Some(init) = &var_decl.initializer {
            match self.expression(init) {
                Ok(inferred) => {
                    if inferred.is_nil() {
                        symbol = Symbol::unknown(var_decl.span);
                        self.diagnostics
                            .borrow_mut()
                            .push((SemanticError::AssigningToNil, var_decl.span))
                    } else {
                        symbol = inferred
                    }
                }
                Err(e) => {
                    self.diagnostics.borrow_mut().push(e);
                    symbol = Symbol::unknown(var_decl.span)
                }
            }
        } else {
            symbol = Symbol::unknown(var_decl.span);
        }
        self.stage.borrow_mut().set(name, symbol);
    }

    fn breack(&'a self, brk: &ast::Break<'a>) {
        todo!()
    }

    fn kontinue(&'a self, cont: &ast::Continue<'a>) {
        todo!()
    }

    fn test_block(&'a self, test_block: &ast::TestBlock<'a>) {
        if self.stage.borrow().depth() > 0 {
            self.diagnostics
                .borrow_mut()
                .push((SemanticError::IllegalTestBlock, test_block.span))
        }
        self.block(&test_block.body)
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
        self.stage.borrow_mut().create_inner();
        for statement in &block.body {
            self.statement(statement);
        }
        self.stage.borrow_mut().ascend();
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
