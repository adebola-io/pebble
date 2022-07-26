mod ast;
use crate::utils::stack::Stack;

use super::{
    error::CompileError,
    scanner::{
        helpers::precedenceOf,
        token::{NumericKind, Token},
    },
};
use ast::{Block, Expression, Literal, NodeRange, Program, Statement};

type ParseResult = Result<Program, CompileError>;
type ParseInternalResult = Result<(), CompileError>;
type ExpressionOrError = Result<Expression, CompileError>;
type StatementOrError = Result<Statement, CompileError>;

pub fn parse(tokens: Vec<Token>) -> ParseResult {
    let mut parser = Parser::new();
    parser.parse(tokens)?;
    Ok(parser.result)
}

struct Parser {
    result: Program,
    token: Token,
    index: usize,
    store: NodeRange,
    tokens: Vec<Token>,
    end: bool,
    operator_stack: Stack<String>,
}

impl Parser {
    fn new() -> Self {
        Parser {
            result: Program::new(),
            operator_stack: Stack::new(),
            tokens: vec![],
            index: 0,
            end: false,
            store: [0, 0, 0, 0],
            token: Token::SOF,
        }
    }
    /// Advance to the next token.
    fn next(&mut self) {
        if self.token != Token::EOF {
            self.index += 1;
            self.token = self.tokens[self.index].clone()
        } else {
            self.end = true;
        }
    }
    fn error(&mut self, message: &str) -> ParseInternalResult {
        let error = CompileError {
            message: message.to_string(),
            line: self.token.get_end_line(),
            column: self.token.get_end_column(),
        };
        Err(error)
    }
    fn is_lower_precedence(&self, operator: &str) -> bool {
        if self.operator_stack.is_empty {
            false
        } else if precedenceOf(operator) <= precedenceOf(self.operator_stack.top().unwrap()) {
            true
        } else {
            false
        }
    }
    fn start_range(&mut self) {
        let loc = self.token.get_location();
        self.store[0] = loc[0];
        self.store[1] = loc[1];
    }
    fn stop_range(&mut self) {
        let loc = self.token.get_location();
        self.store[2] = loc[2];
        self.store[3] = loc[3];
    }
    /// Parses a stream of tokens into a valid Program.
    fn parse(&mut self, tokens: Vec<Token>) -> Result<(), CompileError> {
        self.tokens = tokens;
        self.next();
        while self.token != Token::EOF {
            let statement = self.parse_statement()?;
            self.result.append(statement);
        }
        Ok(())
    }
    /// Parses a statement.
    fn parse_statement(&mut self) -> StatementOrError {
        if let Token::Keyword { .. } = &self.token {
            Ok(self.parse_other_statement()?)
        } else {
            Ok(self.parse_expression_statement()?)
        }
    }
    fn parse_expression_statement(&mut self) -> StatementOrError {
        let expression = self.parse_expression()?;
        if self.token.is_semi_colon() {
            let exprstat = Statement::expression_statement(expression, self.token.get_location());
            self.next();
            Ok(exprstat)
        } else {
            println!("{:?}", self.token);
            self.error("Expected a semi-colon")?;
            panic!()
        }
    }
    fn parse_other_statement(&mut self) -> StatementOrError {
        Ok(Statement::EmptyStatement { range: self.store })
    }
    /// Parses an expression.
    fn parse_expression(&mut self) -> ExpressionOrError {
        match &self.token {
            Token::Number { .. } => {
                let number = self.parse_number()?;
                let exp = self.reparse(number)?;
                Ok(exp)
            }
            _ => Ok(Expression::Null),
        }
    }
    fn parse_number(&mut self) -> ExpressionOrError {
        let exp;
        if self.token.is_number() {
            exp = Expression::number(self.token.clone());
        } else {
            self.error("Expected a number.")?;
            panic!();
        }
        self.next();
        Ok(exp)
    }
    fn reparse(&mut self, node: Expression) -> ExpressionOrError {
        if self.token.is_semi_colon() {
            return Ok(node);
        }
        if let Token::Operator { value, .. } = self.token.clone() {
            match value.as_str() {
                "+" | "-" | "/" | "%" | "*" | "**" | ">" | "<" | "&" | "|" | ">>" | "<<" | "<="
                | ">=" => Ok(self.binary_expression(node, value)?),
                _ => Ok(node),
            }
        } else {
            Ok(node)
        }
    }
    /// Parses a binary expression, e.g 2 + 2, 3 * 6, etc.
    fn binary_expression(&mut self, left_node: Expression, operator: String) -> ExpressionOrError {
        if self.is_lower_precedence(&operator) {
            Ok(left_node)
        } else {
            self.next(); // Move past operator.
            self.operator_stack.push(operator.clone());
            let right_node = self.parse_expression()?;
            self.operator_stack.pop();
            let binexp = Expression::binary_expression(left_node, operator, right_node);
            Ok(self.reparse(binexp)?)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::scanner;
    use super::*;
    #[test]
    fn it_parses_binary_expressions() {
        let text = "2 + 2 ;";
        let tokens = scanner::scan(text.to_string()).unwrap();
        let tree = parse(tokens).unwrap();
        assert_eq!(
            tree,
            Program {
                body: Block {
                    statements: vec![Statement::ExpressionStatement {
                        expression: Expression::BinaryExpression {
                            operator: String::from("+"),
                            left: Box::new(Expression::Number {
                                kind: NumericKind::Decimal,
                                range: [1, 1, 1, 2],
                                raw: String::from("2"),
                                value: String::from("2")
                            }),
                            right: Box::new(Expression::Number {
                                kind: NumericKind::Decimal,
                                range: [1, 5, 1, 6],
                                raw: String::from("2"),
                                value: String::from("2")
                            }),
                            range: [1, 1, 1, 6]
                        },
                        range: [1, 1, 1, 7]
                    }],
                    range: [1, 1, 1, 7],
                },
                range: [1, 1, 1, 7],
            }
        )
    }
}
