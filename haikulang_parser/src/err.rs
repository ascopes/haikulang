use haikulang_lexer::err::LexerError;
use haikulang_lexer::location::Location;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub struct ParserError {
    pub kind: ParserErrorKind,
    pub raw: String,
    pub start_location: Location,
    pub end_location: Location,
}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "error during parsing: {}", self.kind.message())?;

        if self.start_location != self.end_location {
            write!(f, ", at {} -> {}", self.start_location, self.end_location)?;
        } else {
            write!(f, ", at {}", self.start_location)?;
        }

        write!(f, " ({:?})", self.raw)?;
        Ok(())
    }
}

impl Error for ParserError {}

#[derive(Debug)]
pub enum ParserErrorKind {
    LexerError(LexerError),
}

impl ParserErrorKind {
    pub fn message(&self) -> String {
        use ParserErrorKind::*;

        match self {
            LexerError(error) => error.to_string(),
        }
    }
}

impl Display for ParserErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}
