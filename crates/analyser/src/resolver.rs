#![allow(unused_variables)]
#![allow(dead_code)]

use crate::{
    FunctionType, ParameterSymbol, ResolveError, Symbol, SymbolOrError, SymbolType, TypeAlias,
};
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
            let primitives = Symbol::primitives();
            for primitive in primitives {
                self.stage.borrow_mut().set(primitive.0, primitive.1);
            }
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
            Expression::CallExpression(c) => self.call_exp(c),
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
            Some(d) => {
                if let SymbolType::Alias(_) = d._type {
                    return Err((
                        SemanticError::AliasUsedAsValue(ident.value.to_string()),
                        ident.span,
                    ));
                } else {
                    Ok(d.clone())
                }
            }
            // Store error if the value is undefined.
            None => Err((
                SemanticError::Undeclared(ident.value.to_string()),
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
        let element_symbol;
        if let SymbolType::Array(x) = accessor_symbol._type {
            element_symbol = *x;
        } else {
            return Err((
                SemanticError::InvalidIndex(accessor_symbol._type),
                accessor_symbol.span,
            ));
        }
        if let SymbolType::Number = property_symbol._type {
            Ok(element_symbol)
        } else {
            return Err((
                SemanticError::InvalidIndexer(property_symbol._type),
                property_symbol.span,
            ));
        }
    }

    fn call_exp(&'a self, call_exp: &ast::CallExpression<'a>) -> SymbolOrError {
        let callee_symbol = self.expression(&call_exp.callee)?;
        if let SymbolType::Function(f) = callee_symbol._type {
            if f.parameters.len() != call_exp.arguments.len() {
                return Err((
                    SemanticError::UnequalArgs(f.parameters.len(), call_exp.arguments.len()),
                    call_exp.span,
                ));
            }
            let mut i = 0;
            while i < f.parameters.len() {
                let arg_symbol = self.expression(&call_exp.arguments[i])?;
                if f.parameters[i]._type != arg_symbol._type
                    && f.parameters[i]._type != SymbolType::Unknown
                {
                    return Err((
                        SemanticError::ParameterMismatch(
                            f.parameters[i]._type.clone(),
                            arg_symbol._type,
                        ),
                        call_exp.arguments[i].get_range(),
                    ));
                }
                i += 1;
            }
            Ok(*f.return_type.clone())
        } else {
            Err((
                SemanticError::Uncallable(callee_symbol._type),
                callee_symbol.span,
            ))
        }
    }

    /// Type checks an array expression.
    fn array_exp(&'a self, array_exp: &ast::ArrayExpression<'a>) -> SymbolOrError {
        if array_exp.elements.len() == 0 {
            Ok(Symbol::array(
                Symbol::unknown(array_exp.span),
                array_exp.span,
            ))
        } else {
            // Match the types of all elements in the array against the first element.
            let first_symbol = self.expression(&array_exp.elements[0])?;
            for child_expression in &array_exp.elements {
                let child_symbol = self.expression(child_expression)?;
                if child_symbol._type != first_symbol._type {
                    return Err((
                        SemanticError::HeterogenousArray(first_symbol._type, child_symbol._type),
                        child_symbol.span,
                    ));
                }
            }
            Ok(Symbol::array(first_symbol, array_exp.span))
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
    /// Type checks a statement.
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
            Statement::Function(f) => self.function(f),
            Statement::TypeAlias(t) => self.type_alias(t),
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
        let mut inferred_symbol;
        // Infer type from assigned expression.
        if let Some(init) = &var_decl.initializer {
            match self.expression(init) {
                Ok(i) => {
                    if i.is_nil() {
                        inferred_symbol = Symbol::unknown(var_decl.span);
                        self.diagnostics
                            .borrow_mut()
                            .push((SemanticError::AssigningToNil, var_decl.span))
                    } else {
                        inferred_symbol = i
                    }
                }
                Err(e) => {
                    self.diagnostics.borrow_mut().push(e);
                    inferred_symbol = Symbol::unknown(var_decl.span)
                }
            }
        } else {
            inferred_symbol = Symbol::unknown(var_decl.span);
        }
        // Compare the inferred type with the assigned type label.
        if let Some(t) = &var_decl.type_label {
            let given_symbol;
            match self.type_label(t) {
                Ok(s) => given_symbol = s,
                Err(e) => {
                    given_symbol = Symbol::unknown(var_decl.span);
                    self.diagnostics.borrow_mut().push(e);
                }
            }

            if !(given_symbol._type == inferred_symbol._type || given_symbol.is_unknown()) {
                self.diagnostics.borrow_mut().push((
                    SemanticError::InconsistentAssignment(
                        given_symbol._type.clone(),
                        inferred_symbol._type,
                    ),
                    var_decl.span,
                ));
                inferred_symbol = given_symbol;
            }
        }
        self.stage.borrow_mut().set(name, inferred_symbol);
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
        self.statement(&public_mod.statement);
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
        let name = function.name.value;
        let mut parameter_symbols = vec![];
        for parameter_node in &function.parameters {
            let _type;
            match self.parameter(parameter_node) {
                Err(e) => {
                    self.diagnostics.borrow_mut().push(e);
                    _type = SymbolType::Unknown
                }
                Ok(s) => _type = s._type,
            };
            parameter_symbols.push(ParameterSymbol {
                name: parameter_node.name.value.to_string(),
                _type,
                span: parameter_node.span,
            });
        }
        // If the return type is either invalid or absent, set it to nil.
        let return_type = match &function.return_type {
            Some(s) => self.type_label(&s).unwrap_or(Symbol::nil(function.span)),
            None => Symbol::nil(function.span),
        };

        if let Some(sym) = self.stage.borrow().get(name) {
            self.diagnostics.borrow_mut().push((
                SemanticError::AlreadyDeclared(name.to_string()),
                function.name.span,
            ));
            return;
        }
        self.stage.borrow_mut().set(
            name,
            Symbol {
                _type: SymbolType::Function(FunctionType {
                    parameters: parameter_symbols,
                    return_type: Box::new(return_type),
                }),
                span: function.span,
            },
        )
    }

    fn enum_declaration(&'a self, enum_: &ast::Enum<'a>) {
        todo!()
    }

    fn record_declaration(&'a self, record: &ast::Record<'a>) {
        todo!()
    }

    fn mapping(&'a self, map: &ast::Mapping<'a>) {
        todo!()
    }

    fn variant(&'a self, variant: &ast::Variant<'a>) {
        todo!()
    }

    fn parameter(&'a self, param: &ast::Parameter<'a>) -> SymbolOrError {
        match &param.label {
            Some(label) => Ok(self.type_label(label)?),
            None => Ok(Symbol::unknown(param.span)),
        }
    }

    /// Type checks type aliases.
    fn type_alias(&'a self, type_alias: &ast::TypeAlias<'a>) {
        let name = type_alias.name.value;
        let aliased_symbol;
        match self.type_label(&type_alias.value) {
            Ok(s) => aliased_symbol = s,
            Err(e) => {
                self.diagnostics.borrow_mut().push(e);
                aliased_symbol = Symbol::unknown(type_alias.span)
            }
        }
        let arguments = Vec::new();
        self.stage.borrow_mut().set(
            name,
            Symbol {
                _type: SymbolType::Alias(TypeAlias {
                    actual_symbol: Box::new(aliased_symbol),
                    arguments,
                }),
                span: type_alias.span,
            },
        );
    }

    fn interface(&'a self, interface: &ast::Interface<'a>) {
        todo!()
    }

    fn gen_arg(&'a self, argument: &ast::GenericArgument) {
        todo!()
    }

    fn type_label(&'a self, label: &ast::Type<'a>) -> SymbolOrError {
        match label {
            ast::Type::Concrete(c) => self.concrete_type(c),
            ast::Type::Function(f) => self.functional_type(f),
            ast::Type::Dot(_) => todo!(),
        }
    }

    fn concrete_type(&'a self, concrete_type: &ast::ConcreteType<'a>) -> SymbolOrError {
        for argument in &concrete_type.arguments {
            let arg = self.type_label(&argument)?;
        }
        match self.stage.borrow().lookup(concrete_type.name.value) {
            Some(d) => match &d._type {
                SymbolType::Class(_) => Ok(d.clone()),
                SymbolType::Alias(a) => Ok(*a.actual_symbol.clone()),
                _ => {
                    return Err((
                        SemanticError::ValueUsedAsAlias(concrete_type.name.value.to_string()),
                        concrete_type.span,
                    ))
                }
            },
            // Store error if the type is undefined.
            None => Err((
                SemanticError::Undeclared(concrete_type.name.value.to_string()),
                concrete_type.span,
            )),
        }
    }

    fn functional_type(&'a self, functional_type: &ast::FunctionType<'a>) -> SymbolOrError {
        todo!()
    }
}
