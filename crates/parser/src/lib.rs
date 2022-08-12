mod ast;
use crate::utils::stack::Stack;

use self::ast::Location;

use super::{
    error::CompileError,
    scanner::{
        helpers::precedence_of,
        token::{BracketKind, NumericKind, Token},
    },
};
use ast::{Block, Expression, MatchCase, NodeRange, Program, Statement};

type ParseResult = Result<Program, CompileError>;
type ParseInternalResult = Result<(), CompileError>;
type ExpressionOrError = Result<Expression, CompileError>;
type StatementOrError = Result<Statement, CompileError>;

pub fn parse(tokens: Vec<Token>) -> ParseResult {
    let mut parser = Parser::new();
    parser.parse(tokens)?;
    Ok(parser.result)
}
#[allow(dead_code)]
pub struct Parser {
    result: Program,
    token: Token,
    index: usize,
    store: NodeRange,
    tokens: Vec<Token>,
    end: bool,
    operator_stack: Stack<String>,
}

impl Parser {
    pub fn new() -> Self {
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
    pub fn parse(&mut self, tokens: Vec<Token>) -> Result<(), CompileError> {
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
        match &self.token {
            Token::Keyword { value, .. } => match value.as_str() {
                "if" => Ok(self.parse_if_statement()?),
                "while" => Ok(self.parse_while_statement()?),
                "do" => Ok(self.parse_do_while_statement()?),
                "println" => Ok(self.parse_println_statement()?),
                // "match" => Ok(self.parse_match_statement()?),
                _ => {
                    self.error("Unexpected keyword.")?;
                    unreachable!()
                }
            },
            Token::Bracket {
                kind: BracketKind::LCurly,
                ..
            } => Ok(self.parse_block()?),
            _ => Ok(self.parse_expression_statement()?),
        }
    }
    fn parse_condition(&mut self) -> ExpressionOrError {
        if !self.token.is_bracket(BracketKind::LParen) {
            self.error("Expected a ( here.")?;
        }
        self.next(); // Move past (

        let test = self.parse_expression()?;
        if !self.token.is_bracket(BracketKind::RParen) {
            self.error("Expected a ) here.")?;
        }
        self.next(); // Move past )
        Ok(test)
    }
    /// Parses an if statement.
    fn parse_if_statement(&mut self) -> StatementOrError {
        let start = self.token.get_location();
        self.next(); // Move past if.
        let test = self.parse_condition()?;
        let body = self.parse_statement()?;
        let alternate;
        if let Token::Keyword { value, .. } = &self.token {
            if value.as_str() == "else" {
                self.next(); // Move past else.
                alternate = Some(self.parse_statement()?);
            } else {
                alternate = None;
            }
        } else {
            alternate = None;
        };
        let end;
        if self.token.is_semi_colon() {
            end = self.token.get_location();
            self.next();
        } else if let Some(statement) = &alternate {
            end = statement.get_range();
        } else {
            end = test.get_range();
        }
        let ifstat = Statement::IfStatement {
            test,
            body: Box::new(body),
            alternate: Box::new(alternate),
            range: [start[0], start[1], end[2], end[3]],
        };
        Ok(ifstat)
    }
    fn parse_block(&mut self) -> Result<Statement, CompileError> {
        if !self.token.is_bracket(BracketKind::LCurly) {
            self.error("Expected a { here.")?;
        }
        let start = self.token.get_location();
        self.next(); // Move past {
        let mut statements = vec![];
        while !(self.end || self.token.is_bracket(BracketKind::RCurly)) {
            let statement = self.parse_statement()?;
            statements.push(statement);
        }
        if self.end {
            self.error("Expected a } here.")?
        }
        let end = self.token.get_location();
        self.next();
        let blockstat = Statement::BlockStatement {
            statements,
            range: [start[0], start[1], end[2], end[3]],
        };
        Ok(blockstat)
    }
    fn parse_while_statement(&mut self) -> StatementOrError {
        let start = self.token.get_location();
        self.next(); // Move past while.
        let test = self.parse_condition()?;
        let body = self.parse_statement()?;
        let mut end = body.get_range();
        if self.token.is_semi_colon() {
            end = self.token.get_location();
            self.next();
        }
        let whilestat = Statement::WhileStatement {
            test,
            body: Box::new(body),
            range: [start[0], start[1], end[2], end[3]],
        };
        Ok(whilestat)
    }
    fn parse_do_while_statement(&mut self) -> StatementOrError {
        let start = self.token.get_location();
        self.next(); // Move past do.
        let body = self.parse_statement()?;
        if let Token::Keyword { value, .. } = &self.token {
            if !(value.as_str() == "while") {
                self.error("Expected a while here.")?;
            }
            self.next(); // Move past while.
            let test = self.parse_condition()?;
            let mut end = test.get_range();
            if self.token.is_semi_colon() {
                end = self.token.get_location();
                self.next();
            }
            let dowhilestat = Statement::DoWhileStatement {
                test,
                body: Box::new(body),
                range: [start[0], start[1], end[2], end[3]],
            };
            Ok(dowhilestat)
        } else {
            self.error("Expected a while here.")?;
            unreachable!()
        }
    }
    fn parse_println_statement(&mut self) -> StatementOrError {
        let start = self.token.get_location();
        self.next(); // Move past println.
        let argument = self.parse_expression()?;
        if !self.token.is_semi_colon() {
            self.error("Expected a semicolon here.")?;
        }
        let end = self.token.get_location();
        self.next();
        let printstat = Statement::PrintLnStatement {
            argument,
            range: [start[0], start[1], end[2], end[3]],
        };
        Ok(printstat)
    }
    // fn parse_match_statement(&mut self) -> StatementOrError {
    //     let start = self.token.get_location();
    //     self.next(); // Move past match
    //     let discriminant = self.parse_condition()?;
    //     let mut cases = vec![];
    //     if !self.token.is_bracket(BracketKind::LCurly) {
    //         self.error("Expected a { here.")?;
    //     }
    //     self.next(); // Move past {
    //     while !(self.end || self.token.is_bracket(BracketKind::RCurly)) {
    //         let case = self.parse_match_case()?;
    //         cases.push(case);
    //     }
    //     if self.end {
    //         self.error("Expected a } here.")?;
    //     }
    //     let mut end = self.token.get_location();
    //     self.next();
    //     if self.token.is_semi_colon() {
    //         end = self.token.get_location();
    //         self.next();
    //     }
    //     let matchstat = Statement::MatchStatement {
    //         discriminant,
    //         cases,
    //         range: [start[0], start[1], end[2], end[3]],
    //     };
    //     Ok(matchstat)
    // }
    // fn parse_match_case(&mut self) -> Result<MatchCase, CompileError> {
    //     if !self.token.is_keyword("case") {
    //         self.error("Expected a match case here.")?;
    //     }
    //     self.next(); // Move past case.

    //     let test = self.parse_pattern()?;

    //     if !self.token.is_operator("=>") {

    //     }

    // }
    // fn parse_pattern(&mut self) -> Result<Pattern, CompileError> {

    // }
    /// Parses an expression statement.
    fn parse_expression_statement(&mut self) -> StatementOrError {
        let expression = self.parse_expression()?;
        if self.token.is_semi_colon() {
            let exprstat = Statement::expression_statement(expression, self.token.get_location());
            self.next();
            Ok(exprstat)
        } else {
            self.error("Expected a semi-colon")?;
            unreachable!()
        }
    }
    /// Parses an expression.
    fn parse_expression(&mut self) -> ExpressionOrError {
        match self.token.clone() {
            // a number token.
            Token::Number { .. } => {
                let number = self.parse_number()?;
                Ok(self.reparse(number)?)
            }
            Token::Operator { value, .. } => match value.as_str() {
                "!" | "++" | "--" | "+" | "-" => {
                    let unary = self.parse_unary_expression(value)?;
                    Ok(self.reparse(unary)?)
                }
                _ => {
                    self.error("Unexpected operator.")?;
                    unreachable!();
                }
            },
            // an identifier token.
            Token::Identifier { value, .. } => match value.as_str() {
                "new" => {
                    let newxp = self.parse_new_expression()?;
                    Ok(self.reparse(newxp)?)
                }
                _ => {
                    let exp = self.parse_identifier()?;
                    Ok(self.reparse(exp)?)
                }
            },
            Token::Literal { .. } => {
                let exp = self.parse_literal()?;
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
            // An open square bracket [
            Token::Bracket {
                kind: BracketKind::LSquare,
                ..
            } => {
                let exp = self.parse_array_expression()?;
                Ok(self.reparse(exp)?)
            }
            _ => Ok(Expression::Null),
        }
    }
    /// Parses a number token into a node.
    fn parse_number(&mut self) -> ExpressionOrError {
        let exp = Expression::number(self.token.clone());
        self.next();
        Ok(exp)
    }
    /// Parses a literal expression, e.g. true, false, self, etc.
    fn parse_literal(&mut self) -> ExpressionOrError {
        let exp = match &self.token {
            Token::Literal { value, loc } => match value.as_str() {
                "true" | "false" => Ok(Expression::boolean(value.clone(), *loc)),
                "self" => Ok(Expression::self_expression(*loc)),
                "nil" => Ok(Expression::nil_expression(*loc)),
                "core" => Ok(Expression::core_expression(*loc)),
                _ => unreachable!(),
            },
            _ => {
                self.error("Error. Expected a literal value.")?;
                unreachable!()
            }
        };
        self.next();
        exp
    }
    /// Parses an identifier token into a node.
    fn parse_identifier(&mut self) -> ExpressionOrError {
        let exp = Expression::identifier(self.token.clone());
        self.next();
        Ok(exp)
    }
    /// Parse a parenthesized group.
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
            Ok(node)
        } else {
            match &self.token {
                Token::Operator { value, .. } => match value.as_str() {
                    "||" | "&&" => Ok(self.parse_logical_expression(node, value.clone())?),
                    "." => Ok(self.parse_member_expression(node)?),
                    ".." => Ok(self.parse_range_expression(node)?),
                    "+=" | "/=" | "||=" | "&&=" | "*=" | "-=" | "%=" | "=" => {
                        Ok(self.parse_assignment_expression(node, value.clone())?)
                    }
                    "::" => Ok(self.parse_namespace_expression(node)?),
                    "?" => Ok(self.parse_ternary_expression(node)?),
                    "++" | "--" => Ok(self.parse_update_expression(node, value.clone())?),
                    "+" | "-" | "/" | "%" | "*" | "**" | ">" | "<" | "&" | "|" | ">>" | "<<"
                    | "==" | "!=" | "<=" | ">=" => {
                        Ok(self.parse_binary_expression(node, value.clone())?)
                    }
                    _ => Ok(node),
                },
                Token::Bracket {
                    kind: BracketKind::LParen,
                    ..
                } => Ok(self.parse_call_expression(node)?),
                Token::Bracket {
                    kind: BracketKind::LSquare,
                    ..
                } => Ok(self.parse_access_expression(node)?),
                _ => Ok(node),
            }
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
    /// Parses an update expression, e,g i++, y--
    fn parse_update_expression(
        &mut self,
        value: Expression,
        operator: String,
    ) -> ExpressionOrError {
        if self.is_lower_precedence(operator.as_str()) {
            Ok(value)
        } else {
            let updexp = Expression::update_expression(value, self.token.clone());
            self.next(); // Move past ++ or --
            Ok(self.reparse(updexp)?)
        }
    }
    /// Parses a unary expression. e.g !true, -4;
    fn parse_unary_expression(&mut self, operator: String) -> ExpressionOrError {
        let optoken = self.token.clone();
        self.next(); // Move past the operator.
        self.operator_stack.push(operator);
        let argument = self.parse_expression()?;
        self.operator_stack.pop();
        let expression = Expression::unary_expression(argument, optoken);
        Ok(self.reparse(expression)?)
    }
    /// Parses a new expression.
    fn parse_new_expression(&mut self) -> ExpressionOrError {
        let start = self.token.get_location();
        self.next(); // Move past new.
        self.operator_stack.push("new".to_string());
        let construct = self.parse_expression()?;
        let mut arguments = vec![];
        let mut end = [0, 0, 0, 0];
        self.operator_stack.pop();
        if self.token.is_bracket(BracketKind::LParen) {
            self.next(); // Move past parenthesis.
            self.operator_stack.push("new_call".to_string());
            while !(self.end || self.token.is_bracket(BracketKind::RParen)) {
                let argument = self.parse_expression()?;
                arguments.push(argument);
                if self.token.is_comma() {
                    self.next()
                } else {
                    self.error("A comma was expected here.")?;
                }
            }
            if self.end {
                self.error("A closing ) was expected.")?;
            }
            self.operator_stack.pop();
            end = self.token.get_location();
            self.next(); // Move past ).
        } else {
            self.error("New expressions must be constructed with arguments. Leave an empty parenthesis if no arguments are required.")?
        };
        let newexp = Expression::new_expression(start, construct, arguments, end);
        Ok(self.reparse(newexp)?)
    }
    /// Parses an array expression. e.g. [1, 2, 3]
    fn parse_array_expression(&mut self) -> ExpressionOrError {
        let start = self.token.get_location();
        let mut elements = vec![];
        self.operator_stack.push("arr".to_string()); // Mock operator to prevent precedence clashing.
        self.next(); // Move past [
        while !(self.end || self.token.is_bracket(BracketKind::RSquare)) {
            let element = self.parse_expression()?;
            elements.push(element);
            if self.token.is_comma() {
                self.next();
            } else if self.token.is_bracket(BracketKind::RSquare) {
                break;
            } else {
                self.error("Expected a comma here. ")?;
                unreachable!()
            }
        }
        if !self.token.is_bracket(BracketKind::RSquare) {
            self.error("Expected a closing bracket ] here. ")?;
            unreachable!();
        }
        let end = self.token.get_location();
        self.next();
        self.operator_stack.pop();
        let arrexp = Expression::array_expression(start, elements, end);
        Ok(self.reparse(arrexp)?)
    }
    /// Parses a range expression, e.g. 0..2, x..y
    fn parse_range_expression(&mut self, lower_boundary: Expression) -> ExpressionOrError {
        if self.is_lower_precedence("..") {
            Ok(lower_boundary)
        } else {
            self.next();
            self.operator_stack.push("..".to_string()); // Move past ..
            let upper_boundary = self.parse_expression()?;
            self.operator_stack.pop();
            let rangexp = Expression::range_expression(lower_boundary, upper_boundary);
            Ok(self.reparse(rangexp)?)
        }
    }
    /// Parses an assignment expression.
    fn parse_assignment_expression(
        &mut self,
        left_node: Expression,
        operator: String,
    ) -> ExpressionOrError {
        if self.is_lower_precedence("=") {
            Ok(left_node)
        } else {
            self.next(); // Move past the operator.
            self.operator_stack.push("=".to_string());
            let right_node = self.parse_expression()?;
            let assexp = Expression::assignment_expression(left_node, operator, right_node);
            Ok(self.reparse(assexp)?)
        }
    }
    /// Parses a call expression. foo(bar), greet()
    fn parse_call_expression(&mut self, callee: Expression) -> ExpressionOrError {
        if self.is_lower_precedence("(") {
            Ok(callee)
        } else {
            self.next(); // Move past parenthesis.
            let mut arguments = vec![];
            self.operator_stack.push("temp".to_string());
            while !(self.end || self.token.is_bracket(BracketKind::RParen)) {
                let argument = self.parse_expression()?;
                arguments.push(argument);
                if self.token.is_comma() {
                    self.next()
                }
            }
            self.operator_stack.pop();
            let end = self.token.get_location();
            if self.end {
                self.error("Expected a ')' here.")?;
            } else {
                self.next();
            }
            let callexp = Expression::call_expression(callee, arguments, end);
            Ok(self.reparse(callexp)?)
        }
    }
    /// Parses an access expression. e.g. foo[bar], names[2]
    fn parse_access_expression(&mut self, arr: Expression) -> ExpressionOrError {
        if self.is_lower_precedence("[") {
            return Ok(arr);
        }
        let mut end = [0, 0, 0, 0];
        self.next(); // Move past [.
        self.operator_stack.push("temp".to_string());
        let element = self.parse_expression()?;
        if !self.token.is_bracket(BracketKind::RSquare) {
            self.error("Expected a ']' here.")?;
        } else {
            end = self.token.get_location();
            self.next();
        }
        self.operator_stack.pop();
        let accexp = Expression::access_expression(arr, element, end);
        Ok(self.reparse(accexp)?)
    }
    /// Parses a logical expression, e.g isTall && isFair, 7 > 4 && 5 < 11 etc.
    fn parse_logical_expression(
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
            let logexp = Expression::logical_expression(left_node, operator, right_node);
            Ok(self.reparse(logexp)?)
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
    fn parse_namespace_expression(&mut self, left: Expression) -> ExpressionOrError {
        if self.is_lower_precedence("::") {
            Ok(left)
        } else {
            self.next(); // Move past "::"
            self.operator_stack.push("::".to_string());
            let right = self.parse_expression()?;
            self.operator_stack.pop();
            let namexp = Expression::namespace_expression(left, right);
            Ok(self.reparse(namexp)?)
        }
    }
    /// Parses a ternary expression. name === "adebola" ? "hello" : "who are you?";
    fn parse_ternary_expression(&mut self, test: Expression) -> ExpressionOrError {
        if self.is_lower_precedence("?") {
            Ok(test)
        } else {
            self.next(); // Move past ?
            self.operator_stack.push("?".to_string());
            let consequent = self.parse_expression()?;
            let alternate;
            self.operator_stack.pop();
            if self.token.is_colon() {
                self.next();
                self.operator_stack.push(":".to_string());
                alternate = self.parse_expression()?;
                self.operator_stack.pop();
                let ternexp = Expression::ternary_expression(test, consequent, alternate);
                Ok(self.reparse(ternexp)?)
            } else {
                self.error("Expected a colon here for the ternary alternate.")?;
                unreachable!()
            }
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
    #[test]
    fn it_parses_call_expressions() {}
}