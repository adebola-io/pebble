use crate::scanner::Scanner;
use ast::{precedence_of, Literal, LiteralKind, Node, Operator, Token, TokenKind};
use utils::Stack;

type NodeOrError = Result<Node, (&'static str, [u64; 2])>;
/// The recursive descent parser that goes over the array of tokens scanned from the source text, and iteratively builds out the AST syntax tree.
pub struct Parser {
    pub statements: Vec<Node>,
    scanner: Scanner,
    token: Token,
    index: usize,
    op_stack: Stack<&'static str>,
}

impl Parser {
    pub fn next(&mut self) {
        self.index += 1;
        if self.index < self.scanner.tokens.len() {
            self.token = self.scanner.tokens[self.index].clone()
        }
    }
    fn is_lower_precedence(&self, operator: &'static str) -> bool {
        if self.op_stack.is_empty {
            false
        } else if precedence_of(operator) <= precedence_of(self.op_stack.top().unwrap()) {
            true
        } else {
            false
        }
    }
}

impl Parser {
    pub fn from_scanner(mut scanner: Scanner) -> Self {
        scanner.run();
        let mut parser = Parser {
            token: scanner.tokens[0].clone(),
            scanner,
            statements: vec![],
            index: 0,
            op_stack: Stack::new(),
        };
        while !parser.token.is_eof() {
            let parsed = parser.statement();
            match parsed {
                Ok(statement) => parser.statements.push(statement),
                Err(_) => todo!(),
            }
        }
        parser
    }
    fn statement(&mut self) -> NodeOrError {
        match self.token.kind {
            TokenKind::Keyword(_) => todo!(),
            TokenKind::Identifier(_) | TokenKind::Literal(_) => self.expression_statement(),
            _ => todo!(),
        }
    }
    fn expression_statement(&mut self) -> NodeOrError {
        let start = self.token.span;
        let expression = self.expression()?;
        if self.token.is_semi_colon() {
            self.next();
            let end = self.token.span;
            Ok(Node::expression_statement([start[0], end[1]], expression))
        } else {
            Err(("Expected a semicolon here.", self.token.span[1]))
        }
    }
    /// Parses an expression.
    fn expression(&mut self) -> NodeOrError {
        match self.token.kind {
            TokenKind::Literal(_) => self.reparse(self.literal()),
            _ => todo!(),
        }
    }
    /// Recursively parse an expression.
    fn reparse(&mut self, n: NodeOrError) -> NodeOrError {
        match n {
            Ok(node) => {
                self.next();
                match &self.token {
                    Token {
                        kind:
                            TokenKind::Operator(
                                Operator::Add
                                | Operator::Subtract
                                | Operator::Multiply
                                | Operator::Divide,
                            ),
                        ..
                    } => self.binary_expression(node),
                    _ => Ok(node),
                }
            }
            Err(e) => Err(e),
        }
    }
    /// Parses a literal.
    fn literal(&self) -> NodeOrError {
        if let Token {
            span,
            kind: TokenKind::Literal(literal),
        } = &self.token
        {
            let node = match literal {
                Literal {
                    kind: LiteralKind::BooleanLiteral,
                    value,
                } => Node::boolean_expression(*span, if value == "true" { true } else { false }),
                Literal {
                    kind: LiteralKind::StringLiteral,
                    value,
                } => Node::string_expression(*span, value.to_string()),
                Literal {
                    kind: LiteralKind::NumericLiteral,
                    value,
                } => Node::number_expression(*span, value.to_string()),
                _ => unreachable!(),
            };
            Ok(node)
        } else {
            unreachable!()
        }
    }
    /// Parses a binary expression.
    fn binary_expression(&mut self, left: Node) -> NodeOrError {
        todo!()
    }
}
