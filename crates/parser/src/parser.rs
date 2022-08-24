use std::cell::RefCell;

use crate::scanner::Scanner;
use ast::{
    precedence_of, BracketKind, Expression, Identifier, Keyword, LiteralKind, Operator,
    Punctuation, Statement, TextSpan, Token, TokenKind,
};
use utils::Stack;

type ParserError = (&'static str, TextSpan);
type NodeOrError<T> = Result<T, ParserError>;

/// The provider is a pseudo iterator that supplies tokens to the parser.
pub struct Provider {
    pub(crate) scanner: Scanner,
    pub(crate) index: usize,
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
    diagnostics: RefCell<Vec<ParserError>>,
    pub(crate) statements: RefCell<Vec<Statement<'a>>>,
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
            TokenKind::Keyword(keyword) => self.control_statement(keyword),
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
            TokenKind::Punctuation(Punctuation::Bracket(BracketKind::LeftParenthesis)) => {
                self.grouped_expression()
            }
            _ => todo!(),
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
                kind: TokenKind::Identifier(Identifier { value }),
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
}

impl<'a> Parser<'a> {
    fn control_statement(&'a self, keyword: &Keyword) -> NodeOrError<Statement<'a>> {
        match keyword {
            Keyword::If => self.if_statememt(),
            _ => todo!(),
        }
    }
    fn if_statememt(&'a self) -> NodeOrError<Statement<'a>> {
        let _start = self.token().span.clone();
        self.advance(); // Move past the if.
        todo!()
    }
}
