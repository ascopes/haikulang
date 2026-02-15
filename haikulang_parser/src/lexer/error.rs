use std::fmt::{Display, Formatter};

#[derive(Clone, Default, Debug, PartialEq)]
pub enum LexerError {
    InvalidStringLit(String),
    InvalidIntLit(String),
    InvalidFloatLit(String),
    UnclosedStringLit(String),

    #[default]
    UnexpectedError,
}

impl Display for LexerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LexerError::InvalidStringLit(text) => write!(f, "invalid string literal - {}", text),
            LexerError::InvalidIntLit(text) => write!(f, "invalid int literal - {}", text),
            LexerError::InvalidFloatLit(text) => write!(f, "invalid float literal - {}", text),
            LexerError::UnclosedStringLit(text) => write!(f, "unclosed string literal - {}", text),
            LexerError::UnexpectedError => {
                write!(f, "an unknown error occurred tokenizing the input")
            }
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
        LexerError::UnexpectedError,
        "an unknown error occurred tokenizing the input"
        ; "UnexpectedError"
    )]
    fn test_lexer_error_formats_correctly(error: LexerError, expected: &str) {
        // Then
        assert_eq!(format!("{}", error), expected);
    }
}
