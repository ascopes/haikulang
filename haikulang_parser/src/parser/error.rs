use crate::lexer::error::LexerError;

#[derive(Clone, Debug)]
pub enum ParserError {
    UnknownSequence(String),
    SyntaxError(String),
    LexerError(LexerError),
}
