use crate::location::Location;
use crate::token::{Token, TokenType};
use std::str::FromStr;

#[derive(Debug)]
pub struct Lexer<'a> {
    input: &'a str,
    location: Location,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            location: Location::default(),
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.peek(0) {
            Some('0'..='9') => self.tokenize_num_lit(),
            Some('a'..='z' | 'A'..='Z' | '_') => self.tokenize_ident(),
            Some('"') => self.tokenize_str_lit(),
            Some('+') => self.tokenize_simple(TokenType::Add, 1),
            Some('-') => self.tokenize_simple(TokenType::Sub, 1),
            Some('*') => match self.peek(1) {
                Some('*') => self.tokenize_simple(TokenType::Pow, 2),
                _ => self.tokenize_simple(TokenType::Mul, 1),
            },
            Some('/') => match self.peek(1) {
                Some('/') => self.tokenize_simple(TokenType::IntDiv, 2),
                _ => self.tokenize_simple(TokenType::Div, 1),
            },
            Some('%') => self.tokenize_simple(TokenType::Mod, 1),
            Some(';') => self.tokenize_simple(TokenType::Semi, 1),
            Some('(') => self.tokenize_simple(TokenType::LeftParen, 1),
            Some(')') => self.tokenize_simple(TokenType::RightParen, 1),
            Some(_) => self.tokenize_simple(TokenType::Unknown, 1),
            None => Token {
                token_type: TokenType::Eof,
                raw: "".to_string(),
                location: self.location,
            },
        }
    }

    fn skip_whitespace(&mut self) {
        while matches!(self.peek(0), Some(' ' | '\n' | '\r' | '\t')) {
            self.advance(1);
        }
    }

    // NUMBER_LIT ::= FLOAT_LIT | INT_LIT ;
    //  FLOAT_LIT ::= [0-9]+ , '.' , [0-9]+ , ( [eE] , [+-]? , [0-9]+ ) ?
    //              | [0-9]+ , [eE] , [+-]? , [0-9]+
    //              ;
    //    INT_LIT ::= [0-9]+ ;
    fn tokenize_num_lit(&mut self) -> Token {
        let start_location = self.location;
        self.advance_while(|_, c| matches!(c, '0'..='9'));

        // If we end with a period, 'e', or 'E', then we're parsing a float literal. Otherwise,
        // we're just parsing an int and can stop now.
        if !matches!(self.peek(0), Some('.' | 'e' | 'E')) {
            let raw = &self.input[start_location.offset..self.location.offset];
            return match u64::from_str(raw) {
                Ok(value) => Token {
                    token_type: TokenType::IntLit(value),
                    raw: raw.to_string(),
                    location: start_location,
                },
                Err(_) => Token {
                    token_type: TokenType::MalformedLiteral(
                        "Invalid value for int literal",
                        self.location,
                    ),
                    raw: raw.to_string(),
                    location: start_location,
                },
            };
        }

        // Parse the decimal part.
        if matches!(self.peek(0), Some('.')) {
            self.advance(1);

            // If we have zero decimal digits, we're malformed, so fail out.
            if self.advance_while(|_, c| matches!(c, '0'..='9')) == 0 {
                return Token {
                    token_type: TokenType::MalformedLiteral(
                        "Missing decimal for float literal",
                        self.location,
                    ),
                    raw: self.input[start_location.offset..self.location.offset].to_string(),
                    location: start_location,
                };
            }
        }

        // Parse the exponent.
        if matches!(self.peek(0), Some('e' | 'E')) {
            self.advance(1);

            // Take a single optional + or - before the exponent value.
            if matches!(self.peek(0), Some('+' | '-')) {
                self.advance(1);
            }

            // If we have zero exponent digits, we're malformed, so fail out.
            if self.advance_while(|_, c| matches!(c, '0'..='9')) == 0 {
                return Token {
                    token_type: TokenType::MalformedLiteral(
                        "Missing exponent value for float literal",
                        self.location,
                    ),
                    raw: self.input[start_location.offset..self.location.offset].to_string(),
                    location: start_location,
                };
            }
        }

        let raw = &self.input[start_location.offset..self.location.offset];

        match f64::from_str(raw) {
            Ok(value) => Token {
                token_type: TokenType::FloatLit(value),
                raw: raw.to_string(),
                location: start_location,
            },
            Err(_) => Token {
                token_type: TokenType::MalformedLiteral(
                    "Invalid value for float literal",
                    self.location,
                ),
                raw: raw.to_string(),
                location: start_location,
            },
        }
    }

    //      STR_LIT ::= '"' , ( STR_LIT_CHAR | STR_LIT_ESCAPE * ) , '"' ;
    // STR_LIT_CHAR ::= /* any char, except '"', '\r', '\n', or '\\' */ ;
    fn tokenize_str_lit(&mut self) -> Token {
        let start_location = self.location;
        let mut parsed_string = String::new();

        debug_assert!(matches!(self.peek(0), Some('"')));
        self.advance(1);

        loop {
            match self.peek(0) {
                Some('"') => break,
                Some('\\') => match self.tokenize_str_lit_escape() {
                    Ok(c) => parsed_string.push(c),
                    Err((err, location)) => {
                        return Token {
                            token_type: TokenType::MalformedLiteral(err, location),
                            raw: self.substring(start_location).to_string(),
                            location: start_location,
                        };
                    }
                },
                Some('\r' | '\n') | None => {
                    // Newlines are not allowed in strings.
                    return Token {
                        token_type: TokenType::MalformedLiteral(
                            "Expected string close double quotes",
                            self.location,
                        ),
                        raw: self.substring(start_location).to_string(),
                        location: start_location,
                    };
                }
                Some(c) => {
                    parsed_string.push(c);
                    self.advance(1);
                }
            };
        }

        debug_assert!(matches!(self.peek(0), Some('"')));
        self.advance(1);

        Token {
            token_type: TokenType::StrLit(parsed_string),
            raw: self.substring(start_location).to_string(),
            location: start_location,
        }
    }

    // STR_LIT_ESCAPE ::= '\\' , [rnt\\] ;
    fn tokenize_str_lit_escape(&mut self) -> Result<char, (&'static str, Location)> {
        debug_assert!(matches!(self.peek(0), Some('\\')));

        let location = self.location;
        self.advance(1);
        match self.peek(0) {
            Some('n') => {
                self.advance(1);
                Ok('\n')
            }
            Some('r') => {
                self.advance(1);
                Ok('\r')
            }
            Some('t') => {
                self.advance(1);
                Ok('\t')
            }
            Some('\\') => {
                self.advance(1);
                Ok('\\')
            }
            Some(_) | None => {
                self.advance(1);
                Err(("Unrecognised escape sequence", location))
            }
        }
    }

    // IDENT ::= [A-Za-z_] , [A-Za-z0-9_]+ ;
    fn tokenize_ident(&mut self) -> Token {
        let start_location = self.location;

        debug_assert!(matches!(self.peek(0), Some('a'..='z' | 'A'..='Z' | '_')));
        self.advance_while(|_, c| matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_'));

        Token {
            token_type: TokenType::Ident,
            raw: self.substring(start_location).to_string(),
            location: start_location,
        }
    }

    fn tokenize_simple(&mut self, token_type: TokenType, length: usize) -> Token {
        let token = Token {
            token_type,
            raw: self.peek_range(0, length).to_string(),
            location: self.location,
        };
        self.advance(length);

        token
    }

    fn peek(&self, offset: usize) -> Option<char> {
        if self.location.offset + offset >= self.input.len() {
            return None;
        }

        self.input[self.location.offset + offset..self.location.offset + offset + 1]
            .chars()
            .next()
    }

    fn peek_range(&self, start: usize, end: usize) -> &str {
        let absolute_start = self.location.offset + start;
        let absolute_end = self.location.offset + end;

        &self.input[absolute_start..absolute_end]
    }

    fn advance(&mut self, n: usize) {
        debug_assert!(n > 0, "negative or zero offset {}", n);

        for i in 0..n {
            match &self.input[i..i + 1] {
                "" => break,
                "\n" => {
                    self.location.column = 1;
                    self.location.line += 1;
                }
                _ => {
                    self.location.column += 1;
                }
            }

            self.location.offset += 1;
        }
    }

    fn advance_while(&mut self, condition: impl Fn(usize, char) -> bool) -> usize {
        let mut offset = 0;
        while let Some(c) = self.peek(0)
            && condition(offset, c)
        {
            self.advance(1);
            offset += 1;
        }

        offset
    }

    fn substring(&self, start_location: Location) -> &str {
        &self.input[start_location.offset..self.location.offset]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(                 "123",                  123 ; "regular int")]
    #[test_case(              "012345",                12345 ; "int with a leading zero") ]
    #[test_case("18446744073709551615", 18446744073709551615 ; "max value for a 64 bit unsigned int")]
    fn int_lit_is_tokenized_as_expected(input: &str, expected_output: u64) {
        // Given
        let mut lexer = Lexer::new(input);

        // When
        let token = lexer.next_token();

        // Then
        assert_eq!(&token.raw, input);
        assert_eq!(
            token.location,
            Location {
                offset: 0,
                line: 1,
                column: 1
            }
        );
        assert_eq!(token.token_type, TokenType::IntLit(expected_output));
        assert_eq!(
            lexer.location,
            Location {
                offset: input.len(),
                line: 1,
                column: 1 + input.len()
            }
        );
    }

    #[test_case(              "1234.5",               1234.5 ; "float with no exponent")]
    #[test_case(               "123e4",                123e4 ; "float with lowercase exponent but no decimal")]
    #[test_case(               "123E4",                123e4 ; "float with uppercase exponent but no decimal")]
    #[test_case(              "123e+4",                123e4 ; "float with lowercase exponent and plus but no decimal")]
    #[test_case(              "123E+4",                123e4 ; "float with uppercase exponent and plus but no decimal")]
    #[test_case(              "123e-4",               123e-4 ; "float with lowercase exponent and minus but no decimal")]
    #[test_case(              "123E-4",               123e-4 ; "float with uppercase exponent and minus but no decimal")]
    #[test_case(           "123.987e4",            123.987e4 ; "float with lowercase exponent and decimal")]
    #[test_case(           "123.987E4",            123.987e4 ; "float with uppercase exponent and decimal")]
    #[test_case(          "123.987e+4",            123.987e4 ; "float with lowercase exponent, plus and decimal")]
    #[test_case(          "123.987E+4",            123.987e4 ; "float with uppercase exponent, plus and decimal")]
    #[test_case(          "123.987e-4",           123.987e-4 ; "float with lowercase exponent, minus and decimal")]
    #[test_case(          "123.987E-4",           123.987e-4 ; "float with uppercase exponent, minus and decimal")]
    #[test_case("2.78281828459045e123", 2.78281828459045e123 ; "float with a big exponent")]
    fn float_lit_is_tokenized_as_expected(input: &str, expected_output: f64) {
        // Given
        let mut lexer = Lexer::new(input);

        // When
        let token = lexer.next_token();

        // Then
        assert_eq!(&token.raw, input);
        assert_eq!(
            token.location,
            Location {
                offset: 0,
                line: 1,
                column: 1
            }
        );
        assert_eq!(token.token_type, TokenType::FloatLit(expected_output));
        assert_eq!(
            lexer.location,
            Location {
                offset: input.len(),
                line: 1,
                column: 1 + input.len()
            }
        );
    }
}
