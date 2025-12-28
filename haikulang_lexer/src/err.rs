use crate::location::Location;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub struct LexerError {
    pub kind: LexerErrorKind,
    pub raw: String,
    pub start_location: Location,
    pub end_location: Location,
    pub cause: Option<Box<dyn Debug>>,
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

        if let Some(cause) = &self.cause {
            write!(f, " caused by {:?}", cause)?;
        }

        Ok(())
    }
}

impl Error for LexerError {}

#[derive(Debug, PartialEq)]
pub enum LexerErrorKind {
    UnrecognisedCharacter,
    UnrecognisedStringEscape,
    IncompleteIntLiteral,
    IncompleteFloatLiteral,
    UnexpectedEndOfLine,
    OtherInvalidNumericLiteral,
}

impl LexerErrorKind {
    pub fn message(&self) -> &str {
        match *self {
            LexerErrorKind::UnrecognisedCharacter => "unexpected character in input",
            LexerErrorKind::UnrecognisedStringEscape => "unrecognised escape sequence in string",
            LexerErrorKind::IncompleteIntLiteral => "incomplete integer literal",
            LexerErrorKind::IncompleteFloatLiteral => "incomplete float literal",
            LexerErrorKind::UnexpectedEndOfLine => "unexpected line end",
            LexerErrorKind::OtherInvalidNumericLiteral => "failed to parse numeric literal",
        }
    }
}

impl Display for LexerErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}
