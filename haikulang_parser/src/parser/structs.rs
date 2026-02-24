use crate::ast::structs::{StructDecl, StructMemberDecl};
use crate::error::ParserResult;
use crate::lexer::token::Token;
use crate::parser::core::Parser;
use crate::span::Spanned;

impl<'src, 'err> Parser<'src, 'err> {
    // struct_decl ::= STRUCT , identifier , LEFT_BRACE , ( struct_member , ( COMMA , struct_member )
    pub(super) fn parse_struct_decl(&mut self) -> ParserResult<StructDecl> {
        let start = self.eat(Token::Struct, "'struct' keyword")?;
        let identifier = self.parse_identifier()?;
        let mut members: Vec<Spanned<StructMemberDecl>> = Vec::new();

        self.eat(Token::LeftBrace, "left brace")?;

        while self.current()?.value() != Token::RightBrace {
            members.push(self.parse_struct_member()?);
            self.eat(Token::Semicolon, "semicolon")?;
        }

        let end = self.eat(Token::RightBrace, "right brace")?;

        Ok(Spanned::new(
            StructDecl {
                identifier,
                members: Box::from(members),
            },
            start.span().to(end.span()),
        ))
    }

    // struct_member ::= identifier , COLON , identifier_path ;
    fn parse_struct_member(&mut self) -> ParserResult<StructMemberDecl> {
        let identifier = self.parse_identifier()?;
        self.eat(Token::Colon, "colon")?;
        let identifier_path = self.parse_identifier_path()?;

        let span = identifier.span().to(identifier_path.span());

        Ok(Spanned::new(
            StructMemberDecl {
                identifier,
                type_name: identifier_path,
            },
            span,
        ))
    }
}
