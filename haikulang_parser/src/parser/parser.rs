use crate::lexer::{Token, TokenStream};
use crate::parser::AstNode;
use crate::parser::error::ParserError;
use crate::span::Spanned;

pub type ParserResult = Result<Spanned<AstNode>, Spanned<ParserError>>;

pub struct Parser<'src> {
    stream: TokenStream<'src>,
}

impl<'src> Parser<'src> {
    pub fn new(stream: TokenStream<'src>) -> Self {
        Self { stream }
    }

    fn parse_expr(&mut self) -> ParserResult {
        self.parse_atom()
    }

    fn parse_atom(&mut self) -> ParserResult {
        let next = self.peek()?;

        match next.value() {
            Token::True => Ok(Spanned::new(AstNode::Bool(true), next.span())),
            Token::False => Ok(Spanned::new(AstNode::Bool(false), next.span())),
            Token::IntLit(value) => Ok(Spanned::new(AstNode::Int(value.clone()), next.span())),
            Token::FloatLit(value) => Ok(Spanned::new(AstNode::Float(value.clone()), next.span())),
            Token::StringLit(value) => {
                Ok(Spanned::new(AstNode::String(value.clone()), next.span()))
            }
            Token::Identifier(value) => Ok(Spanned::new(AstNode::Var(value), next.span())),
            _ => todo!(),
        }
    }

    fn peek(&mut self) -> Result<Spanned<Token>, Spanned<ParserError>> {
        self.stream.peek().map_err(|err| {
            let new_err = ParserError::LexerError(err.value().clone());
            Spanned::new(new_err, err.span())
        })
    }

    fn advance(&mut self) -> Result<(), Spanned<ParserError>> {
        self.stream.next().map(|_| ()).map_err(|err| {
            let new_err = ParserError::LexerError(err.value().clone());
            Spanned::new(new_err, err.span())
        })
    }
}
