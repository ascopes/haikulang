use std::iter::{FusedIterator, Peekable};
use std::str::Chars;
use crate::lexer::token::Token;
use super::location::Location;

#[derive(Debug)]
pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    current_location: Location,
}

impl<'a> Lexer<'a> {
    pub fn new(input: Chars<'a>) -> Self {
        Self {
            input: input.peekable(),
            current_location: Location::default(),
        }
    }
    
    fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();
        
        if let Some(c) = self.peek_char() {
            todo!();
        } else {
            None
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
                    self.current_location.column = 1;
                    self.current_location.line += 1;
                }
                _ => self.current_location.column += 1,
            };
            self.current_location.offset += 1;
            
            Some(c)
        } else {
            None
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()   
    }
}

impl<'a> FusedIterator for Lexer<'a> {}
