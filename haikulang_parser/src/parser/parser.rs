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
        Self { stream }
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
    }

    // Verify the current token matches a predicate, advance the lexer, and return
    // the verified token. If it doesn't match, then nothing is advanced, and a
    // syntax error is instead returned specifying that the current token is expected
    // to match the given string description.
    #[inline]
    pub(super) fn eat<F>(&mut self, matcher: F, description: &str) -> ParserResult<Token>
    where
        F: FnOnce(Token) -> bool,
    {
        let current = self.current()?;
        if matcher(current.value()) {
            self.advance();
            Ok(current)
        } else {
            Err(Spanned::new(
                ParserError::SyntaxError(format!("expected {}", description)),
                current.span(),
            ))
        }
    }

    /*
     * Common helpers and operations.
     */
    #[inline]
    pub(super) fn eat_identifier(&mut self) -> ParserResult<String> {
        let current = self.current()?;
        if let Token::Identifier(name) = current.value() {
            self.advance();
            Ok(Spanned::new(name.to_string(), current.span()))
        } else {
            Err(Spanned::new(
                ParserError::SyntaxError("expected identifier".to_string()),
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
