use std::iter::Peekable;
use std::str::Chars;
use crate::lexer::token::{Token, TokenType};
use super::location::Location;

#[derive(Debug)]
pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    location: Location,
}

impl<'a> Lexer<'a> {
    pub fn new(input: Chars<'a>) -> Self {
        Self {
            input: input.peekable(),
            location: Location::default(),
        }
    }
    
    fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        
        if let Some(c) = self.peek_char() {
            todo!();
        } else {
            Token { token_type: TokenType::Eof, data: "".to_string(), location: self.location.clone() }
        }
    }        
    
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek_char() && matches!(c, ' ' | '\n' | '\r' | '\t') {
            self.next_char().unwrap();
        }
    }
    
    fn peek_char(&mut self) -> Option<&char> {
        self.input.peek()
    }
    
    fn next_char(&mut self) -> Option<char> {
        if let Some(c) = self.input.next() {
            match c {
                '\n' => {
                    self.location.column = 1;
                    self.location.line += 1;
                }
                _ => self.location.column += 1,
            };
            self.location.offset += 1;
            
            Some(c)
        } else {
            None
        }
    }
}
