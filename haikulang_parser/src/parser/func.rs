use crate::ast::func::*;
use crate::ast::ident::IdentifierPath;
use crate::error::ParserResult;
use crate::lexer::token::Token;
use crate::parser::core::Parser;
use crate::span::Spanned;

impl<'src, 'err> Parser<'src, 'err> {
    // extern_function_decl ::= FN , identifier , params , function_return_type? ;
    pub(super) fn parse_extern_function_decl(&mut self) -> ParserResult<ExternFunctionDecl> {
        let start = self.eat(Token::Extern, "'extern' keyword")?;
        self.eat(Token::Fn, "'fn' keyword")?;
        let name = self.parse_identifier()?;
        let parameters = self.parse_parameter_decls()?;

        let (return_type, end_span) = if self.current()?.value() == Token::Arrow {
            let return_type = self.parse_function_return_type()?;
            let span = return_type.span();
            (Some(return_type), span)
        } else {
            (None, parameters.span())
        };

        let span = start.span().to(end_span);

        Ok(Spanned::new(
            ExternFunctionDecl {
                name,
                parameters,
                return_type,
            },
            span,
        ))
    }

    // function_decl ::= FN , identifier , params , function_return_type? , block_statement     /* procedural function */
    //                 | FN , identifier , params , ASSIGN , expr_statement , semicolon         /* expression function */
    //                 ;
    pub(super) fn parse_function_decl(&mut self) -> ParserResult<FunctionDecl> {
        let start = self.eat(Token::Fn, "'fn' keyword")?;
        let name = self.parse_identifier()?;
        let parameters = self.parse_parameter_decls()?;

        // We have an expression function if we have an assignment symbol.
        if self.current()?.value() == Token::Assign {
            let body = self.parse_expr_statement()?;
            let end = self.eat(Token::Semicolon, "semicolon")?;

            return Ok(Spanned::new(
                FunctionDecl {
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
            FunctionDecl {
                name,
                parameters,
                return_type,
                body,
            },
            span,
        ))
    }

    // function_return_type ::= ARROW , identifier_path ;
    fn parse_function_return_type(&mut self) -> ParserResult<IdentifierPath> {
        self.eat(Token::Arrow, "arrow")?;
        self.parse_identifier_path()
    }

    // parameter_decl_list ::= LEFT_PAREN , ( parameter_decl , ( COMMA , parameter_decl )* )?, RIGHT_PAREN ;
    fn parse_parameter_decls(&mut self) -> ParserResult<Box<[Spanned<ParameterDecl>]>> {
        let start = self.eat(Token::LeftParen, "left parenthesis")?;

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

        let end = self.eat(Token::RightParen, "right parenthesis")?;

        Ok(Spanned::new(params, start.span().to(end.span())))
    }

    // parameter_decl ::= identifier , COLON , identifier_path ;
    fn parse_parameter_decl(&mut self) -> ParserResult<ParameterDecl> {
        let name = self.parse_identifier()?;
        self.eat(Token::Colon, "colon")?;
        let identifier_path = self.parse_identifier_path()?;
        let span = name.span().to(identifier_path.span());
        Ok(Spanned::new(
            ParameterDecl {
                name,
                type_name: identifier_path,
            },
            span,
        ))
    }
}
