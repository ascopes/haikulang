use haikulang_lexer::err::LexerError;
use haikulang_lexer::location::Location;
use haikulang_lexer::token::Token;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Clone, Debug)]
pub struct ParserError {
    pub kind: ParserErrorKind,
    pub location: Location,
}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "error during parsing at {}: {}",
            self.location,
            self.kind.message()
        )
    }
}

impl Error for ParserError {}

#[derive(Clone, Debug)]
pub enum ParserErrorKind {
    LexerError(LexerError),
    SyntaxError(Token, String),
}

impl ParserErrorKind {
    pub fn message(&self) -> String {
        use ParserErrorKind::*;

        match self {
            LexerError(error) => format!("lexer error: {}", error),
            SyntaxError(token, message) => {
                format!("syntax error: {} (token was {:?})", message, token)
            }
        }
    }
}

impl Display for ParserErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}
