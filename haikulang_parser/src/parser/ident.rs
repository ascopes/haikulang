use crate::ast::ident::{Identifier, TypeName};
use crate::lexer::token::Token;
use crate::parser::core::{Parser, ParserResult};
use crate::parser::error::ParserError;
use crate::span::Spanned;

impl<'src> Parser<'src> {
    // type_name ::= identifier , ( DOUBLE_COLON , identifier )* ;
    pub(super) fn parse_type_name(&mut self) -> ParserResult<TypeName> {
        let mut qualifier: Vec<Spanned<Identifier>> = Vec::new();
        let start = self.current()?.span();

        loop {
            let identifier = self.parse_identifier()?;

            if self.current()?.value() == Token::DoubleColon {
                qualifier.push(identifier);
                self.advance();
            } else {
                let span = start.to(identifier.span());
                let type_name = TypeName {
                    qualifier: Box::from(qualifier),
                    local_name: identifier,
                };
                return Ok(Spanned::new(type_name, span));
            }
        }
    }

    // identifier ::= IDENTIFIER ;
    pub(super) fn parse_identifier(&mut self) -> ParserResult<Identifier> {
        let current = self.current()?;
        if let Token::Identifier(name) = current.value() {
            self.advance();
            Ok(Spanned::new(
                Identifier::from_boxed_str(name),
                current.span(),
            ))
        } else {
            Err(Spanned::new(
                ParserError::SyntaxError("expected identifier".to_string()),
                current.span(),
            ))
        }
    }
}
