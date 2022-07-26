mod ast;
use self::ast::Expression;
use super::{error::CompileError, scanner::token::Token};
use ast::Program;

type ParseResult = Result<Program, CompileError>;
type ParseInternalResult = Result<(), CompileError>;

pub fn parse(tokens: Vec<Token>) -> ParseResult {
    let mut parser = Parser::new(tokens);
    parser.parse()?;
    Ok(parser.result)
}

struct Parser {
    result: Program,
    token: Token,
    index: usize,
    tokens: Vec<Token>,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser {
            result: Program::new(),
            tokens,
            index: 0,
            token: Token::SOF,
        }
    }
    /// Advance to the next token.
    fn next(&mut self) {
        if self.token != Token::EOF {
            self.index += 1;
            self.token = self.tokens[self.index].clone()
        }
    }
    /// Parses a stream of tokens into a valid Program.
    fn parse(&mut self) -> Result<(), CompileError> {
        self.next();
        while self.token != Token::EOF {
            self.parse_statement()?
        }
        Ok(())
    }
    /// Parses a statement.
    fn parse_statement(&mut self) -> ParseInternalResult {
        match &self.token {
            _ => {}
        }
        Ok(())
    }
    fn parse_expression_statement(&mut self) -> ParseInternalResult {
        Ok(())
    }
    fn parse_other_statement(&mut self) -> ParseInternalResult {
        Ok(())
    }
    /// Parses an expression.
    fn parse_expression(&mut self) -> Result<(), CompileError> {
        Ok(())
    }
}
