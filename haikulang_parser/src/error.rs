use crate::span::Spanned;
use std::fmt::{Display, Formatter};

pub type ParserResult<T> = Result<Spanned<T>, Spanned<ParserError>>;

#[derive(Clone, Debug, Default, PartialEq)]
pub enum ParserError {
    // Parser issues.
    SyntaxError(String),

    // Lexer issues.
    InvalidStringLit(String),
    InvalidIntLit(String),
    InvalidFloatLit(String),
    UnclosedStringLit(String),
    UnknownToken(String),

    // Emitted by Logos if we hit an unexpected issue.
    #[default]
    UnknownError,
}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SyntaxError(text) => write!(f, "syntax error in file: {}", text),
            Self::InvalidStringLit(text) => write!(f, "invalid string literal: {}", text),
            Self::InvalidIntLit(text) => write!(f, "invalid int literal: {}", text),
            Self::InvalidFloatLit(text) => write!(f, "invalid float literal: {}", text),
            Self::UnclosedStringLit(text) => write!(f, "unclosed string literal: {}", text),
            Self::UnknownToken(value) => write!(f, "unknown token in input: {}", value),
            Self::UnknownError => write!(f, "unknown error"),
        }
    }
}

pub trait ErrorReporter {
    fn report(&mut self, error: &Spanned<ParserError>);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(
        ParserError::SyntaxError("blam".to_string()),
        "syntax error in file: blam"
        ; "SyntaxError"
    )]
    #[test_case(
        ParserError::InvalidStringLit("bad words used".to_string()),
        "invalid string literal: bad words used"
        ; "InvalidStringLit"
    )]
    #[test_case(
        ParserError::InvalidIntLit("0f1d is not a valid int".to_string()),
        "invalid int literal: 0f1d is not a valid int"
        ; "InvalidIntLit"
    )]
    #[test_case(
        ParserError::InvalidFloatLit("0.1f1d is not a valid float".to_string()),
        "invalid float literal: 0.1f1d is not a valid float"
        ; "InvalidFloatLit"
    )]
    #[test_case(
        ParserError::UnclosedStringLit("the string was not closed".to_string()),
        "unclosed string literal: the string was not closed"
        ; "UnclosedStringLit"
    )]
    #[test_case(
        ParserError::UnknownToken("foobar".to_string()),
        "unknown token in input: foobar"
        ; "UnknownToken"
    )]
    #[test_case(
        ParserError::UnknownError,
        "unknown error"
        ; "UnknownError"
    )]
    fn test_parser_error_formats_correctly(error: ParserError, expected: &str) {
        // Then
        assert_eq!(format!("{}", error), expected);
    }
}
