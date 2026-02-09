use crate::ast::stmt::*;
use crate::lexer::token::Token;
use crate::parser::parser::{Parser, ParserResult};
use crate::span::Spanned;

impl<'src> Parser<'src> {
    // statement ::= empty_statement
    //             | var_decl_statement
    //             | if_statement
    //             | while_statement
    //             | block_statement
    //             | expr_statement
    //             ;
    pub fn parse_statement(&mut self) -> ParserResult<Statement> {
        let first = self.current()?;
        match first.value() {
            Token::Semicolon => self.parse_empty_statement(),
            Token::Let => self.parse_var_decl_statement(),
            Token::If => self.parse_if_statement(),
            Token::While => self.parse_while_statement(),
            Token::LeftBrace => self.parse_block_statement(),
            _ => self.parse_expr_statement(),
        }
    }

    // empty_statement ::= SEMICOLON ;
    fn parse_empty_statement(&mut self) -> ParserResult<Statement> {
        let token = self.eat(|token| matches!(token, Token::Semicolon), "semicolon")?;
        Ok(Spanned::new(Statement::Empty, token.span()))
    }

    fn parse_var_decl_statement(&mut self) -> ParserResult<Statement> {
        let first = self.eat(|token| matches!(token, Token::Let), "'let' keyword")?;
        todo!("not implemented");
    }

    fn parse_if_statement(&mut self) -> ParserResult<Statement> {
        let first = self.eat(|token| matches!(token, Token::If), "'if' keyword")?;
        todo!("not implemented");
    }

    fn parse_while_statement(&mut self) -> ParserResult<Statement> {
        let first = self.eat(|token| matches!(token, Token::While), "'while' keyword")?;
        todo!("not implemented");
    }

    fn parse_block_statement(&mut self) -> ParserResult<Statement> {
        let first = self.eat(|token| matches!(token, Token::LeftBrace), "left brace")?;
        todo!("not implemented");
    }

    // expr_statement ::= expr , SEMICOLON ;
    fn parse_expr_statement(&mut self) -> ParserResult<Statement> {
        let expr = self.parse_expr()?;
        let semi = self.eat(|token| matches!(token, Token::Semicolon), "semicolon")?;
        Ok(ExprStatement::new(expr, semi.span()))
    }
}
