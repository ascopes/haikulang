use crate::ast::unit::CompilationUnit;
use crate::lexer::token::Token;
use crate::lexer::token_stream::TokenStream;
use crate::parser::error::{ErrorReporter, ParserError};
use crate::span::{Span, Spanned};
use std::path::Path;

pub type ParserResult<T> = Result<Spanned<T>, ()>;

pub struct Parser<'src, 'err> {
    stream: TokenStream<'src>,
    path: &'src Path,
    error_reporter: &'err mut dyn ErrorReporter,
}

impl<'src, 'err> Parser<'src, 'err> {
    pub fn new(
        stream: TokenStream<'src>,
        path: &'src Path,
        error_reporter: &'err mut impl ErrorReporter,
    ) -> Self {
        Self {
            stream,
            path,
            error_reporter,
        }
    }

    pub fn parse(&mut self) -> ParserResult<CompilationUnit> {
        self.consume_comments();
        self.parse_compilation_unit(self.path)
    }

    // Report an error.
    #[inline]
    pub(super) fn report_error(&mut self, error: ParserError, span: Span) {
        self.error_reporter.report(error, span);
    }

    // Return a copy of the current token within the lexer.
    #[inline]
    pub(super) fn current(&mut self) -> ParserResult<Token> {
        match self.stream.current() {
            Ok(token) => Ok(token),
            Err(err) => {
                let span = err.span();
                let new_err = ParserError::LexerError(err.value());
                self.report_error(new_err, span);
                Err(())
            }
        }
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
            self.report_error(
                ParserError::SyntaxError(format!("expected {}", description)),
                current.span(),
            );
            Err(())
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
