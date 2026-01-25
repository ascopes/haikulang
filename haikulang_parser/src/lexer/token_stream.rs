use crate::lexer::{LexerError, Token};
use crate::span::{Span, Spanned};
use logos::{Logos, SpannedIter};

type LexerResult = Result<Spanned<Token>, Spanned<LexerError>>;

pub struct TokenStream<'src> {
    iter: SpannedIter<'src, Token>,
    peeked: Option<LexerResult>,
}

impl<'src> TokenStream<'src> {
    pub fn new(source: &'src str) -> Self {
        let iter = Token::lexer(source).spanned();
        Self { iter, peeked: None }
    }

    pub fn next(&mut self) -> LexerResult {
        self.peeked
            .take()
            .unwrap_or_else(|| Self::take_next(&mut self.iter))
    }

    pub fn peek(&mut self) -> LexerResult {
        if self.peeked.is_none() {
            let next = Self::take_next(&mut self.iter);
            self.peeked = Some(next);
        }
        self.peeked.clone().unwrap()
    }

    fn take_next(iter: &mut SpannedIter<'src, Token>) -> LexerResult {
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
