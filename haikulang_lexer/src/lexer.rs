use crate::err::LexerError;
use crate::token::Token;

pub type LexerResult = Result<Token, LexerError>;

/// Interface that a lexer should expose.
///
/// Abstracted to `StringLexer` to allow for stubbing/mocking in other modules.
pub trait Lexer {
    fn next_token(&mut self) -> LexerResult;
}
