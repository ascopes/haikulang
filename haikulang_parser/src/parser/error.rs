use crate::lexer::error::LexerError;
use crate::parser::parser::ParserResult;
use crate::span::{Span, Spanned};

#[derive(Clone, Debug)]
pub enum ParserError {
    UnknownSequence(String),
    SyntaxError(String),
    LexerError(LexerError),
}

pub fn syntax_error(span: Span, message: impl ToString) -> ParserResult {
    let err = ParserError::SyntaxError(message.to_string());
    Err(Spanned::new(err, span))
}
