use crate::lexer::LexerError;

#[derive(Clone, Debug)]
pub enum ParserError {
    LexerError(LexerError),
}
