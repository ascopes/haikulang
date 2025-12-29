use super::err::LexerError;
use super::token::Token;
use crate::lexer::basic_lexer::BasicLexer;

pub type LexerResult = Result<Token, LexerError>;

pub trait Lexer {
    fn next_token(&mut self) -> LexerResult;
}

fn new_default_lexer(input: &str) -> impl Lexer {
    BasicLexer::new(input)
}
