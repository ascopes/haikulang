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
            Some('a'..='z' | 'A'..='Z' | '_') => self.tokenize_keyword_or_ident(),
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
            Some('!') => match self.peek(1) {
                Some('=') => self.tokenize_simple(TokenType::Ne, 2),
                _ => self.tokenize_simple(TokenType::Not, 1),
            },
            Some('<') => match self.peek(1) {
                Some('=') => self.tokenize_simple(TokenType::Lte, 2),
                _ => self.tokenize_simple(TokenType::Lt, 1),
            },
            Some('>') => match self.peek(1) {
                Some('=') => self.tokenize_simple(TokenType::Gte, 2),
                _ => self.tokenize_simple(TokenType::Gt, 1),
            },
            Some('=') => match self.peek(1) {
                Some('=') => self.tokenize_simple(TokenType::Eq, 2),
                _ => self.tokenize_simple(TokenType::Assign, 1),
            },
            Some('(') => self.tokenize_simple(TokenType::LeftParen, 1),
            Some(')') => self.tokenize_simple(TokenType::RightParen, 1),
            Some('{') => self.tokenize_simple(TokenType::LeftBrace, 1),
            Some('}') => self.tokenize_simple(TokenType::RightBrace, 1),
            Some('[') => self.tokenize_simple(TokenType::LeftSq, 1),
            Some(']') => self.tokenize_simple(TokenType::RightSq, 1),
            Some(';') => self.tokenize_simple(TokenType::Semi, 1),
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
    fn tokenize_keyword_or_ident(&mut self) -> LexerResult {
        let start_location = self.location;

        debug_assert!(matches!(self.peek(0), Some('a'..='z' | 'A'..='Z' | '_')));
        self.advance_while(|_, c| matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_'));

        let raw = self.substring(start_location);

        let token_type = match raw {
            "fn" => TokenType::Fn,
            "return" => TokenType::Return,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "for" => TokenType::For,
            "while" => TokenType::While,
            "break" => TokenType::Break,
            "continue" => TokenType::Continue,
            "true" => TokenType::True,
            "false" => TokenType::False,
            _ => TokenType::Ident,
        };

        Ok(Token {
            token_type,
            raw: raw.to_string(),
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

    #[test_case("0bhello" ; "literal with garbage value")]
    #[test_case("0b"      ; "incomplete literal with EOF")]
    fn invalid_bin_int_lit_emits_error(input: &str) {
        // Given
        let mut lexer = BasicLexer::new(input);

        // When
        let error = lexer.next_token().expect_err("tokenization succeeded");

        // Then
        assert_eq!(
            error.start_location,
            Location {
                offset: 0,
                line: 1,
                column: 1
            }
        );
        assert_eq!(
            error.end_location,
            Location {
                offset: 2,
                line: 1,
                column: 3
            }
        );
        assert_eq!(
            error.kind,
            LexerErrorKind::InvalidNumericLiteral("missing binary literal value".to_string())
        );
        assert_eq!(&error.raw, "0b");
    }

    #[test_case("0ohello" ; "literal with garbage value")]
    #[test_case("0o"      ; "incomplete literal with EOF")]
    fn invalid_oct_int_lit_emits_error(input: &str) {
        // Given
        let mut lexer = BasicLexer::new(input);

        // When
        let error = lexer.next_token().expect_err("tokenization succeeded");

        // Then
        assert_eq!(
            error.start_location,
            Location {
                offset: 0,
                line: 1,
                column: 1
            }
        );
        assert_eq!(
            error.end_location,
            Location {
                offset: 2,
                line: 1,
                column: 3
            }
        );
        assert_eq!(
            error.kind,
            LexerErrorKind::InvalidNumericLiteral("missing octal literal value".to_string())
        );
        assert_eq!(&error.raw, "0o");
    }

    #[test_case("0xhello" ; "literal with garbage value")]
    #[test_case("0x"      ; "incomplete literal with EOF")]
    fn invalid_hex_int_lit_emits_error(input: &str) {
        // Given
        let mut lexer = BasicLexer::new(input);

        // When
        let error = lexer.next_token().expect_err("tokenization succeeded");

        // Then
        assert_eq!(
            error.start_location,
            Location {
                offset: 0,
                line: 1,
                column: 1
            }
        );
        assert_eq!(
            error.end_location,
            Location {
                offset: 2,
                line: 1,
                column: 3
            }
        );
        assert_eq!(
            error.kind,
            LexerErrorKind::InvalidNumericLiteral("missing hexadecimal literal value".to_string())
        );
        assert_eq!(&error.raw, "0x");
    }

    #[test]
    fn too_long_int_lit_emits_error() {
        // Given
        let mut lexer = BasicLexer::new("0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF");

        // When
        let error = lexer.next_token().expect_err("tokenization succeeded");

        // Then
        assert_eq!(error.raw, "0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF");
        assert_eq!(
            error.kind,
            LexerErrorKind::InvalidNumericLiteral(
                "number too large to fit in target type".to_string()
            )
        );
        assert_eq!(
            error.start_location,
            Location {
                offset: 0,
                line: 1,
                column: 1
            }
        );
        assert_eq!(
            error.end_location,
            Location {
                offset: 35,
                line: 1,
                column: 36
            }
        );
    }

    #[test]
    fn invalid_num_lit_prefix_emitted_as_multiple_tokens() {
        // Given
        let mut lexer = BasicLexer::new("0lol123");

        // When
        let token1 = lexer.next_token().expect("tokenization failed");
        let token2 = lexer.next_token().expect("tokenization failed");

        // Then
        assert_eq!(token1.raw, "0");
        assert_eq!(token1.token_type, TokenType::IntLit(0));

        assert_eq!(token2.raw, "lol123");
        assert_eq!(token2.token_type, TokenType::Ident);
    }

    #[test_case(      "fn",         TokenType::Fn ; "'fn' keyword")]
    #[test_case(  "return",     TokenType::Return ; "'return' keyword")]
    #[test_case(      "if",         TokenType::If ; "'if' keyword")]
    #[test_case(    "else",       TokenType::Else ; "'else' keyword")]
    #[test_case(     "for",        TokenType::For ; "'for' keyword")]
    #[test_case(   "while",      TokenType::While ; "'while' keyword")]
    #[test_case(   "break",      TokenType::Break ; "'break' keyword")]
    #[test_case("continue",   TokenType::Continue ; "'continue' keyword")]
    #[test_case(       "+",        TokenType::Add ; "add operator")]
    #[test_case(       "-",        TokenType::Sub ; "subtract operator")]
    #[test_case(       "*",        TokenType::Mul ; "multiply operator")]
    #[test_case(       "/",        TokenType::Div ; "divide operator")]
    #[test_case(      "//",      TokenType::IntDiv ; "integer divide operator")]
    #[test_case(       "%",        TokenType::Mod ; "modulo operator")]
    #[test_case(      "**",        TokenType::Pow ; "power operator")]
    #[test_case(       "!",        TokenType::Not ; "boolean negation operator")]
    #[test_case(      "==",         TokenType::Eq ; "equality operator")]
    #[test_case(      "!=",         TokenType::Ne ; "inequality operator")]
    #[test_case(       "<",         TokenType::Lt ; "less than operator")]
    #[test_case(      "<=",        TokenType::Lte ; "less than or equal operator")]
    #[test_case(       ">",         TokenType::Gt ; "greater than operator")]
    #[test_case(      ">=",        TokenType::Gte ; "greater than or equal operator")]
    #[test_case(       "=",     TokenType::Assign ; "assignment operator")]
    #[test_case(       "(",  TokenType::LeftParen ; "left parenthesis")]
    #[test_case(       ")", TokenType::RightParen ; "right parenthesis")]
    #[test_case(       "{",  TokenType::LeftBrace ; "left brace")]
    #[test_case(       "}", TokenType::RightBrace ; "right brace")]
    #[test_case(       "[",     TokenType::LeftSq ; "left square bracket")]
    #[test_case(       "]",    TokenType::RightSq ; "right square bracket")]
    #[test_case(       ";",       TokenType::Semi ; "semicolon")]
    fn static_symbols_are_resolved_as_expected(input: &str, expected_type: TokenType) {
        // Given
        let mut lexer = BasicLexer::new(input);

        // When
        let token = lexer.next_token().expect("tokenization failed");

        // Then
        assert_eq!(token.raw, input);
        assert_eq!(token.token_type, expected_type);
        assert_eq!(
            token.location,
            Location {
                offset: 0,
                line: 1,
                column: 1
            }
        );
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
