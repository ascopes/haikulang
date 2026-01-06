use crate::location::Location;
use std::fmt::{Debug, Display, Formatter};

#[derive(Clone, Debug)]
pub struct LexerError {
    pub kind: LexerErrorKind,
    pub raw: String,
    pub location: Location,
}

impl Display for LexerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} at {} ({:?})", self.kind, self.location, self.raw)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum LexerErrorKind {
    UnknownToken,
    UnknownStringEscape,
    PrematureEndOfLine,
    InvalidNumericLiteral(String),
}

impl Display for LexerErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use LexerErrorKind::*;
        match self {
            UnknownToken => write!(f, "unknown token in input"),
            UnknownStringEscape => write!(f, "unknown escape sequence in string"),
            PrematureEndOfLine => write!(f, "premature end of line"),
            InvalidNumericLiteral(cause) => write!(f, "failed to parse numeric literal: {}", cause),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn lexer_error_is_formatted_as_expected() {
        // Given
        let kind = LexerErrorKind::UnknownToken;
        let raw = "unga bunga".to_string();
        let location = Location {
            offset: 512,
            line: 32,
            column: 64,
        };
        let err = LexerError {
            kind: kind.clone(),
            raw: raw.clone(),
            location,
        };

        // When
        let actual = format!("{}", err);

        // Then
        let expected = format!("{} at {} (\"unga bunga\")", kind, location);
        assert_eq!(actual, expected);
    }

    #[test_case(LexerErrorKind::UnknownToken, "unknown token in input")]
    #[test_case(
        LexerErrorKind::UnknownStringEscape,
        "unknown escape sequence in string"
    )]
    #[test_case(LexerErrorKind::PrematureEndOfLine, "premature end of line")]
    #[test_case(LexerErrorKind::InvalidNumericLiteral("wahh!".to_string()), "failed to parse numeric literal: wahh!")]
    fn lexer_error_kind_is_formatted_as_expected(kind: LexerErrorKind, expected: &str) {
        // When
        let actual = format!("{}", kind);

        // Then
        assert_eq!(actual, expected);
    }
}
