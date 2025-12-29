use crate::lexer::err::{LexerError, LexerErrorKind};
use crate::lexer::lexer::{Lexer, LexerResult};
use crate::lexer::token::{FloatValue, IntValue, Token, TokenType};
use crate::location::Location;
use std::str::FromStr;

#[derive(Debug)]
pub(super) struct BasicLexer<'a> {
    input: &'a str,
    location: Location,
}

impl<'a> Lexer for BasicLexer<'a> {
    fn next_token(&mut self) -> LexerResult {
        self.next()
    }
}

impl<'a> BasicLexer<'a> {
    pub(super) fn new(input: &'a str) -> Self {
        Self {
            input,
            location: Location::default(),
        }
    }

    #[inline(always)]
    fn next(&mut self) -> LexerResult {
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
            Some(c) => Err(LexerError {
                kind: LexerErrorKind::UnrecognisedCharacter,
                raw: c.to_string(),
                start_location: self.location,
                end_location: self.location,
            }),
            None => Ok(Token {
                token_type: TokenType::Eof,
                raw: "".to_string(),
                location: self.location,
            }),
        }
    }

    fn skip_whitespace(&mut self) {
        while matches!(self.peek(0), Some(' ' | '\n' | '\r' | '\t')) {
            self.advance(1);
        }
    }

    // NUM_LIT ::= BIN_INT_LIT
    //           | OCT_INT_LIT
    //           | HEX_INT_LIT
    //           | DEC_NUM_LIT
    //           ;
    fn tokenize_num_lit(&mut self) -> LexerResult {
        if self.peek(0).unwrap() == '0' {
            match self.peek(1) {
                Some('b' | 'B') => return self.tokenize_bin_int_lit(),
                Some('o' | 'O') => return self.tokenize_oct_int_lit(),
                Some('x' | 'X') => return self.tokenize_hex_int_lit(),
                _ => {}
            };
        }

        self.tokenize_dec_num_lit()
    }

    // BIN_INT_LIT ::= ( '0b' | '0B' ) , [01] + ;
    fn tokenize_bin_int_lit(&mut self) -> LexerResult {
        let location = self.location;
        self.advance(2);
        debug_assert!(matches!(self.substring(location), "0b" | "0B"));

        let mut digits = String::new();

        if self.take_digits_with_radix(&mut digits, 2) == 0 {
            return Err(LexerError {
                kind: LexerErrorKind::InvalidNumericLiteral(
                    "missing binary literal value".to_string(),
                ),
                raw: self.substring(location).to_string(),
                start_location: location,
                end_location: self.location,
            });
        }

        self.parse_int(&digits, 2, location)
    }

    // OCT_INT_LIT ::= ( '0o' | '0O' ) , [0-7] + ;
    fn tokenize_oct_int_lit(&mut self) -> LexerResult {
        let location = self.location;
        self.advance(2);
        debug_assert!(matches!(self.substring(location), "0o" | "0O"));

        let mut digits = String::new();

        if self.take_digits_with_radix(&mut digits, 8) == 0 {
            return Err(LexerError {
                kind: LexerErrorKind::InvalidNumericLiteral(
                    "missing octal literal value".to_string(),
                ),
                raw: self.substring(location).to_string(),
                start_location: location,
                end_location: self.location,
            });
        }

        self.parse_int(&digits, 8, location)
    }

    // HEX_INT_LIT ::= ( '0x' | '0X' ) , [0-9a-fA-F] + ;
    fn tokenize_hex_int_lit(&mut self) -> LexerResult {
        let location = self.location;
        self.advance(2);
        debug_assert!(matches!(self.substring(location), "0x" | "0X"));

        let mut digits = String::new();

        if self.take_digits_with_radix(&mut digits, 16) == 0 {
            return Err(LexerError {
                kind: LexerErrorKind::InvalidNumericLiteral(
                    "missing hexadecimal literal value".to_string(),
                ),
                raw: self.substring(location).to_string(),
                start_location: location,
                end_location: self.location,
            });
        }

        self.parse_int(&digits, 16, location)
    }

    // DEC_NUM_LIT   ::= DEC_FLOAT_LIT | DEC_INT_LIT ;
    // DEC_FLOAT_LIT ::= [0-9] + , ( '.' , [0-9] * ) ? , ( [eE] , [+-] ? , [0-9] + )
    //                 | [0-9] + , '.' , [0-9] +
    //                 ;
    // DEC_INT_LIT   ::= [0-9] + ;
    fn tokenize_dec_num_lit(&mut self) -> LexerResult {
        let location = self.location;
        let mut is_float = false;

        let mut number = String::new();
        let initial_digits = self.take_digits_with_radix(&mut number, 10);
        debug_assert!(initial_digits > 0);

        if matches!(self.peek(0), Some('.')) {
            number.push('.');
            is_float = true;
            self.advance(1);

            // We allow a dangling period if an exponent follows the fraction part.
            if self.take_digits_with_radix(&mut number, 10) == 0
                && !matches!(self.peek(0), Some('e' | 'E'))
            {
                return Err(LexerError {
                    kind: LexerErrorKind::InvalidNumericLiteral(
                        "incomplete float mantissa".to_string(),
                    ),
                    raw: self.substring(location).to_string(),
                    start_location: location,
                    end_location: self.location,
                });
            }
        }

        if matches!(self.peek(0), Some('e' | 'E')) {
            number.push('e');
            is_float = true;
            self.advance(1);

            if let Some(c) = self.peek(0)
                && matches!(c, '+' | '-')
            {
                number.push(c);
                self.advance(1);
            }

            if self.take_digits_with_radix(&mut number, 10) == 0 {
                return Err(LexerError {
                    kind: LexerErrorKind::InvalidNumericLiteral(
                        "incomplete float exponent".to_string(),
                    ),
                    raw: self.substring(location).to_string(),
                    start_location: location,
                    end_location: self.location,
                });
            }
        }

        if is_float {
            self.parse_float(&number, location)
        } else {
            self.parse_int(&number, 10, location)
        }
    }

    fn take_digits_with_radix(&mut self, value: &mut String, radix: u32) -> usize {
        let mut offset = 0usize;
        while let Some(c) = self.peek(0)
            && c.is_digit(radix)
        {
            value.push(c);
            self.advance(1);
            offset += 1;
        }

        offset
    }

    fn parse_int(&mut self, digits: &str, radix: u32, location: Location) -> LexerResult {
        match IntValue::from_str_radix(&digits, radix) {
            Ok(value) => Ok(Token {
                token_type: TokenType::IntLit(value),
                raw: self.substring(location).to_string(),
                location,
            }),
            Err(cause) => Err(LexerError {
                kind: LexerErrorKind::InvalidNumericLiteral(cause.to_string()),
                raw: self.substring(location).to_string(),
                start_location: location,
                end_location: self.location,
            }),
        }
    }

    fn parse_float(&mut self, number: &str, location: Location) -> LexerResult {
        match FloatValue::from_str(&number) {
            Ok(value) => Ok(Token {
                token_type: TokenType::FloatLit(value),
                raw: self.substring(location).to_string(),
                location,
            }),
            Err(cause) => Err(LexerError {
                kind: LexerErrorKind::InvalidNumericLiteral(cause.to_string()),
                raw: self.substring(location).to_string(),
                start_location: location,
                end_location: self.location,
            }),
        }
    }

    //      STR_LIT ::= '"' , ( STR_LIT_CHAR | STR_LIT_ESCAPE * ) , '"' ;
    // STR_LIT_CHAR ::= /* any char, except '"', '\r', '\n', or '\\' */ ;
    fn tokenize_str_lit(&mut self) -> LexerResult {
        let start_location = self.location;
        let mut parsed_string = String::new();

        debug_assert!(matches!(self.peek(0), Some('"')));
        self.advance(1);

        loop {
            match self.peek(0) {
                Some('"') => break,
                Some('\\') => match self.tokenize_str_lit_escape() {
                    Ok(c) => parsed_string.push(c),
                    Err(err) => return Err(err),
                },
                Some('\r' | '\n') | None => {
                    // Newlines are not allowed in strings.
                    return Err(LexerError {
                        kind: LexerErrorKind::UnexpectedEndOfLine,
                        raw: self.substring(start_location).to_string(),
                        start_location,
                        end_location: self.location,
                    });
                }
                Some(c) => {
                    parsed_string.push(c);
                    self.advance(1);
                }
            };
        }

        debug_assert!(matches!(self.peek(0), Some('"')));
        self.advance(1);

        Ok(Token {
            token_type: TokenType::StrLit(parsed_string),
            raw: self.substring(start_location).to_string(),
            location: start_location,
        })
    }

    // STR_LIT_ESCAPE ::= '\\' , [rnt\\] ;
    fn tokenize_str_lit_escape(&mut self) -> Result<char, LexerError> {
        let location = self.location;

        debug_assert!(matches!(self.peek(0), Some('\\')));
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
                Err(LexerError {
                    kind: LexerErrorKind::UnrecognisedStringEscape,
                    raw: self.substring(location).to_string(),
                    start_location: location,
                    end_location: self.location,
                })
            }
        }
    }

    // IDENT ::= [A-Za-z_] , [A-Za-z0-9_]+ ;
    fn tokenize_ident(&mut self) -> LexerResult {
        let start_location = self.location;

        debug_assert!(matches!(self.peek(0), Some('a'..='z' | 'A'..='Z' | '_')));
        self.advance_while(|_, c| matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_'));

        Ok(Token {
            token_type: TokenType::Ident,
            raw: self.substring(start_location).to_string(),
            location: start_location,
        })
    }

    fn tokenize_simple(&mut self, token_type: TokenType, length: usize) -> LexerResult {
        let token = Token {
            token_type,
            raw: self.peek_range(0, length).to_string(),
            location: self.location,
        };
        self.advance(length);

        Ok(token)
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

    #[test_case(                                                         "0b1111011",                  123 ; "regular int with lowercase prefix")]
    #[test_case(                                                         "0B1111010",                  122 ; "regular int with uppercase prefix")]
    #[test_case(                                                       "0b001111010",                  122 ; "int with leading zero")]
    #[test_case("0b1111111111111111111111111111111111111111111111111111111111111111", 18446744073709551615 ; "max value for a 64 bit unsigned int")]
    fn bin_int_lit_is_tokenized_as_expected(input: &str, expected_output: u64) {
        // Given
        let mut lexer = BasicLexer::new(input);

        // When
        let token = lexer.next_token().expect("tokenization failed");

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

    #[test_case(                   "0o123",                   83 ; "regular lowercase int")]
    #[test_case(                   "0O125",                   85 ; "regular uppercase int")]
    #[test_case(                "0o012345",                 5349 ; "int with a leading zero")]
    #[test_case("0o1777777777777777777777", 18446744073709551615 ; "max value for a 64 bit unsigned int")]
    fn oct_int_lit_is_tokenized_as_expected(input: &str, expected_output: u64) {
        // Given
        let mut lexer = BasicLexer::new(input);

        // When
        let token = lexer.next_token().expect("tokenization failed");

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

    #[test_case(          "0x1a2b3c",              1715004 ; "regular lowercase int")]
    #[test_case(          "0X1A2B3F",              1715007 ; "regular uppercase int")]
    #[test_case(         "0x01a2b3F",              1715007 ; "int with a leading zero")]
    #[test_case("0xFFFFFFFFFFFFFFFF", 18446744073709551615 ; "max value for a 64 bit unsigned int")]
    fn hex_int_lit_is_tokenized_as_expected(input: &str, expected_output: u64) {
        // Given
        let mut lexer = BasicLexer::new(input);

        // When
        let token = lexer.next_token().expect("tokenization failed");

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

    #[test_case(                 "123",                  123 ; "regular int")]
    #[test_case(              "012345",                12345 ; "int with a leading zero")]
    #[test_case("18446744073709551615", 18446744073709551615 ; "max value for a 64 bit unsigned int")]
    fn dec_int_lit_is_tokenized_as_expected(input: &str, expected_output: u64) {
        // Given
        let mut lexer = BasicLexer::new(input);

        // When
        let token = lexer.next_token().expect("tokenization failed");

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
    #[test_case(              "123.e4",              123.0e4 ; "float with lowercase exponent and missing decimal")]
    #[test_case(           "123.987e4",            123.987e4 ; "float with lowercase exponent and decimal")]
    #[test_case(           "123.987E4",            123.987e4 ; "float with uppercase exponent and decimal")]
    #[test_case(          "123.987e+4",            123.987e4 ; "float with lowercase exponent, plus and decimal")]
    #[test_case(          "123.987E+4",            123.987e4 ; "float with uppercase exponent, plus and decimal")]
    #[test_case(          "123.987e-4",           123.987e-4 ; "float with lowercase exponent, minus and decimal")]
    #[test_case(          "123.987E-4",           123.987e-4 ; "float with uppercase exponent, minus and decimal")]
    #[test_case("2.78281828459045e123", 2.78281828459045e123 ; "float with a big exponent")]
    fn float_lit_is_tokenized_as_expected(input: &str, expected_output: f64) {
        // Given
        let mut lexer = BasicLexer::new(input);

        // When
        let token = lexer.next_token().expect("tokenization failed");

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
