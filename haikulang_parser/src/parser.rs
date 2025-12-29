use crate::ast::AstNode;
use crate::err::{ParserError, ParserErrorKind};
use haikulang_lexer::lexer::{Lexer, LexerResult};
use haikulang_lexer::token::Token;

pub type ParserResult = Result<AstNode, ParserError>;

pub struct Parser<'a> {
    lexer: &'a mut dyn Lexer,
    current_token: LexerResult,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut impl Lexer) -> Self {
        let current_token = lexer.next_token();
        Self {
            lexer,
            current_token,
        }
    }

    pub fn parse(&mut self) -> ParserResult {
        self.parse_syntax_error("no viable alternative".to_string())
    }

    fn parse_syntax_error(&mut self, message: String) -> ParserResult {
        let token = self.advance()?;
        let location = token.location;
        Err(ParserError {
            kind: ParserErrorKind::SyntaxError(token, message),
            location,
        })
    }

    fn advance(&mut self) -> Result<Token, ParserError> {
        // Swap the memory contents around to transfer ownership.
        let mut next = self.lexer.next_token();
        std::mem::swap(&mut self.current_token, &mut next);

        match next {
            Ok(token) => Ok(token),
            Err(lexer_error) => {
                let location = lexer_error.end_location;
                Err(ParserError {
                    kind: ParserErrorKind::LexerError(lexer_error),
                    location,
                })
            }
        }
    }
}
