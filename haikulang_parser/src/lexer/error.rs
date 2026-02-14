use crate::lexer::token::Token;
use logos::Lexer;
use std::fmt::{Display, Formatter, write};

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
