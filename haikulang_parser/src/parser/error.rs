use crate::lexer::error::LexerError;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub enum ParserError {
    SyntaxError(String),
    LexerError(LexerError),
}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::SyntaxError(text) => write!(f, "syntax error in file: {}", text),
            ParserError::LexerError(err) => write!(f, "failed to tokenize file: {}", err),
        }
    }
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
        ParserError::LexerError(LexerError::InvalidStringLit("blah blah".to_string())),
        "failed to tokenize file: invalid string literal - blah blah"
        ; "LexerError"
    )]
    fn test_parser_error_formats_correctly(error: ParserError, expected: &str) {
        // Then
        assert_eq!(format!("{}", error), expected);
    }
}
