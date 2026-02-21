use crate::ast::func::*;
use crate::ast::ident::TypeName;
use crate::lexer::token::Token;
use crate::parser::core::{Parser, ParserResult};
use crate::span::Spanned;

impl<'src> Parser<'src> {
    // function_decl ::= FN , identifier , params , function_return_type? , block_statement     /* procedural function */
    //                 | FN , identifier , params , ASSIGN , expr_statement , semicolon         /* expression function */
    //                 ;
    pub(super) fn parse_function_decl(&mut self) -> ParserResult<Function> {
        let start = self.eat(|token| token == Token::Fn, "'fn' keyword")?;
        let name = self.parse_identifier()?;
        let parameters = self.parse_parameter_decls()?;

        // We have an expression function if we have an assignment symbol.
        if self.current()?.value() == Token::Assign {
            let body = self.parse_expr_statement()?;
            let end = self.eat(|token| token == Token::Semicolon, "semicolon")?;

            return Ok(Spanned::new(
                Function {
                    name,
                    parameters,
                    return_type: None,
                    body,
                },
                start.span().to(end.span()),
            ));
        }
        let return_type = if self.current()?.value() == Token::Arrow {
            Some(self.parse_function_return_type()?)
        } else {
            None
        };

        let body = self.parse_block_statement()?;
        let span = start.span().to(body.span());

        Ok(Spanned::new(
            Function {
                name,
                parameters,
                return_type,
                body,
            },
            span,
        ))
    }

    // function_return_type ::= ARROW , type_name ;
    fn parse_function_return_type(&mut self) -> ParserResult<TypeName> {
        self.eat(|token| token == Token::Arrow, "arrow")?;
        self.parse_type_name()
    }

    // parameter_decl_list ::= LEFT_PAREN , ( parameter_decl , ( COMMA , parameter_decl )* )?, RIGHT_PAREN ;
    fn parse_parameter_decls(&mut self) -> ParserResult<Box<[Spanned<ParameterDecl>]>> {
        let start = self.eat(|token| token == Token::LeftParen, "left parenthesis")?;

        let params = if self.current()?.value() != Token::RightParen {
            let mut params: Vec<Spanned<ParameterDecl>> = Vec::new();
            params.push(self.parse_parameter_decl()?);

            while self.current()?.value() == Token::Comma {
                self.advance();
                params.push(self.parse_parameter_decl()?);
            }

            params.into_boxed_slice()
        } else {
            Box::new([])
        };

        let end = self.eat(|token| token == Token::RightParen, "right parenthesis")?;

        Ok(Spanned::new(params, start.span().to(end.span())))
    }

    // parameter_decl ::= identifier , COLON , type_name ;
    fn parse_parameter_decl(&mut self) -> ParserResult<ParameterDecl> {
        let name = self.parse_identifier()?;
        self.eat(|token| token == Token::Colon, "colon")?;
        let type_name = self.parse_type_name()?;
        let span = name.span().to(type_name.span());
        Ok(Spanned::new(ParameterDecl { name, type_name }, span))
    }
}
