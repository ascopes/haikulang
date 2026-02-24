use crate::error::ParserResult;
use crate::lexer::token::Token;
use crate::span::{Span, Spanned};
use logos::{Logos, SpannedIter};

pub struct TokenStream<'src> {
    iter: SpannedIter<'src, Token>,
    next: ParserResult<Token>,
}

impl<'src> TokenStream<'src> {
    pub fn new(source: &'src str) -> Self {
        let mut iter = Token::lexer(source).spanned();
        let next = Self::take_next(&mut iter);
        Self { iter, next }
    }

    pub fn advance(&mut self) {
        self.next = Self::take_next(&mut self.iter);
    }

    pub fn current(&mut self) -> ParserResult<Token> {
        self.next.clone()
    }

    fn take_next(iter: &mut SpannedIter<'src, Token>) -> ParserResult<Token> {
        let result = iter.next();
        match result {
            Some((result, span)) => {
                let generic_span = Span::new(span.start, span.end);
                match result {
                    Ok(token) => Ok(Spanned::new(token, generic_span)),
                    Err(error) => Err(Spanned::new(error, generic_span)),
                }
            }
            None => {
                let loc = iter.span().end;
                Ok(Spanned::new(Token::Eof, Span::new(loc, loc)))
            }
        }
    }
}
