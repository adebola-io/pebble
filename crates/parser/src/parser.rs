use std::{cell::RefCell, marker::PhantomData};

use crate::scanner::Scanner;
use ast::{
    precedence_of, Block, BracketKind, Break, CrashStatement, Expression, Function,
    FunctionalSignature, Identifier, IfStatement, Injunction, Keyword, LiteralKind, Location, Loop,
    Operator, Parameter, PrependStatement, PrintLnStatement, PublicModifier, Punctuation,
    RecoverBlock, ReturnStatement, Statement, TestBlock, TextSpan, Token, TokenIdentifier,
    TokenKind, TryBlock, Type, WhileStatement,
};
use utils::Stack;

type ParserError = (&'static str, TextSpan);
type NodeOrError<T> = Result<T, ParserError>;

/// The provider is a pseudo iterator that supplies tokens to the parser.
pub struct Provider {
    pub scanner: Scanner,
    pub index: usize,
}
impl Provider {
    pub fn current(&self) -> &Token {
        &self.scanner.tokens[self.index]
    }
    pub fn next(&mut self) {
        self.index += 1
    }
}

/// The recursive descent parser that goes over the array of tokens scanned from the source text, and iteratively builds out the AST syntax tree.
pub struct Parser<'a> {
    provider: RefCell<Provider>,
    operators: RefCell<Stack<&'a Operator>>,
    pub diagnostics: RefCell<Vec<ParserError>>,
    pub statements: RefCell<Vec<Statement<'a>>>,
}

impl<'a> Parser<'a> {
    /// Return a reference to the current token.
    fn token(&self) -> &Token {
        unsafe { self.provider.try_borrow_unguarded().unwrap().current() }
    }
    /// Shift to the next token in the stream.
    fn advance(&self) {
        self.provider.borrow_mut().next()
    }
    /// Check if the stream has ended.
    fn end(&'a self) -> bool {
        self.provider.borrow().current().is_eof()
    }
    /// Push an operator unto the operator stack.
    fn push_operator(&'a self, operator: &'a Operator) {
        self.operators.borrow_mut().push(operator)
    }
    /// Check if the current operator in the stream is lower in precedence to the operator previously parsed.
    fn is_lower_precedence(&'a self, operator: &Operator) -> bool {
        match self.operators.borrow().top() {
            None => false,
            Some(op) => {
                if precedence_of(operator) <= precedence_of(op) {
                    true
                } else {
                    false
                }
            }
        }
    }
    /// Store an error that has been encountered.
    fn store_error(&self, error: ParserError) {
        self.diagnostics.borrow_mut().push(error)
    }
}

impl<'a> Parser<'a> {
    pub fn new(provider: Provider) -> Parser<'a> {
        Parser {
            provider: RefCell::new(provider),
            operators: RefCell::new(Stack::new()),
            diagnostics: RefCell::new(vec![]),
            statements: RefCell::new(vec![]),
        }
    }
    pub fn parse(&'a self) {
        // Continously parse statements and store them in a statements array until the end of the file is reached.
        while !self.end() {
            let parsed = self.statement();
            match parsed {
                Ok(stmnt) => {
                    self.statements.borrow_mut().push(stmnt);
                }
                Err(e) => {
                    // Errors are checked at the statement boundary.
                    // If an error is found while parsing, the parser stores it, skips over till it finds the next statement, then continues parsing from there.
                    self.store_error(e);
                    while !(self.end() || self.token().is_semi_colon()) {
                        self.advance();
                    }
                }
            };
        }
    }
    /// Parse a single statement.
    fn statement(&'a self) -> NodeOrError<Statement<'a>> {
        match &self.token().kind {
            TokenKind::Keyword(Keyword::Injunction(injunction)) => self.injunction(injunction),
            TokenKind::Keyword(Keyword::Fn) => self.expression_statement(),
            TokenKind::Keyword(keyword) => self.control_statement(keyword),
            TokenKind::Punctuation(Punctuation::Bracket(BracketKind::LeftCurly)) => {
                self.block_statement()
            }
            _ => self.expression_statement(),
        }
    }
    fn expression_statement(&'a self) -> NodeOrError<Statement<'a>> {
        let parsed = self.expression();
        match parsed {
            Err(e) => Err(e),
            Ok(exp) => {
                if self.token().is_semi_colon() {
                    self.advance();
                    Ok(Statement::create_expr_stmnt(exp))
                } else {
                    // Every expression statement must end with a semi-colon.
                    Err(("Expected a semi-colon.", self.token().span.clone()))
                }
            }
        }
    }
    fn expression(&'a self) -> NodeOrError<Expression<'a>> {
        match &self.token().kind {
            TokenKind::Literal(_) => self.literal(),
            TokenKind::Identifier(_) => self.identifier(),
            TokenKind::Operator(operator) => self.unary_expression(operator),
            TokenKind::Keyword(Keyword::Fn) => self.functional_expression(),
            TokenKind::Punctuation(Punctuation::Bracket(BracketKind::LeftParenthesis)) => {
                self.grouped_expression()
            }
            _ => {
                println!("{:?}", self.token());
                todo!()
            }
        }
    }
    /// Parses a literal token into its respective expression node.
    fn literal(&'a self) -> NodeOrError<Expression<'a>> {
        if let Token {
            span,
            kind: TokenKind::Literal(literal),
        } = self.token()
        {
            let node = match literal.kind {
                LiteralKind::StringLiteral => Expression::create_str_expr(&literal.value, *span),
                LiteralKind::NumericLiteral => Expression::create_num_expr(&literal.value, *span),
                LiteralKind::BooleanLiteral => Expression::create_bool_expr(&literal.value, *span),
                LiteralKind::CharacterLiteral => {
                    Expression::create_char_expr(&literal.value, *span)
                }
            };
            self.advance();
            Ok(self.reparse(node)?)
        } else {
            unreachable!()
        }
    }
    /// Parses an identifier token into an identifier expression.
    fn identifier(&'a self) -> NodeOrError<Expression<'a>> {
        match self.token() {
            Token {
                span,
                kind: TokenKind::Identifier(TokenIdentifier { value }),
            } => {
                let node = Expression::create_ident_expr(value, *span);
                self.advance();
                Ok(self.reparse(node)?)
            }
            _ => unreachable!(),
        }
    }
}

impl<'a> Parser<'a> {
    fn reparse(&'a self, node: Expression<'a>) -> NodeOrError<Expression<'a>> {
        match &self.token().kind {
            TokenKind::Operator(operator) => match operator {
                Operator::Dot => self.dot_expression(node, operator),
                Operator::Namespace => self.namespace_expression(node, operator),
                Operator::RangeBetween => self.range_expression(node, operator),
                Operator::LogicalAnd | Operator::LogicalOr => {
                    self.logical_expression(node, operator)
                }
                Operator::Confirm => self.ternary_expression(node, operator),
                Operator::Add
                | Operator::Multiply
                | Operator::Divide
                | Operator::Subtract
                | Operator::Remainder
                | Operator::LessThan
                | Operator::GreaterThan
                | Operator::GreaterThanOrEquals
                | Operator::LessThanOrEquals
                | Operator::BitwiseLeftShift
                | Operator::BitwiseRightShift
                | Operator::BitwiseOr
                | Operator::BitwiseAnd
                | Operator::Equals
                | Operator::NotEquals
                | Operator::PowerOf => self.binary_expression(node, operator),
                Operator::Assign
                | Operator::AddAssign
                | Operator::DivideAssign
                | Operator::MultiplyAssign
                | Operator::SubtractAssign
                | Operator::LogicalOrAssign
                | Operator::LogicalAndAssign => self.assign_expression(node, operator),
                _ => Ok(node),
            },
            TokenKind::Punctuation(punctuation) => match punctuation {
                Punctuation::Bracket(BracketKind::LeftParenthesis) => self.call_expression(node),
                Punctuation::Bracket(BracketKind::LeftSquare) => self.index_expression(node),
                _ => Ok(node),
            },
            _ => Ok(node),
        }
    }
    fn grouped_expression(&'a self) -> NodeOrError<Expression<'a>> {
        self.advance();
        self.operators.borrow_mut().push(&Operator::Temp);
        let expression = self.expression()?;
        self.operators.borrow_mut().pop();
        self.advance();
        Ok(self.reparse(expression)?)
    }
    /// Parses a dot or member expression.
    fn dot_expression(
        &'a self,
        object: Expression<'a>,
        operator: &'a Operator,
    ) -> NodeOrError<Expression<'a>> {
        if self.is_lower_precedence(operator) {
            Ok(object)
        } else {
            self.advance(); // Move past operator.
            self.operators.borrow_mut().push(operator); // Push the dot operator onto the stack.
            let property = self.expression()?; // Parse the property of the object.
            self.operators.borrow_mut().pop(); // Remove the dot operator from the stack.
            let dot_exp = Expression::create_dot_expr(object, property);
            Ok(self.reparse(dot_exp)?)
        }
    }
    /// Parses a namespace expression.
    fn namespace_expression(
        &'a self,
        namespace: Expression<'a>,
        operator: &'a Operator,
    ) -> NodeOrError<Expression<'a>> {
        if self.is_lower_precedence(operator) {
            Ok(namespace)
        } else {
            self.advance(); // Move past operator.
            self.operators.borrow_mut().push(operator); // Push the namespace operator onto the stack.
            let property = self.expression()?; // Parse the property of the object.
            self.operators.borrow_mut().pop(); // Remove the namespace operator from the stack.
            let dot_exp = Expression::create_namespace_expr(namespace, property);
            Ok(self.reparse(dot_exp)?)
        }
    }
    /// Parses a binary expression.
    fn binary_expression(
        &'a self,
        left: Expression<'a>,
        operator: &'a Operator,
    ) -> NodeOrError<Expression<'a>> {
        if self.is_lower_precedence(&operator) {
            Ok(left)
        } else {
            self.advance(); // Move past operator.
            self.operators.borrow_mut().push(operator); // Push the binary operator onto the stack.
            let right = self.expression()?; // Parse the expression at the right hand side of the binary expression.
            self.operators.borrow_mut().pop(); // Remove the binary operator from the stack.
            let bin_exp = Expression::create_bin_expr(left, operator, right);
            Ok(self.reparse(bin_exp)?)
        }
    }
    /// Parses a call expression.
    fn call_expression(&'a self, callee: Expression<'a>) -> NodeOrError<Expression<'a>> {
        let call_op = Operator::Call;
        if self.is_lower_precedence(&call_op) {
            Ok(callee)
        } else {
            self.advance(); // Move past operator.
            self.operators.borrow_mut().push(&Operator::Temp);
            let mut arguments = vec![];
            let right_bracket = BracketKind::RightParenthesis;
            while !(self.end() || self.token().is_bracket(&right_bracket)) {
                let argument = self.expression()?;
                arguments.push(argument);
                if self.token().is_comma() {
                    self.advance(); // Move past the comma.
                } else if !self.token().is_bracket(&right_bracket) {
                    return Err((
                        "Unexpected token. Expected a function argument.",
                        self.token().span.clone(),
                    ));
                }
            }
            self.operators.borrow_mut().pop();
            if self.end() {
                Err((
                    "Unexpected end of text. Expected a ).",
                    self.token().span.clone(),
                ))
            } else {
                let end = self.token().span.clone()[1];
                self.advance(); // Move past )
                let callexp = Expression::create_call_expr(callee, arguments, end);

                Ok(self.reparse(callexp)?)
            }
        }
    }
    /// Parses an index expression.
    fn index_expression(&'a self, accessor: Expression<'a>) -> NodeOrError<Expression<'a>> {
        let index_op = Operator::Index;
        if self.is_lower_precedence(&index_op) {
            Ok(accessor)
        } else {
            self.advance(); // Move past [
            self.operators.borrow_mut().push(&Operator::Temp);
            let property = self.expression()?; // Parse property.
            self.operators.borrow_mut().pop();
            if !self.token().is_bracket(&BracketKind::RightSquare) {
                return Err(("Expected a ].", self.token().span.clone()));
            }
            let end = self.token().span.clone()[1];
            self.advance(); // Move past ]
            let index_exp = Expression::create_index_expr(accessor, property, end);
            Ok(self.reparse(index_exp)?)
        }
    }
    /// Parses a unary expression.
    fn unary_expression(&'a self, operator: &'a Operator) -> NodeOrError<Expression<'a>> {
        match operator {
            Operator::Add
            | Operator::Subtract
            | Operator::Decrement
            | Operator::Increment
            | Operator::LogicalNot
            | Operator::BitWiseNot => {
                self.operators.borrow_mut().push(operator);
                let start = self.token().span.clone()[0];
                self.advance(); // Move past operator.
                let operand = self.expression()?;
                self.operators.borrow_mut().pop();
                let un_exp = Expression::create_unary_expr(start, operator, operand);
                Ok(self.reparse(un_exp)?)
            }
            _ => Err((
                "Unexpected operator. Expected an expression",
                self.token().span.clone(),
            )),
        }
    }
    fn range_expression(
        &'a self,
        top: Expression<'a>,
        operator: &'a Operator,
    ) -> NodeOrError<Expression<'a>> {
        if self.is_lower_precedence(operator) {
            Ok(top)
        } else {
            self.advance(); // Move past operator.
            self.operators.borrow_mut().push(operator);
            let bottom = self.expression()?;
            self.operators.borrow_mut().pop();
            let range_exp = Expression::create_range_expr(top, bottom);
            Ok(self.reparse(range_exp)?)
        }
    }
    /// Parses a logical expression.
    fn logical_expression(
        &'a self,
        left: Expression<'a>,
        operator: &'a Operator,
    ) -> NodeOrError<Expression<'a>> {
        if self.is_lower_precedence(operator) {
            Ok(left)
        } else {
            self.advance(); // Move past operator.
            self.operators.borrow_mut().push(operator);
            let right = self.expression()?;
            self.operators.borrow_mut().pop();
            let log_exp = Expression::create_logical_expr(left, operator, right);
            Ok(self.reparse(log_exp)?)
        }
    }
    /// Parses a ternary expression.
    fn ternary_expression(
        &'a self,
        test: Expression<'a>,
        operator: &'a Operator,
    ) -> NodeOrError<Expression<'a>> {
        if self.is_lower_precedence(operator) {
            Ok(test)
        } else {
            self.advance(); // Move past operator.
            self.operators.borrow_mut().push(operator);
            let consequent = self.expression()?;
            self.operators.borrow_mut().pop();
            if !self.token().is_colon() {
                return Err(("Expected a colon.", self.token().span.clone()));
            }
            self.advance();
            self.operators.borrow_mut().push(&Operator::Colon);
            let alternate = self.expression()?;
            self.operators.borrow_mut().pop();
            let tern_exp = Expression::create_ternary_expr(test, consequent, alternate);
            Ok(self.reparse(tern_exp)?)
        }
    }
    /// Parses an assignment expression.
    fn assign_expression(
        &'a self,
        left: Expression<'a>,
        operator: &'a Operator,
    ) -> NodeOrError<Expression<'a>> {
        if self.is_lower_precedence(operator) {
            Ok(left)
        } else {
            self.advance(); // Move past operator.
            let right = self.expression()?;
            let assign_exp = Expression::create_assign_expr(left, operator, right);
            Ok(self.reparse(assign_exp)?)
        }
    }
    fn functional_expression(&'a self) -> NodeOrError<Expression<'a>> {
        let start = self.token().span.clone()[0];
        self.advance(); // Move past fn.
        let signature = self.functional_signature()?;
        if signature.name.is_some() {
            return Err((
                "A functional expression cannot be named.",
                self.token().span.clone(),
            ));
        }
        let end = signature.get_range()[1];
        let fn_exp = Expression::FnExpr {
            signature: Box::new(signature),
            span: [start, end],
        };
        Ok(self.reparse(fn_exp)?)
    }
    /// Parses a functional signature.
    fn functional_signature(&'a self) -> NodeOrError<FunctionalSignature<'a>> {
        todo!()
    }
}

/// Injunctions
impl<'a> Parser<'a> {
    fn injunction(&'a self, injunction: &Injunction) -> NodeOrError<Statement<'a>> {
        match injunction {
            Injunction::Function => self.function_declaration(),
            Injunction::Type => todo!(),
            Injunction::Class => todo!(),
            Injunction::Record => todo!(),
            Injunction::Const => todo!(),
            Injunction::Let => todo!(),
            Injunction::Use => todo!(),
            Injunction::Prepend => self.prepend_statement(),
            Injunction::Test => self.test_block(),
            Injunction::Enum => todo!(),
            Injunction::Interface => todo!(),
            Injunction::Implement => todo!(),
            Injunction::Module => self.module(),
            Injunction::Public => self.public_statement(),
            Injunction::Unknown(_) => self.unknown_injunction(),
        }
    }
    /// Parses a function decalaration.
    fn function_declaration(&'a self) -> NodeOrError<Statement<'a>> {
        let start = self.token().span.clone()[0];
        self.advance(); // Move past @function
        let name;
        if let Token {
            kind: TokenKind::Identifier(TokenIdentifier { value }),
            span,
        } = &self.token()
        {
            name = Identifier { value, span: *span };
        } else {
            return Err(("Expected a function name.", self.token().span.clone()));
        }
        let label = self.maybe_generic_label()?;
        let parameters = self.parameters()?;
        let return_type = self.maybe_return_type()?;
        let body = self.block()?;
        let end = self.token().span.clone()[1];
        let decl = Statement::Function(Function {
            name,
            label,
            parameters,
            return_type,
            body,
            span: [start, end],
        });
        Ok(decl)
    }
    /// Parse a generic label that may or may not exist.
    fn maybe_generic_label(&'a self) -> NodeOrError<Option<Type>> {
        todo!()
    }
    /// Parse a return type that may or may not exist.
    fn maybe_return_type(&'a self) -> NodeOrError<Option<Type>> {
        todo!()
    }
    /// Parses a prepend statement.
    fn prepend_statement(&'a self) -> NodeOrError<Statement<'a>> {
        let start = self.token().span.clone()[0];
        self.advance(); // Move past @prepend
        let source = self.expression()?;
        if !self.token().is_semi_colon() {
            Err(("Expected a semicolon.", self.token().span.clone()))
        } else {
            let end = self.token().span.clone()[1];
            self.advance();
            let prep_stat = Statement::PrependStatement(PrependStatement {
                source,
                span: [start, end],
            });
            Ok(prep_stat)
        }
    }
    /// Parses a test block.
    fn test_block(&'a self) -> NodeOrError<Statement<'a>> {
        let start = self.token().span.clone()[0];
        self.advance(); // Move past @test.
        let body = self.block()?;
        let end = body.get_range()[1];
        let test_block = Statement::TestBlock(TestBlock {
            body,
            span: [start, end],
        });
        Ok(test_block)
    }
    /// Parses a public statement.
    fn public_statement(&'a self) -> NodeOrError<Statement<'a>> {
        let start = self.token().span.clone()[0];
        self.advance(); // Move past @public.
        let statement = self.statement()?;
        let end = statement.get_range()[1];
        let pub_stat = Statement::PublicModifier(PublicModifier {
            statement: Box::new(statement),
            span: [start, end],
        });
        Ok(pub_stat)
    }
    fn module(&'a self) -> NodeOrError<Statement<'a>> {
        // let start = self.token().span.clone()[0];
        // let name;
        // self.advance(); // Move past @module.
        // if let Token {
        //     span,
        //     kind: TokenKind::Identifier(TokenIdentifier { value }),
        // } = self.token()
        // {
        //     name = Expression::create_ident_expr(value, *span)
        // } else {
        //     return Err(("Expected a module name.", self.token().span.clone()));
        // }
        todo!()
    }
    fn unknown_injunction(&'a self) -> NodeOrError<Statement<'a>> {
        Err(("Unknown injunction ", self.token().span.clone()))
    }
}

/// Control statements
impl<'a> Parser<'a> {
    /// Parse a control statement.
    fn control_statement(&'a self, keyword: &Keyword) -> NodeOrError<Statement<'a>> {
        match keyword {
            Keyword::If => self.if_statememt(),
            Keyword::Println => self.print_statement(),
            Keyword::While => self.while_statement(),
            Keyword::Return => self.return_statement(),
            Keyword::Crash => self.crash_statement(),
            Keyword::Loop => self.loop_statement(),
            Keyword::Break => self.break_statement(),
            Keyword::Try => self.try_block(),
            _ => todo!(),
        }
    }
    /// Parse the condition of a while loop or an if statement.
    fn condition(&'a self) -> NodeOrError<Expression<'a>> {
        if !self.token().is_bracket(&BracketKind::LeftParenthesis) {
            return Err(("Expected a (", self.token().span.clone()));
        }
        self.advance();
        self.operators.borrow_mut().push(&Operator::Temp);
        let expression = self.expression()?;
        self.operators.borrow_mut().pop();
        if !self.token().is_bracket(&BracketKind::RightParenthesis) {
            return Err(("Expected a )", self.token().span.clone()));
        }
        self.advance();
        Ok(expression)
    }
    /// Parse the consequent of a while loop or an if statement.
    fn consequent(&'a self) -> NodeOrError<Statement<'a>> {
        if self.token().is_bracket(&BracketKind::LeftCurly) {
            self.block_statement()
        } else {
            self.statement()
        }
    }
    fn block_statement(&'a self) -> NodeOrError<Statement<'a>> {
        Ok(Statement::BlockStatement(self.block()?))
    }
    fn block(&'a self) -> NodeOrError<Block<'a>> {
        let start = self.token().span.clone()[0];
        self.advance(); // Move past {
        let close = BracketKind::RightCurly;
        let mut statements = vec![];
        while !(self.end() || self.token().is_bracket(&close)) {
            let statement = self.statement()?;
            statements.push(statement);
        }
        if self.end() {
            return Err(("Expected a }", self.token().span.clone()));
        }
        let end = self.token().span.clone()[1];
        self.advance(); // Move past }
        let block = Block {
            body: statements,
            span: [start, end],
        };
        Ok(block)
    }
    /// Parse an if statement.
    fn if_statememt(&'a self) -> NodeOrError<Statement<'a>> {
        let start = self.token().span.clone()[0];
        self.advance(); // Move past the if.
        let test = self.condition()?;
        let body = self.consequent()?;
        let if_stat;
        let end;
        let alternate;
        if let Token {
            kind: TokenKind::Keyword(Keyword::Else),
            ..
        } = self.token()
        {
            self.advance();
            alternate = self.consequent()?;
            end = self.token().span.clone()[0];
            if_stat = Statement::IfStatement(IfStatement {
                test,
                body: Box::new(body),
                alternate: Some(Box::new(alternate)),
                span: [start, end],
            });
        } else {
            end = self.token().span.clone()[0];
            if_stat = Statement::IfStatement(IfStatement {
                test,
                body: Box::new(body),
                alternate: None,
                span: [start, end],
            });
        }
        if self.token().is_semi_colon() {
            self.advance();
        }
        Ok(if_stat)
    }
    fn while_statement(&'a self) -> NodeOrError<Statement<'a>> {
        let start = self.token().span.clone()[0];
        self.advance(); // Move past while.
        let test = self.condition()?;
        let body = self.consequent()?;
        let end = body.get_range()[1];
        let while_stat = Statement::WhileStatement(WhileStatement {
            test,
            body: Box::new(body),
            span: [start, end],
        });
        Ok(while_stat)
    }
    /// Parses a print statement.
    fn print_statement(&'a self) -> NodeOrError<Statement<'a>> {
        let start = self.token().span.clone()[0];
        self.advance(); // Move past print.
        let argument = self.expression()?;
        if !self.token().is_semi_colon() {
            Err(("Expected a semicolon.", self.token().span.clone()))
        } else {
            let end = self.token().span.clone()[1];
            self.advance(); // Move past ;
            let print_stat = Statement::PrintLnStatement(PrintLnStatement {
                argument,
                span: [start, end],
            });
            Ok(print_stat)
        }
    }
    /// Parses a return statement.
    fn return_statement(&'a self) -> NodeOrError<Statement<'a>> {
        let start = self.token().span.clone()[0];
        self.advance(); // Move past return.
        let argument;
        let end;
        if self.token().is_semi_colon() {
            end = self.token().span.clone()[1];
            self.advance();
            argument = None;
        } else {
            argument = Some(self.expression()?);
            if !self.token().is_semi_colon() {
                return Err(("Expected a semicolon.", self.token().span.clone()));
            } else {
                end = self.token().span.clone()[1];
                self.advance(); // Move past ;
            }
        }
        let ret_stat = Statement::ReturnStatement(ReturnStatement {
            argument,
            span: [start, end],
        });
        Ok(ret_stat)
    }
    /// Parses a loop statement.
    fn loop_statement(&'a self) -> NodeOrError<Statement<'a>> {
        let start = self.token().span.clone()[0];
        self.advance(); // Move past loop.
        let constraint;
        if self.token().is_bracket(&BracketKind::LeftParenthesis) {
            constraint = Some(self.condition()?);
        } else {
            constraint = None;
        }
        let body = self.block()?;
        let end = body.get_range()[1];
        if self.token().is_semi_colon() {
            self.advance();
        }
        let loop_stat = Statement::LoopStmnt(Loop {
            constraint,
            body,
            span: [start, end],
        });
        Ok(loop_stat)
    }
    /// Parses a break statement.
    fn break_statement(&'a self) -> NodeOrError<Statement<'a>> {
        let start = self.token().span.clone()[0];
        self.advance();
        if !self.token().is_semi_colon() {
            return Err(("Expected a semicolon ", self.token().span.clone()));
        }
        let end = self.token().span.clone()[1];
        self.advance();
        let break_stat = Statement::Break(Break {
            span: [start, end],
            phantom: PhantomData,
        });
        Ok(break_stat)
    }
    /// Parses a crash statemnet.
    fn crash_statement(&'a self) -> NodeOrError<Statement<'a>> {
        let start = self.token().span.clone()[0];
        self.advance(); // Move past crash.
        let argument = self.expression()?;
        if !self.token().is_semi_colon() {
            return Err(("Expected a semicolon", self.token().span.clone()));
        }
        let end = self.token().span.clone()[1];
        self.advance();
        let crash_stat = Statement::CrashStmnt(CrashStatement {
            argument,
            span: [start, end],
        });
        Ok(crash_stat)
    }
    /// Parses a try block.
    fn try_block(&'a self) -> NodeOrError<Statement<'a>> {
        let start = self.token().span.clone()[0];
        self.advance(); // Move past try.
        let body = self.block()?;
        let recoverblock;
        let end;
        if let Token {
            kind: TokenKind::Keyword(Keyword::Recover),
            ..
        } = self.token()
        {
            recoverblock = Some(self.recover_block()?);
            end = recoverblock.as_ref().unwrap().get_range()[1];
        } else {
            recoverblock = None;
            end = body.get_range()[1];
        }
        if self.token().is_semi_colon() {
            self.advance();
        }
        println!("{:?}", self.token());
        let try_bloc = Statement::TryBlock(TryBlock {
            body,
            recover: recoverblock,
            span: [start, end],
        });
        Ok(try_bloc)
    }
    /// Parses a recover block.
    fn recover_block(&'a self) -> NodeOrError<RecoverBlock<'a>> {
        let start = self.token().span.clone()[0];
        self.advance(); // Move past recover.
        let params = self.parameters()?;
        let body = self.block()?;
        let end = body.get_range()[1];
        let recover_block = RecoverBlock {
            params,
            body,
            span: [start, end],
        };
        Ok(recover_block)
    }
}

/// Functions and Types
impl<'a> Parser<'a> {
    fn parameters(&'a self) -> NodeOrError<Vec<Parameter<'a>>> {
        if !self.token().is_bracket(&BracketKind::LeftParenthesis) {
            return Err((("Expected an ("), self.token().span.clone()));
        }
        let mut parameters = vec![];
        self.advance(); // Move past (
        while !(self.end() || self.token().is_bracket(&BracketKind::RightParenthesis)) {
            let name;
            let label;
            if let Token {
                kind: TokenKind::Identifier(t),
                span,
            } = self.token()
            {
                name = Identifier {
                    value: &t.value,
                    span: *span,
                }
            } else {
                return Err(("Expected a paramter name", self.token().span.clone()));
            }

            if self.token().is_colon() {
                self.advance(); // Move past :
                label = Some(self.label()?);
            } else {
                label = None;
            }
            let start = name.get_range()[0];
            let end;
            if self.token().is_comma() {
                end = self.token().span.clone()[1];
                self.advance();
            } else if !self.token().is_bracket(&BracketKind::RightParenthesis) {
                return Err(("Expected a )", self.token().span.clone()));
            } else {
                end = self.token().span.clone()[0];
            }
            let parameter = Parameter {
                name,
                label,
                span: [start, end],
            };
            parameters.push(parameter);
        }
        if self.end() {
            return Err(("Expected an )", self.token().span.clone()));
        };
        self.advance();
        Ok(parameters)
    }
    fn label(&'a self) -> NodeOrError<Type> {
        todo!()
    }
}
