use crate::location::Location;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Clone, Debug)]
pub struct LexerError {
    pub kind: LexerErrorKind,
    pub raw: String,
    pub start_location: Location,
    pub end_location: Location,
}

impl Display for LexerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "error during tokenization: {}", self.kind.message())?;

        if self.start_location != self.end_location {
            write!(f, ", at {} -> {}", self.start_location, self.end_location)?;
        } else {
            write!(f, ", at {}", self.start_location)?;
        }

        write!(f, " ({:?})", self.raw)?;
        Ok(())
    }
}

impl Error for LexerError {}

#[derive(Clone, Debug)]
pub enum LexerErrorKind {
    UnrecognisedCharacter,
    UnrecognisedStringEscape,
    UnexpectedEndOfLine,
    InvalidNumericLiteral(String),
}

impl LexerErrorKind {
    pub fn message(&self) -> String {
        use LexerErrorKind::*;

        match self {
            UnrecognisedCharacter => "unexpected character in input".to_string(),
            UnrecognisedStringEscape => "unrecognised escape sequence in string".to_string(),
            UnexpectedEndOfLine => "unexpected line end".to_string(),
            InvalidNumericLiteral(cause) => format!("failed to parse numeric literal: {}", cause),
        }
    }
}

impl Display for LexerErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}
