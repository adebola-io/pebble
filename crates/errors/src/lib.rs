use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Error {
    ParserError,
    RuntimeError,
}

pub enum LexicalError {
    /// A String does not have a closing quote mark.
    UnterminatedStringLiteral,
    /// A character token has more than one character in its body.
    InvalidCharacterCount,
}

#[derive(Debug, PartialEq)]
pub enum SyntaxError {
    NamedFunctionExpr,
    UninitializedTypeAlias,
    UninitializedConstant,
    UninitializedUntypedVariable,
    ExpectedImportSource,
    ExpectedImport,
    UnclosedImportSpace,
    StrayImplement,
    IllegalElse,
    IllegalRecover,
    ExpectedFunctionName,
    ExpectedParamterName,
    ExpectedVariableName,
    ExpectedPropertyName,
    ExpectedInterfaceName,
    ExpectedTypeName,
    ExpectedGenericTypeParameter,
    ExpectedReturnType,
    ExpectedAModuleName,
    ExpectedIdentifier,
    ExpectedAs,
    ExpectedFrom,
    ExpectedIn,
    ExpectedSemiColon,
    ExpectedColon,
    ExpectedFunctionArgument,
    ExpectedLParen,
    ExpectedRParen,
    ExpectedLCurly,
    ExpectedRCurly,
    ExpectedLSquareBrac,
    ExpectedRSquareBrac,
    ExpectedLAngleBrac,
    ExpectedRAngleBrac,
    ExpectedCommaOrRSquareBrac,
    ExpectedCommaOrRCurly,
    ExpectedCommaOrRAngleBrac,
    ExpectedExpression,
    UnrecognizedInjunction,
    UnexpectedOperator,
    UnexpectedKeyword,
    IllegalDeclaration,
}

impl Display for SyntaxError {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

pub enum TypeError {
    UnsupportedAddOperation,
    UnsupportedSubtractOperation,
    UnsupportedMultiplyOperation,
    InconsistentAssignment,
    InconsistentInitializer,
    UnusedVariable,
    AssigningToNil,
}

impl Error {
    pub fn scanner_error(error_code: i32) -> &'static str {
        match error_code {
            1 => "Unterminated String Literal.",
            _ => unreachable!(),
        }
    }
}
