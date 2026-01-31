use crate::lexer::LexerError;

#[derive(Clone, Debug)]
pub enum ParserError {
    UnknownSequence(String),
    SyntaxError(String),
    LexerError(LexerError),
}
