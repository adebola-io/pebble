mod ast;
use crate::utils::stack::Stack;

use super::{
    error::CompileError,
    scanner::{
        helpers::precedence_of,
        token::{BracketKind, NumericKind, Token},
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
    /// Log an error that has been encountered during parsing.
    fn error(&mut self, message: &str) -> ParseInternalResult {
        let error = CompileError {
            message: message.to_string(),
            line: self.token.get_end_line(),
            column: self.token.get_end_column(),
        };
        Err(error)
    }
    /// Check if an operator has a lower precedence in regards to the previously parsed operator.
    /// This function determines associativity and operator precedence.
    fn is_lower_precedence(&self, operator: &str) -> bool {
        if self.operator_stack.is_empty {
            false
        } else if precedence_of(operator) <= precedence_of(self.operator_stack.top().unwrap()) {
            true
        } else {
            false
        }
    }
    /// Mark the start of a node.
    fn _start_range(&mut self) {
        let loc = self.token.get_location();
        self.store[0] = loc[0];
        self.store[1] = loc[1];
    }
    /// Mark the end of a node.
    fn _stop_range(&mut self) {
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
            // a number token.
            Token::Number { .. } => {
                let number = self.parse_number()?;
                Ok(self.reparse(number)?)
            }
            // an identifier token.
            Token::Identifier { .. } => {
                let exp = self.parse_identifier()?;
                Ok(self.reparse(exp)?)
            }
            // an open bracket token (
            Token::Bracket {
                kind: BracketKind::LParen,
                ..
            } => {
                let exp = self.parse_group()?;
                Ok(self.reparse(exp)?)
            }
            _ => Ok(Expression::Null),
        }
    }
    /// Parses a number token into a node.
    fn parse_number(&mut self) -> ExpressionOrError {
        let exp;
        exp = Expression::number(self.token.clone());
        self.next();
        Ok(exp)
    }
    /// Parses an identifier token into a node.
    fn parse_identifier(&mut self) -> ExpressionOrError {
        let exp = Expression::identifier(self.token.clone());
        self.next();
        Ok(exp)
    }
    // Parse a paremthesized group.
    fn parse_group(&mut self) -> ExpressionOrError {
        self.next(); // Move past the left parenthesis.
        self.operator_stack.push("temp".to_string()); // A mock operator, which prevents the parenthesized group from affecting outer operators.
        let expression = self.parse_expression()?;
        self.operator_stack.pop();
        if self.token.is_bracket(BracketKind::RParen) {
            self.next();
        } else {
            self.error("Expected ')' here.")?;
        }
        Ok(expression)
    }
    fn reparse(&mut self, node: Expression) -> ExpressionOrError {
        if self.token.is_semi_colon() {
            return Ok(node);
        }
        if let Token::Operator { value, .. } = self.token.clone() {
            match value.as_str() {
                "." => Ok(self.parse_member_expression(node)?),
                "+" | "-" | "/" | "%" | "*" | "**" | ">" | "<" | "&" | "|" | ">>" | "<<" | "<="
                | ">=" => Ok(self.parse_binary_expression(node, value)?),
                _ => Ok(node),
            }
        } else {
            Ok(node)
        }
    }
    // Parses a member expression, e.g. core.format, person.age.
    fn parse_member_expression(&mut self, object: Expression) -> ExpressionOrError {
        if self.is_lower_precedence(".") {
            Ok(object)
        } else {
            self.next();
            self.operator_stack.push(".".to_string());
            if !self.token.is_identifier() {
                self.error("Expected object property name.")?;
            }
            let property = self.parse_identifier()?;
            self.operator_stack.pop();
            let memexp = Expression::member_expression(object, property);
            Ok(self.reparse(memexp)?)
        }
    }
    /// Parses a binary expression, e.g 2 + 2, 3 * 6, etc.
    fn parse_binary_expression(
        &mut self,
        left_node: Expression,
        operator: String,
    ) -> ExpressionOrError {
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
    #[test]
    fn it_changes_operator_precendence_based_on_brackets() {
        let text = "(2+2)*8;";
        let tokens = scanner::scan(text.to_string()).unwrap();
        let tree = parse(tokens).unwrap();
        assert_eq!(
            tree.body.statements[0],
            Statement::ExpressionStatement {
                expression: Expression::BinaryExpression {
                    operator: String::from("*"),
                    left: Box::new(Expression::BinaryExpression {
                        operator: String::from("+"),
                        left: Box::new(Expression::Number {
                            kind: NumericKind::Decimal,
                            range: [1, 2, 1, 3],
                            raw: String::from("2"),
                            value: String::from("2")
                        }),
                        right: Box::new(Expression::Number {
                            kind: NumericKind::Decimal,
                            range: [1, 4, 1, 5],
                            raw: String::from("2"),
                            value: String::from("2")
                        }),
                        range: [1, 2, 1, 5]
                    }),
                    right: Box::new(Expression::Number {
                        kind: NumericKind::Decimal,
                        value: String::from("8"),
                        raw: String::from("8"),
                        range: [1, 7, 1, 8]
                    }),
                    range: [1, 2, 1, 8]
                },
                range: [1, 2, 1, 8]
            }
        )
    }
    #[test]
    fn it_parses_member_expressions() {
        let text = "person.name;";
        let tokens = scanner::scan(text.to_string()).unwrap();
        let tree = parse(tokens).unwrap();
        assert_eq!(
            tree.body.statements[0],
            Statement::ExpressionStatement {
                expression: Expression::MemberExpression {
                    object: Box::new(Expression::Identifier {
                        name: String::from("person"),
                        range: [1, 1, 1, 7]
                    }),
                    property: Box::new(Expression::Identifier {
                        name: String::from("name"),
                        range: [1, 8, 1, 12]
                    }),
                    range: [1, 1, 1, 12]
                },
                range: [1, 1, 1, 12]
            }
        )
    }
}
