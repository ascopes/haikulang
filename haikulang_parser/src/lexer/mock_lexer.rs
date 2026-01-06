#![cfg(test)]

use crate::lexer::err::{LexerError, LexerErrorKind};
use crate::lexer::lexer::{Lexer, LexerResult};
use crate::lexer::token::{Token, TokenType};
use crate::location::Location;
use std::collections::VecDeque;
use std::string::ToString;

// Mocked Lexer that can be fed tokens, errors, or operations that change the
// reported location, and that emits each token one by one until exhausted.
pub struct MockLexer {
    location: Location,
    actions: VecDeque<Action>,
}

impl MockLexer {
    pub fn new() -> Self {
        Self {
            location: Location::default(),
            actions: VecDeque::new(),
        }
    }

    pub fn token(&mut self, token_type: TokenType, raw: impl ToString) -> &mut Self {
        self.actions.push_back(Action::Token(Token {
            token_type,
            raw: raw.to_string(),
            // We inject the real
            location: Location::default(),
        }));
        return self;
    }

    pub fn error(&mut self, kind: LexerErrorKind, raw: impl ToString) -> &mut Self {
        self.actions.push_back(Action::Error(LexerError {
            kind,
            raw: raw.to_string(),
            location: Location::default(),
        }));
        return self;
    }

    pub fn nl(&mut self) -> &mut Self {
        self.actions.push_back(Action::NewLine);
        return self;
    }

    fn advance(&mut self, raw: &String) {
        for c in raw.chars() {
            self.location.offset += 1;
            match c {
                '\n' => {
                    self.location.line += 1;
                    self.location.column = 1;
                }
                _ => self.location.column += 1,
            }
        }

        if let Some(c) = raw.chars().last()
            && c != '\n'
        {
            // Simulate a space separating us from the previous token.
            self.location.offset += 1;
            self.location.column += 1;
        }
    }
}

impl Lexer for MockLexer {
    fn next_token(&mut self) -> LexerResult {
        loop {
            match self.actions.pop_front() {
                Some(Action::Token(mut token)) => {
                    token.location = self.location;
                    self.advance(&token.raw);
                    return Ok(token);
                }
                Some(Action::Error(mut err)) => {
                    err.location = self.location;
                    self.advance(&err.raw);
                    return Err(err);
                }
                Some(Action::NewLine) => {
                    self.location.offset += 1;
                    self.location.column = 1;
                    self.location.line += 1;
                }
                None => {
                    return Ok(Token {
                        token_type: TokenType::Eof,
                        raw: "".to_string(),
                        location: self.location,
                    });
                }
            }
        }
    }
}

enum Action {
    Token(Token),
    Error(LexerError),
    NewLine,
}
