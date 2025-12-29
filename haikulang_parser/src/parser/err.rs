use crate::lexer::{LexerError, Token};
use crate::location::Location;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub struct ParserError {
    pub kind: ParserErrorKind,
    pub location: Location,
}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "error parsing document at {}: {}",
            self.kind, self.location
        )
    }
}

impl Error for ParserError {}

#[derive(Clone, Debug)]
pub enum ParserErrorKind {
    LexerError(LexerError),
    SyntaxError(Token, String),
}

impl Display for ParserErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use ParserErrorKind::*;

        match self {
            LexerError(error) => write!(f, "{}", error),
            SyntaxError(token, message) => write!(f, "Unexpected token {:?}: {}", token, message),
        }
    }
}
