use crate::lexer::{Lexer, LexerResult, Token, TokenType};
use crate::parser::{
    AstNode, AstNodeKind, BinaryOperator, Literal, Parser, ParserError, ParserErrorKind,
    ParserResult, UnaryOperator,
};

pub(super) struct BasicParser<'a> {
    lexer: &'a mut dyn Lexer,
    current_token: LexerResult,
}

impl<'a> Parser for BasicParser<'a> {
    fn parse(&mut self) -> ParserResult {
        self.parse_expr()
    }
}

impl<'a> BasicParser<'a> {
    pub(super) fn new(lexer: &'a mut impl Lexer) -> Self {
        let current_token = lexer.next_token();
        BasicParser {
            lexer,
            current_token,
        }
    }

    // expr ::= additive_expr ;
    fn parse_expr(&mut self) -> ParserResult {
        self.parse_additive_expr()
    }



    // additive_expr ::= additive_expr , ADD , additive_expr
    //                 | additive_expr , SUB , additive_expr
    //                 | multiplicative_expr
    //                 ;
    fn parse_additive_expr(&mut self) -> ParserResult {
        let mut node = self.parse_multiplicative_expr()?;

        loop {
            let op = match self.peek()?.token_type {
                TokenType::Add => BinaryOperator::Add,
                TokenType::Sub => BinaryOperator::Sub,
                _ => break,
            };
            self.advance();

            let right = self.parse_multiplicative_expr()?;

            let location = node.location;

            node = AstNode {
                kind: AstNodeKind::BinaryOperator(Box::from(node), op, Box::from(right)),
                location,
            }
        }

        Ok(node)
    }

    // multiplicative_expr ::= multiplicative_expr , MUL , multiplicative_expr
    //                       | multiplicative_expr , DIV , multiplicative_expr
    //                       | multiplicative_expr , INT_DIV , multiplicative_expr
    //                       | multiplicative_expr , MOD , multiplicative_expr
    //                       | exponential_expr
    //                       ;
    fn parse_multiplicative_expr(&mut self) -> ParserResult {
        let mut node = self.parse_exponential_expr()?;

        loop {
            let op = match self.peek()?.token_type {
                TokenType::Mul => BinaryOperator::Mul,
                TokenType::Div => BinaryOperator::Div,
                TokenType::IntDiv => BinaryOperator::IntDiv,
                TokenType::Mod => BinaryOperator::Mod,
                _ => break,
            };
            self.advance();

            let right = self.parse_exponential_expr()?;

            let location = node.location;

            node = AstNode {
                kind: AstNodeKind::BinaryOperator(Box::from(node), op, Box::from(right)),
                location,
            }
        }

        Ok(node)
    }

    // exponential_expr ::= exponential_expr , POW , exponential_expr
    //                    | unary_expr
    //                    ;
    fn parse_exponential_expr(&mut self) -> ParserResult {
        let mut node = self.parse_unary_expr()?;

        loop {
            let op = match self.peek()?.token_type {
                TokenType::Pow => BinaryOperator::Pow,
                _ => break,
            };
            self.advance();

            let right = self.parse_unary_expr()?;

            let location = node.location;

            node = AstNode {
                kind: AstNodeKind::BinaryOperator(Box::from(node), op, Box::from(right)),
                location,
            }
        }

        Ok(node)
    }

    // unary_expr ::= ADD , unary_expr
    //              | SUB , unary_expr
    //              | NOT , unary_expr
    //              | atom
    //              ;
    fn parse_unary_expr(&mut self) -> ParserResult {
        let (location, token_type) = {
            let token = self.peek()?;
            (token.location, token.token_type.clone())
        };

        let op = match token_type {
            TokenType::Add => UnaryOperator::Pos,
            TokenType::Sub => UnaryOperator::Neg,
            TokenType::Not => UnaryOperator::Not,
            _ => return self.parse_atom(),
        };
        self.advance();

        let value = self.parse_unary_expr()?;

        Ok(AstNode {
            kind: AstNodeKind::UnaryOperator(op, Box::from(value)),
            location,
        })
    }

    // atom ::= IDENT
    //        | NUM_LIT
    //        | STR_LIT
    //        | LEFT_PAREN , expr , RIGHT_PAREN
    //        ;
    fn parse_atom(&mut self) -> ParserResult {
        let token = self.peek()?;

        match token.token_type {
            TokenType::LeftParen => {
                self.advance();
                let expr = self.parse_expr();

                match self.peek()?.token_type {
                    TokenType::RightParen => {
                        self.advance();
                        expr
                    }
                    _ => self.unexpected("expected right parenthesis"),
                }
            }
            TokenType::Ident => Ok(AstNode {
                kind: AstNodeKind::Identifier(token.raw.clone()),
                location: token.location,
            }),
            TokenType::FloatLit(f) => Ok(AstNode {
                kind: AstNodeKind::Literal(Literal::Float(f)),
                location: token.location,
            }),
            TokenType::IntLit(i) => Ok(AstNode {
                kind: AstNodeKind::Literal(Literal::Int(i)),
                location: token.location,
            }),
            TokenType::StrLit(ref s) => Ok(AstNode {
                kind: AstNodeKind::Literal(Literal::Str(s.clone())),
                location: token.location,
            }),
            _ => self.unexpected("expected nested expression, identifier, or literal value"),
        }
    }

    /////////////
    // Helpers //
    /////////////

    fn advance(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    fn peek(&self) -> Result<&Token, ParserError> {
        match &self.current_token {
            Ok(token) => Ok(token),
            Err(lexer_error) => Err(ParserError {
                location: lexer_error.start_location,
                kind: ParserErrorKind::LexerError(lexer_error.clone()),
            }),
        }
    }

    fn unexpected(&self, message: impl ToString) -> ParserResult {
        let token = self.peek()?.clone();
        let location = token.location;
        Err(ParserError {
            kind: ParserErrorKind::SyntaxError(token, message.to_string()),
            location,
        })
    }
}
