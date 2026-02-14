use crate::lexer::error::LexerError;
use crate::parser::parser::ParserResult;
use crate::span::{Span, Spanned};
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub enum ParserError {
    SyntaxError(String),
    LexerError(LexerError),
}

pub fn syntax_error<T: Clone>(span: Span, message: impl ToString) -> ParserResult<T> {
    let err = ParserError::SyntaxError(message.to_string());
    Err(Spanned::new(err, span))
}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::SyntaxError(text) => write!(f, "syntax error in file: {}", text),
            ParserError::LexerError(err) => write!(f, "failed to tokenize file: {}", err),
        }
    }
}
