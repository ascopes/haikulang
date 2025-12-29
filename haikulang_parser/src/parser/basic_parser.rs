use crate::lexer::{Lexer, LexerResult, Token, TokenType};
use crate::location::Location;
use crate::parser::{
    AstNode, AstNodeKind, Literal, Operator, Parser, ParserError, ParserErrorKind, ParserResult,
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

    // additive_expr ::= multiplicative_expr , ADD , multiplicative_expr
    //                 | multiplicative_expr , SUB , multiplicative_expr
    //                 | multiplicative_expr
    //                 ;
    fn parse_additive_expr(&mut self) -> ParserResult {
        let left = self.parse_multiplicative_expr()?;

        let op = match self.peek()?.token_type {
            TokenType::Add => Operator::Add,
            TokenType::Sub => Operator::Sub,
            _ => return Ok(left),
        };
        self.advance();

        let right = self.parse_multiplicative_expr()?;

        self.node(
            left.location,
            AstNodeKind::BinaryOp(Box::from(left), op, Box::from(right)),
        )
    }

    // multiplicative_expr ::= exponential_expr , MUL , exponential_expr
    //                       | exponential_expr , DIV , exponential_expr
    //                       | exponential_expr , INT_DIV , exponential_expr
    //                       | exponential_expr , MOD , exponential_expr
    //                       | exponential_expr
    //                       ;
    fn parse_multiplicative_expr(&mut self) -> ParserResult {
        let left = self.parse_exponential_expr()?;

        let op = match self.peek()?.token_type {
            TokenType::Mul => Operator::Mul,
            TokenType::Div => Operator::Div,
            TokenType::IntDiv => Operator::IntDiv,
            TokenType::Mod => Operator::Mod,
            _ => return Ok(left),
        };
        self.advance();

        let right = self.parse_exponential_expr()?;

        self.node(
            left.location,
            AstNodeKind::BinaryOp(Box::from(left), op, Box::from(right)),
        )
    }

    // exponential_expr ::= atom , POW , atom
    //                    | atom
    //                    ;
    fn parse_exponential_expr(&mut self) -> ParserResult {
        let left = self.parse_atom()?;

        let op = match self.peek()?.token_type {
            TokenType::Pow => Operator::Pow,
            _ => return Ok(left),
        };
        self.advance();

        let right = self.parse_atom()?;

        self.node(
            left.location,
            AstNodeKind::BinaryOp(Box::from(left), op, Box::from(right)),
        )
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
            TokenType::Ident => {
                self.node(token.location, AstNodeKind::Identifier(token.raw.clone()))
            }
            TokenType::FloatLit(f) => self.literal(token.location, Literal::Float(f)),
            TokenType::IntLit(i) => self.literal(token.location, Literal::Int(i)),
            TokenType::StrLit(ref s) => self.literal(token.location, Literal::Str(s.clone())),
            _ => self.unexpected("expected nested expression, identifier, or literal value"),
        }
    }

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

    fn node(&self, location: Location, kind: AstNodeKind) -> ParserResult {
        Ok(AstNode { kind, location })
    }

    fn literal(&self, location: Location, kind: Literal) -> ParserResult {
        Ok(AstNode {
            kind: AstNodeKind::Literal(kind),
            location,
        })
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
