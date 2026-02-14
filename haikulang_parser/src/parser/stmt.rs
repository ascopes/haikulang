use crate::ast::stmt::*;
use crate::lexer::token::Token;
use crate::parser::parser::{Parser, ParserResult};
use crate::span::Spanned;

impl<'src> Parser<'src> {
    // statement ::= if_statement
    //             | while_statement
    //             | block_statement
    //             | use_statement , SEMICOLON
    //             | var_decl_statement , SEMICOLON
    //             | expr , SEMICOLON
    //             ;
    pub fn parse_statement(&mut self) -> ParserResult<Statement> {
        let first = self.current()?;
        match first.value() {
            Token::If => self.parse_if_statement(),
            Token::While => self.parse_while_statement(),
            Token::LeftBrace => self.parse_block_statement(),
            Token::Use => {
                let statement = self.parse_use_statement()?;
                self.eat(|token| token == Token::Semicolon, "semicolon")?;
                Ok(statement)
            }
            Token::Let => {
                let statement = self.parse_var_decl_statement()?;
                self.eat(|token| token == Token::Semicolon, "semicolon")?;
                Ok(statement)
            }
            _ => {
                let expr = self.parse_expr()?;
                let statement = ExprStatement::new(expr);
                self.eat(|token| token == Token::Semicolon, "semicolon")?;
                Ok(statement)
            }
        }
    }

    // if_statement ::= IF , LEFT_PAREN , expr , RIGHT_PAREN , statement , ( ELSE , statement )? ;
    fn parse_if_statement(&mut self) -> ParserResult<Statement> {
        let if_token = self.eat(|token| token == Token::If, "'if' keyword")?;
        self.eat(|token| token == Token::LeftParen, "left parenthesis")?;
        let condition = self.parse_expr()?;
        self.eat(|token| token == Token::RightParen, "right parenthesis")?;
        let body = self.parse_statement()?;

        let otherwise = if self.current()?.value() == Token::Else {
            self.advance();
            Some(self.parse_statement()?)
        } else {
            None
        };

        let statement = IfStatement::new(if_token.span(), condition, body, otherwise);
        Ok(statement)
    }

    // while_statement ::= WHILE , LEFT_PAREN , expr , RIGHT_PAREN , statement ;
    fn parse_while_statement(&mut self) -> ParserResult<Statement> {
        let while_token = self.eat(|token| token == Token::While, "'while' keyword")?;
        self.eat(|token| token == Token::LeftParen, "left parenthesis")?;
        let condition = self.parse_expr()?;
        self.eat(|token| token == Token::RightParen, "right parenthesis")?;
        let body = self.parse_statement()?;

        let statement = WhileStatement::new(while_token.span(), condition, body);
        Ok(statement)
    }

    // block_statement ::= LEFT_BRACE , statement* , RIGHT_BRACE ;
    fn parse_block_statement(&mut self) -> ParserResult<Statement> {
        let left_brace_token = self.eat(|token| token == Token::LeftBrace, "left brace")?;
        let mut statements = Vec::<Spanned<Statement>>::new();

        while self.current()?.value() != Token::RightBrace {
            statements.push(self.parse_statement()?);
        }

        let right_brace_token = self.eat(|token| token == Token::RightBrace, "right brace")?;
        let statement = BlockStatement::new(
            left_brace_token.span(),
            statements.into_boxed_slice(),
            right_brace_token.span(),
        );
        Ok(statement)
    }

    // use_statement ::= USE , IDENTIFIER ;
    fn parse_use_statement(&mut self) -> ParserResult<Statement> {
        let use_token = self.eat(|token| token == Token::Use, "'use' keyword")?;
        let identifier = self.eat_identifier()?;
        Ok(UseStatement::new(use_token.span(), identifier))
    }

    // var_decl_statement ::= LET , IDENTIFIER , ( EQ , expr )? ;
    fn parse_var_decl_statement(&mut self) -> ParserResult<Statement> {
        let let_token = self.eat(|token| token == Token::Let, "'let' keyword")?;
        let identifier = self.eat_identifier()?;

        let expr = if self.current()?.value() == Token::Assign {
            self.advance();
            Some(self.parse_expr()?)
        } else {
            None
        };
        Ok(VarDeclStatement::new(let_token.span(), identifier, expr))
    }
}
