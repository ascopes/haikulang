use crate::lexer::token::Token;
use crate::lexer::token_stream::TokenStream;
use crate::parser::error::ParserError;
use crate::span::Spanned;

pub type ParserResult<T> = Result<Spanned<T>, Spanned<ParserError>>;

pub struct Parser<'src> {
    stream: TokenStream<'src>,
}

//noinspection DuplicatedCode
impl<'src> Parser<'src> {
    pub fn new(stream: TokenStream<'src>) -> Self {
        Self { stream }
    }

    #[inline]
    pub(super) fn current(&mut self) -> Result<Spanned<Token>, Spanned<ParserError>> {
        self.stream.current().map_err(|err| {
            let new_err = ParserError::LexerError(err.value().clone());
            Spanned::new(new_err, err.span())
        })
    }

    #[inline]
    pub(super) fn advance(&mut self) {
        self.stream.advance();
    }
}
