use crate::lexer::token::Token;
use crate::lexer::token_stream::TokenStream;
use crate::parser::error::ParserError;
use crate::span::Spanned;

pub type ParserResult<T> = Result<Spanned<T>, Spanned<ParserError>>;

pub struct Parser<'src> {
    stream: TokenStream<'src>,
}

impl<'src> Parser<'src> {
    pub fn new(stream: TokenStream<'src>) -> Self {
        let mut parser = Self { stream };
        parser.consume_comments();
        parser
    }

    // Return a copy of the current token within the lexer.
    #[inline]
    pub(super) fn current(&mut self) -> ParserResult<Token> {
        self.stream.current().map_err(|err| {
            let new_err = ParserError::LexerError(err.value().clone());
            Spanned::new(new_err, err.span())
        })
    }

    // Advance the lexer to the next token.
    #[inline]
    pub(super) fn advance(&mut self) {
        self.stream.advance();
        self.consume_comments();
    }

    // Repeatedly take comments from the token stream.
    // Eventually, we may want to inspect comments and store them if they meet certain
    // criteria, to allow for parsing documentation. For now though, we just discard them.
    fn consume_comments(&mut self) {
        while let Ok(token) = self.current()
            && matches!(
                token.value(),
                Token::InlineComment(_) | Token::MultilineComment(_)
            )
        {
            self.stream.advance();
        }
    }

    // Verify the current token equals a given token, advance the lexer, and return
    // the verified token. If it doesn't match, then nothing is advanced, and a
    // syntax error is instead returned specifying that the current token is expected
    // to match the given string description.
    #[inline]
    pub(super) fn eat(&mut self, expected_token: Token, description: &str) -> ParserResult<Token> {
        let current = self.current()?;
        if current.value() == expected_token {
            self.advance();
            Ok(current)
        } else {
            Err(Spanned::new(
                ParserError::SyntaxError(format!("expected {}", description)),
                current.span(),
            ))
        }
    }
}

// Internal macro to allow asserting a token or AST matches a condition that should
// always be true. Generally, it is preferred to use the 'eat' function which also advances the
// lexer, but this is useful when splitting out preconditions.
#[macro_export]
macro_rules! debug_assert_matches {
    ($current: expr, $matcher: pat) => {
        debug_assert!(
            matches!($current, $matcher),
            "bug: expected expr {} to match {}, but it was actually {:?}",
            stringify!($current),
            stringify!($matcher),
            $current
        );
    };
}
