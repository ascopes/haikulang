use crate::ast::stmt::*;
use crate::lexer::token::Token;
use crate::parser::core::{Parser, ParserResult};
use crate::parser::error::ParserError::SyntaxError;
use crate::span::Spanned;

impl<'src> Parser<'src> {
    // statement ::= if_statement
    //             | while_statement
    //             | block_statement
    //             | use_statement , SEMICOLON
    //             | var_decl_statement , SEMICOLON
    //             | break_statement , SEMICOLON
    //             | continue_statement , SEMICOLON
    //             | return_statement , SEMICOLON
    //             | SEMICOLON
    //             | expr_statement , SEMICOLON  /* fallback */
    //             ;
    pub fn parse_statement(&mut self) -> ParserResult<Statement> {
        let first = self.current()?;
        match first.value() {
            Token::If => self.parse_if_statement(),
            Token::While => self.parse_while_statement(),
            Token::LeftBrace => self.parse_block_statement(),
            Token::Use => self.take_line_statement(Self::parse_use_statement),
            Token::Let => self.take_line_statement(Self::parse_var_decl_statement),
            Token::Break => self.take_line_statement(Self::parse_break_statement),
            Token::Continue => self.take_line_statement(Self::parse_continue_statement),
            Token::Return => self.take_line_statement(Self::parse_return_statement),
            Token::Semicolon => self.take_line_statement(Self::parse_empty_statement),
            _ => self.take_line_statement(Self::parse_expr_statement),
        }
    }

    // empty_statement ::= SEMICOLON ;
    fn parse_empty_statement(&mut self) -> ParserResult<Statement> {
        let token = self.eat(Token::Semicolon, "semicolon")?;
        Ok(Spanned::new(Statement::Empty, token.span()))
    }

    // if_statement ::= IF , LEFT_PAREN , expr , RIGHT_PAREN , statement , ( ELSE , statement )? ;
    fn parse_if_statement(&mut self) -> ParserResult<Statement> {
        let if_token = self.eat(Token::If, "'if' keyword")?;
        self.eat(Token::LeftParen, "left parenthesis")?;
        let condition = self.parse_expr()?;
        self.eat(Token::RightParen, "right parenthesis")?;
        let body = self.parse_statement()?;

        let otherwise = if self.current()?.value() == Token::Else {
            self.advance();
            Some(self.parse_statement()?)
        } else {
            None
        };

        let span = if let Some(spanned_otherwise) = &otherwise {
            if_token.span().to(spanned_otherwise.span())
        } else {
            if_token.span().to(body.span())
        };

        Ok(Spanned::new(
            Statement::If(Box::from(IfStatement {
                condition,
                body,
                otherwise,
            })),
            span,
        ))
    }

    // while_statement ::= WHILE , LEFT_PAREN , expr , RIGHT_PAREN , statement ;
    fn parse_while_statement(&mut self) -> ParserResult<Statement> {
        let while_token = self.eat(Token::While, "'while' keyword")?;
        self.eat(Token::LeftParen, "left parenthesis")?;
        let condition = self.parse_expr()?;
        self.eat(Token::RightParen, "right parenthesis")?;
        let body = self.parse_statement()?;
        let span = while_token.span().to(body.span());

        Ok(Spanned::new(
            Statement::While(Box::from(WhileStatement { condition, body })),
            span,
        ))
    }

    // block_statement ::= LEFT_BRACE , statement* , RIGHT_BRACE ;
    pub(super) fn parse_block_statement(&mut self) -> ParserResult<Statement> {
        let left_brace_token = self.eat(Token::LeftBrace, "left brace")?;
        let mut statements = Vec::<Spanned<Statement>>::new();

        while self.current()?.value() != Token::RightBrace {
            statements.push(self.parse_statement()?);
        }

        let right_brace_token = self.eat(Token::RightBrace, "right brace")?;
        let span = left_brace_token.span().to(right_brace_token.span());

        Ok(Spanned::new(
            Statement::Block(Box::from(BlockStatement {
                statements: statements.into_boxed_slice(),
            })),
            span,
        ))
    }

    // use_statement ::= USE , type_name ;
    fn parse_use_statement(&mut self) -> ParserResult<Statement> {
        let use_token = self.eat(Token::Use, "'use' keyword")?;
        let path = self.parse_type_name()?;
        let span = use_token.span().to(path.span());

        Ok(Spanned::new(
            Statement::Use(Box::from(UseStatement { path })),
            span,
        ))
    }

    // var_decl_statement ::= LET , identifier , COLON , type_name , ( ASSIGN , expr )?
    //                      | LET , identifier , ASSIGN , expr
    //                      ;
    fn parse_var_decl_statement(&mut self) -> ParserResult<Statement> {
        let let_token = self.eat(Token::Let, "'let' keyword")?;
        let identifier = self.parse_identifier()?;

        let (type_name, span) = if self.current()?.value() == Token::Colon {
            self.advance();
            let type_name = self.parse_type_name()?;
            let span = let_token.span().to(type_name.span());
            (Some(type_name), span)
        } else {
            let span = let_token.span().to(identifier.span());
            (None, span)
        };

        let (expr, span) = if self.current()?.value() == Token::Assign {
            self.advance();
            let expr = self.parse_expr()?;
            let span = let_token.span().to(expr.span());
            (Some(expr), span)
        } else {
            // Use the type name span, which might just be the identifier span if no type
            // declaration was present.
            let span = let_token.span().to(span);
            (None, span)
        };

        if type_name.is_none() && expr.is_none() {
            return Err(Spanned::new(
                SyntaxError(
                    "expected colon and type name or assignment in variable declaration"
                        .to_string(),
                ),
                let_token.span().to(self.current()?.span()),
            ));
        }

        Ok(Spanned::new(
            Statement::VarDecl(Box::from(VarDeclStatement {
                identifier,
                type_name,
                expr,
            })),
            span,
        ))
    }

    // break_statement ::= BREAK ;
    fn parse_break_statement(&mut self) -> ParserResult<Statement> {
        let break_token = self.eat(Token::Break, "'break' keyword")?;
        Ok(Spanned::new(Statement::Break, break_token.span()))
    }

    // continue_statement ::= CONTINUE ;
    fn parse_continue_statement(&mut self) -> ParserResult<Statement> {
        let continue_token = self.eat(Token::Continue, "'continue' keyword")?;
        Ok(Spanned::new(Statement::Continue, continue_token.span()))
    }

    // return_statement ::= RETURN , ( expr )? ;
    fn parse_return_statement(&mut self) -> ParserResult<Statement> {
        let return_token = self.eat(Token::Return, "'return' keyword")?;

        let (expr, span) = if self.current()?.value() != Token::Semicolon {
            let expr = self.parse_expr()?;
            let span = return_token.span().to(expr.span());
            (Some(expr), span)
        } else {
            (None, return_token.span())
        };

        Ok(Spanned::new(
            Statement::Return(Box::from(ReturnStatement { expr })),
            span,
        ))
    }

    // expr_statement ::= expr ;
    pub(super) fn parse_expr_statement(&mut self) -> ParserResult<Statement> {
        let expr = self.parse_expr()?;
        let span = expr.span();
        Ok(Spanned::new(
            Statement::Expr(Box::from(ExprStatement { expr })),
            span,
        ))
    }

    /*
     * Helpers
     */

    fn take_line_statement<F>(&mut self, func: F) -> ParserResult<Statement>
    where
        F: FnOnce(&mut Self) -> ParserResult<Statement>,
    {
        let statement = func(self)?;
        self.eat(Token::Semicolon, "semicolon")?;
        Ok(statement)
    }
}
