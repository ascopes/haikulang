use crate::lexer::{LexerError, Token};

#[derive(Clone, Debug)]
pub enum ParserError {
    SyntaxError(Token, String),
    LexerError(LexerError),
}
