#![cfg(test)]

use haikulang_lexer::lexer::{Lexer, LexerResult};
use haikulang_lexer::location::Location;
use haikulang_lexer::token::{Token, TokenType};
use std::collections::VecDeque;

/// Stubbed lexer type that emits the results that it is constructed from.
pub struct MockLexer(VecDeque<LexerResult>);

impl MockLexer {
    pub fn new(tokens: Vec<LexerResult>) -> Self {
        Self(VecDeque::from(tokens))
    }
}

impl Lexer for MockLexer {
    fn next_token(&mut self) -> LexerResult {
        self.0.pop_front().unwrap_or_else(|| {
            Ok(Token {
                token_type: TokenType::Eof,
                raw: "".to_string(),
                location: Location::default(),
            })
        })
    }
}
