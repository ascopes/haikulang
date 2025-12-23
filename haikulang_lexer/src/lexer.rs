use crate::location::Location;
use crate::token::{Token, TokenType};
use std::iter::Peekable;
use std::str::{Chars, FromStr};

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

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let next_char = if let Some(c) = self.peek_char() {
            c
        } else {
            return self.eof();
        };

        match next_char {
            '0'..='9' => self.number(),
            'a'..='z' | 'A'..='Z' | '_' => self.ident(),
            '"' => self.str(),
            _ => self.unknown(),
        }
    }

    /// WHITESPACE ::= ( ' ' | '\n' | '\r' | '\t' )+ ;
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek_char()
            && matches!(c, ' ' | '\n' | '\r' | '\t')
        {
            self.next_char().unwrap();
        }
    }

    /// EOF ::= ;
    fn eof(&self) -> Token {
        Token {
            token_type: TokenType::Eof,
            raw: "".to_string(),
            location: self.location.clone(),
        }
    }

    /// FLOAT           ::= FLOAT_MANTISSA , FLOAT_EXPONENT? ;
    /// _FLOAT_MANTISSA ::= [0-9]+ , ( '.' , [0-9]* )? ;
    /// _FLOAT_EXPONENT ::= ( [eE] , [+-]? , [0-9]+ )
    /// INT    ::= [0-9]+ ;
    ///
    /// Note that a FLOAT without a decimal part and without an exponent is treated as an INT
    /// literal.
    fn number(&mut self) -> Token {
        let location = self.location.clone();
        // We "promote" to a float later if we find floaty characters!
        let mut float = false;
        // First char is guaranteed to be a digit.
        let mut raw = self.next_char().unwrap().to_string();

        // Parse the integer bit.
        while matches!(self.peek_char(), Some('0'..='9')) {
            raw.push(self.next_char().unwrap());
        }

        if let Some('.') = self.peek_char() {
            float = true;

            raw.push('.');

            while matches!(self.peek_char(), Some('0'..='9')) {
                raw.push(self.next_char().unwrap());
            }
        }

        if let Some('e') | Some('E') = self.peek_char() {
            float = true;

            raw.push(self.next_char().unwrap());

            if let Some('+') | Some('-') = self.peek_char() {
                raw.push(self.next_char().unwrap());
            }

            let mut at_least_one_char = false;
            while matches!(self.peek_char(), Some('0'..='9')) {
                at_least_one_char = true;
                raw.push(self.next_char().unwrap());
            }

            // If we do not have at least one digit in the exponent, choke.
            if !at_least_one_char {
                return Token {
                    token_type: TokenType::MalformedLiteral(
                        "Failed to parse float literal: missing exponent".to_string(),
                        self.location.clone(),
                    ),
                    raw,
                    location,
                };
            }
        }

        let token_type = if float {
            match f64::from_str(&raw) {
                Ok(float_value) => TokenType::Float(float_value),
                Err(error) => TokenType::MalformedLiteral(
                    format!("Failed to parse float literal: {}", error),
                    self.location.clone(),
                ),
            }
        } else {
            match u64::from_str(&raw) {
                Ok(int_value) => TokenType::Int(int_value),
                Err(error) => TokenType::MalformedLiteral(
                    format!("Failed to parse int literal: {}", error),
                    self.location.clone(),
                ),
            }
        };

        Token {
            token_type,
            raw,
            location,
        }
    }

    /// IDENT        ::= _IDENT_START , _IDENT_REST*;
    /// _IDENT_START ::= [_a-zA-Z] ;
    /// _IDENT_REST  ::= [_a-zA-Z0-9] ;
    fn ident(&mut self) -> Token {
        let location = self.location.clone();
        // First char is guaranteed to be an identifier start character.
        let mut raw = self.next_char().unwrap().to_string();

        while matches!(
            self.peek_char(),
            Some('a'..='z' | 'A'..='Z' | '0'..='9' | '_')
        ) {
            raw.push(self.next_char().unwrap());
        }

        Token {
            token_type: TokenType::Ident,
            raw,
            location,
        }
    }

    /// STR         ::= '"' , (_STR_ESCAPE | _STR_CHAR)* , '"' ;
    /// _STR_ESCAPE ::= '\\' , ["nrt\\] ;
    /// _STR_CHAR   ::= [^\r\n\\"] ;
    fn str(&mut self) -> Token {
        todo!();
    }

    /// UNKNOWN ::= /* any character not matching the rest of the grammar */ ;
    fn unknown(&mut self) -> Token {
        let location = self.location.clone();

        Token {
            token_type: TokenType::Unknown,
            raw: self.next_char().unwrap().to_string(),
            location,
        }
    }

    #[must_use]
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
