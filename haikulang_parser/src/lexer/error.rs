use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Default, PartialEq)]
pub enum LexerError {
    InvalidStringLit(String),
    InvalidIntLit(String),
    InvalidFloatLit(String),
    UnclosedStringLit(String),
    UnknownToken(String),

    #[default]
    UnknownError,
}

impl Display for LexerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LexerError::InvalidStringLit(text) => write!(f, "invalid string literal - {}", text),
            LexerError::InvalidIntLit(text) => write!(f, "invalid int literal - {}", text),
            LexerError::InvalidFloatLit(text) => write!(f, "invalid float literal - {}", text),
            LexerError::UnclosedStringLit(text) => write!(f, "unclosed string literal - {}", text),
            LexerError::UnknownToken(value) => write!(f, "unknown token in input - {}", value),
            LexerError::UnknownError => write!(f, "unknown error"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(
        LexerError::InvalidStringLit("bad words used".to_string()),
        "invalid string literal - bad words used"
        ; "InvalidStringLit"
    )]
    #[test_case(
        LexerError::InvalidIntLit("0f1d is not a valid int".to_string()),
        "invalid int literal - 0f1d is not a valid int"
        ; "InvalidIntLit"
    )]
    #[test_case(
        LexerError::InvalidFloatLit("0.1f1d is not a valid float".to_string()),
        "invalid float literal - 0.1f1d is not a valid float"
        ; "InvalidFloatLit"
    )]
    #[test_case(
        LexerError::UnclosedStringLit("the string was not closed".to_string()),
        "unclosed string literal - the string was not closed"
        ; "UnclosedStringLit"
    )]
    #[test_case(
        LexerError::UnknownToken("foobar".to_string()),
        "unknown token in input - foobar"
        ; "UnknownToken"
    )]
    #[test_case(
        LexerError::UnknownError,
        "unknown error"
        ; "UnknownError"
    )]
    fn test_lexer_error_formats_correctly(error: LexerError, expected: &str) {
        // Then
        assert_eq!(format!("{}", error), expected);
    }
}
