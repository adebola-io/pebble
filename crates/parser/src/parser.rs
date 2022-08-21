use std::cell::{RefCell, RefMut};

use crate::scanner::Scanner;
use ast::{
    precedence_of, Expression, LiteralKind, Operator, Statement, TextSpan, Token, TokenKind,
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
        match self.token().kind {
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
                    Ok(Statement::create_expression_statement(exp))
                } else {
                    // Every expression statement must end with a semi-colon.
                    Err(("Expected a semi-colon.", self.token().span.clone()))
                }
            }
        }
    }
    fn expression(&'a self) -> NodeOrError<Expression<'a>> {
        match self.token().kind {
            TokenKind::Literal(_) => self.literal(),
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
}

impl<'a> Parser<'a> {
    fn reparse(&'a self, node: Expression<'a>) -> NodeOrError<Expression<'a>> {
        if self.token().is_semi_colon() {
            Ok(node)
        } else if let Token {
            kind: TokenKind::Operator(operator),
            ..
        } = self.token()
        {
            match operator {
                Operator::Add | Operator::Multiply => todo!(),
                _ => todo!(),
            }
        } else {
            Err(("Expected an operator.", self.token().span.clone()))
        }
    }
}

// impl Parser {
//     pub fn from_scanner(mut scanner: Scanner) -> Self {
//         scanner.run();
//         let mut parser = Parser {
//             token: scanner.tokens[0].clone(),
//             scanner,
//             statements: vec![],
//             index: 0,
//             op_stack: Stack::new(),
//         };
//         while !parser.token.is_eof() {
//             let parsed = parser.statement();
//             match parsed {
//                 Ok(statement) => parser.statements.push(statement),
//                 Err(_) => todo!(),
//             }
//         }
//         parser
//     }
//     fn statement(&mut self) -> NodeOrError<Statement> {
//         match self.token.kind {
//             TokenKind::Keyword(_) => todo!(),
//             TokenKind::Identifier(_) | TokenKind::Literal(_) => self.expression_statement(),
//             _ => todo!(),
//         }
//     }
//     fn expression_statement(&mut self) -> NodeOrError<Statement> {
//         let start = self.token.span;
//         println!("Building expression statement...");
//         let expression = self.expression()?;
//         if self.token.is_semi_colon() {
//             self.next();
//             let end = self.token.span;
//             Ok(Node::expression_statement([start[0], end[1]], expression))
//         } else {
//             Err(("Expected a semicolon here.", self.token.span[1]))
//         }
//     }
//     /// Parses an expression.
//     fn expression(&mut self) -> NodeOrError<Expression> {
//         println!("Parsing, expresion...");
//         match self.token.kind {
//             TokenKind::Literal(_) => {
//                 let literal = self.literal();
//                 self.reparse(literal)
//             }
//             _ => todo!(),
//         }
//     }
//     /// Recursively parse an expression.
//     fn reparse(&mut self, n: NodeOrError) -> NodeOrError<Expression> {
//         match n {
//             Ok(node) => {
//                 if self.token.is_semi_colon() {
//                     return Ok(node);
//                 }
//                 match &self.token {
//                     Token {
//                         kind:
//                             TokenKind::Operator(
//                                 Operator::Add
//                                 | Operator::Subtract
//                                 | Operator::Multiply
//                                 | Operator::Divide
//                                 | Operator::LessThan,
//                             ),
//                         ..
//                     } => self.binary_expression(node),
//                     _ => Ok(node),
//                 }
//             }
//             Err(e) => Err(e),
//         }
//     }
//     /// Parses a literal.
//     fn literal(&mut self) -> NodeOrError<Expression> {
//         {
//             println!("Parsing literal...");
//             if let Token {
//                 span,
//                 kind: TokenKind::Literal(literal),
//             } = self.token.clone()
//             {
//                 let node = match literal {
//                     Literal {
//                         kind: LiteralKind::BooleanLiteral,
//                         value,
//                     } => Node::boolean_expression(span, if value == "true" { true } else { false }),
//                     Literal {
//                         kind: LiteralKind::StringLiteral,
//                         value,
//                     } => Node::string_expression(span, value.to_string()),
//                     Literal {
//                         kind: LiteralKind::NumericLiteral,
//                         value,
//                     } => Node::number_expression(span, value.to_string()),
//                     _ => unreachable!(),
//                 };
//                 self.next();
//                 println!("Finished parsing literal. Current node is {:?}", self.token);
//                 Ok(node)
//             } else {
//                 unreachable!()
//             }
//         }
//         /// Parses a binary expression.
//         fn binary_expression(&mut self, left: Node) -> NodeOrError<Expression> {
//             if let Token {
//                 kind: TokenKind::Operator(op),
//                 ..
//             } = self.token.clone()
//             {
//                 println!("Binary expression... The current token is {:?}", self.token);
//                 if self.is_lower_precedence(&op) {
//                     println!("yes.");
//                     Ok(left)
//                 } else {
//                     self.op_stack.push(op.clone());
//                     let operator = op.clone();
//                     self.next(); // Move past operator.
//                     println!("{:?}", self.token);
//                     let right = self.expression()?;
//                     self.op_stack.pop();
//                     let binexp = Node::binary_expression(left, right, operator);
//                     Ok(self.reparse(Ok(binexp))?)
//                 }
//             } else {
//                 unreachable!()
//             }
//         }
//     }
// }
