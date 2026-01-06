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
    UnrecognisedCharacter,
    UnrecognisedStringEscape,
    UnexpectedEndOfLine,
    InvalidNumericLiteral(String),
}

impl Display for LexerErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use LexerErrorKind::*;
        match self {
            UnrecognisedCharacter => write!(f, "unexpected character in input"),
            UnrecognisedStringEscape => write!(f, "unrecognised escape sequence in string"),
            UnexpectedEndOfLine => write!(f, "unexpected line end"),
            InvalidNumericLiteral(cause) => write!(f, "failed to parse numeric literal: {}", cause),
        }
    }
}
