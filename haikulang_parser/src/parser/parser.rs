use crate::lexer::{Token, TokenStream};
use crate::parser::error::ParserError;
use crate::parser::{AstNode, BinaryOp, UnaryOp};
use crate::span::{Span, Spanned};

pub type ParserResult = Result<Spanned<AstNode>, Spanned<ParserError>>;

pub struct Parser<'src> {
    stream: TokenStream<'src>,
}

//noinspection DuplicatedCode
impl<'src> Parser<'src> {
    pub fn new(stream: TokenStream<'src>) -> Self {
        Self { stream }
    }

    // FIXME(ascopes): consolidate expression parsing into a Pratt parser.
    //  This should be slightly less verbose, remove code duplication, and should
    //  help avoid stack overflows on heavily nested expressions.
    #[allow(dead_code)]
    pub fn parse_expr(&mut self) -> ParserResult {
        self.parse_bool_or_expr()
    }

    // bool_or_expr ::= bool_and_expr , BINARY_OR , bool_or_expr
    //                | bool_and_expr
    //                ;
    fn parse_bool_or_expr(&mut self) -> ParserResult {
        let mut left = self.parse_bool_and_expr()?;

        loop {
            let op = match self.current()?.value() {
                Token::BoolOr => BinaryOp::BoolOr,
                _ => break,
            };
            self.advance();

            let right = self.parse_bool_and_expr()?;
            let span = left.span().to(right.span());
            let node = AstNode::BinaryOp(Box::from(left), op, Box::from(right));
            left = Spanned::new(node, span);
        }

        Ok(left)
    }

    // bool_and_expr ::= binary_or_expr , AND , bool_and_expr
    //                 | binary_or_expr
    //                 ;
    fn parse_bool_and_expr(&mut self) -> ParserResult {
        let mut left = self.parse_binary_or_expr()?;

        loop {
            let op = match self.current()?.value() {
                Token::BoolAnd => BinaryOp::BoolAnd,
                _ => break,
            };
            self.advance();

            let right = self.parse_binary_or_expr()?;
            let span = left.span().to(right.span());
            let node = AstNode::BinaryOp(Box::from(left), op, Box::from(right));
            left = Spanned::new(node, span);
        }

        Ok(left)
    }

    // binary_or_expr ::= binary_xor_expr , BINARY_OR , binary_xor_expr
    //                  | binary_xor_expr
    //                  ;
    fn parse_binary_or_expr(&mut self) -> ParserResult {
        let mut left = self.parse_binary_xor_expr()?;

        loop {
            let op = match self.current()?.value() {
                Token::BinaryOr => BinaryOp::BinaryOr,
                _ => break,
            };
            self.advance();

            let right = self.parse_binary_xor_expr()?;
            let span = left.span().to(right.span());
            let node = AstNode::BinaryOp(Box::from(left), op, Box::from(right));
            left = Spanned::new(node, span);
        }

        Ok(left)
    }

    // binary_xor_expr ::= binary_and_expr , BINARY_XOR , binary_xor_expr
    //                   | binary_and_expr
    //                   ;
    fn parse_binary_xor_expr(&mut self) -> ParserResult {
        let mut left = self.parse_binary_and_expr()?;

        loop {
            let op = match self.current()?.value() {
                Token::BinaryXor => BinaryOp::BinaryXor,
                _ => break,
            };
            self.advance();

            let right = self.parse_binary_and_expr()?;
            let span = left.span().to(right.span());
            let node = AstNode::BinaryOp(Box::from(left), op, Box::from(right));
            left = Spanned::new(node, span);
        }

        Ok(left)
    }

    // binary_and_expr ::= equality_expr , AND , binary_and_expr
    //                   | equality_expr
    //                   ;
    fn parse_binary_and_expr(&mut self) -> ParserResult {
        let mut left = self.parse_equality_expr()?;

        loop {
            let op = match self.current()?.value() {
                Token::BinaryAnd => BinaryOp::BinaryAnd,
                _ => break,
            };
            self.advance();

            let right = self.parse_equality_expr()?;
            let span = left.span().to(right.span());
            let node = AstNode::BinaryOp(Box::from(left), op, Box::from(right));
            left = Spanned::new(node, span);
        }

        Ok(left)
    }

    // equality_expr ::= compare_expr , EQ , equality_expr
    //                 | compare_expr , NOT_EQ , equality_expr
    //                 ;
    fn parse_equality_expr(&mut self) -> ParserResult {
        let mut left = self.parse_compare_expr()?;

        loop {
            let op = match self.current()?.value() {
                Token::Eq => BinaryOp::Eq,
                Token::NotEq => BinaryOp::NotEq,
                _ => break,
            };
            self.advance();

            let right = self.parse_compare_expr()?;
            let span = left.span().to(right.span());
            let node = AstNode::BinaryOp(Box::from(left), op, Box::from(right));
            left = Spanned::new(node, span);
        }

        Ok(left)
    }

    // compare_expr ::= binary_shift_expr , LESS , compare_expr
    //                | binary_shift_expr , LESS_EQ , compare_expr
    //                | binary_shift_expr , GREATER , compare_expr
    //                | binary_shift_expr , GREATER_EQ , compare_expr
    //                | binary_shift_expr
    //                ;
    fn parse_compare_expr(&mut self) -> ParserResult {
        let mut left = self.parse_binary_shift_expr()?;

        loop {
            let op = match self.current()?.value() {
                Token::Less => BinaryOp::Less,
                Token::LessEq => BinaryOp::LessEq,
                Token::Greater => BinaryOp::Greater,
                Token::GreaterEq => BinaryOp::GreaterEq,
                _ => break,
            };
            self.advance();

            let right = self.parse_binary_shift_expr()?;
            let span = left.span().to(right.span());
            let node = AstNode::BinaryOp(Box::from(left), op, Box::from(right));
            left = Spanned::new(node, span);
        }

        Ok(left)
    }

    // binary_shift_expr ::= sum_expr , BINARY_SHL , binary_shift_expr
    //                     | sum_expr , BINARY_SHR , binary_shift_expr
    //                     | sum_expr
    //                     ;
    fn parse_binary_shift_expr(&mut self) -> ParserResult {
        let mut left = self.parse_sum_expr()?;

        loop {
            let op = match self.current()?.value() {
                Token::BinaryShl => BinaryOp::BinaryShl,
                Token::BinaryShr => BinaryOp::BinaryShr,
                _ => break,
            };
            self.advance();

            let right = self.parse_sum_expr()?;
            let span = left.span().to(right.span());
            let node = AstNode::BinaryOp(Box::from(left), op, Box::from(right));
            left = Spanned::new(node, span);
        }

        Ok(left)
    }

    // sum_expr ::= factor_expr , ADD , sum_expr
    //            | factor_expr , SUB , sum_expr
    //            | factor_expr
    //            ;
    fn parse_sum_expr(&mut self) -> ParserResult {
        let mut left = self.parse_factor_expr()?;

        loop {
            let op = match self.current()?.value() {
                Token::Add => BinaryOp::Add,
                Token::Sub => BinaryOp::Sub,
                _ => break,
            };
            self.advance();

            let right = self.parse_factor_expr()?;
            let span = left.span().to(right.span());
            let node = AstNode::BinaryOp(Box::from(left), op, Box::from(right));
            left = Spanned::new(node, span);
        }

        Ok(left)
    }

    // factor_expr ::= pow_expr , MUL , factor_expr
    //               | pow_expr , DIV , factor_expr
    //               | pow_expr , MOD , factor_expr
    //               | pow_expr
    //               ;
    fn parse_factor_expr(&mut self) -> ParserResult {
        let mut left = self.parse_pow_expr()?;

        loop {
            let op = match self.current()?.value() {
                Token::Mul => BinaryOp::Mul,
                Token::Div => BinaryOp::Div,
                Token::Mod => BinaryOp::Mod,
                _ => break,
            };
            self.advance();

            let right = self.parse_pow_expr()?;
            let span = left.span().to(right.span());
            let node = AstNode::BinaryOp(Box::from(left), op, Box::from(right));
            left = Spanned::new(node, span);
        }

        Ok(left)
    }

    // pow_expr ::= unary_expr , POW , pow_expr
    //            | unary_expr
    //            ;
    fn parse_pow_expr(&mut self) -> ParserResult {
        let left = self.parse_unary_expr()?;

        let op = match self.current()?.value() {
            Token::Pow => BinaryOp::Pow,
            _ => return Ok(left),
        };
        self.advance();

        // Purposely recursive here, to force right-associativity.
        let right = self.parse_pow_expr()?;
        let span = left.span().to(right.span());
        let node = AstNode::BinaryOp(Box::from(left), op, Box::from(right));
        Ok(Spanned::new(node, span))
    }

    // unary_expr ::= ADD , unary_expr
    //              | SUB , unary_expr
    //              | BINARY_NOT , unary_expr
    //              | atom
    //              ;
    fn parse_unary_expr(&mut self) -> ParserResult {
        let first = self.current()?;
        let op = match first.value() {
            Token::Add => UnaryOp::Plus,
            Token::Sub => UnaryOp::Minus,
            Token::BinaryNot => UnaryOp::Invert,
            _ => return self.parse_atom(),
        };
        self.advance();

        let value = self.parse_unary_expr()?;
        let span = first.span().to(value.span());
        let node = AstNode::UnaryOp(op, Box::from(value));
        wrap(node, span)
    }

    // atom ::= IDENTIFIER
    //        | TRUE
    //        | FALSE
    //        | INT_LIT
    //        | FLOAT_LIT
    //        | STRING_LIT
    //        | LEFT_PAREN , expr , RIGHT_PAREN
    //        ;
    fn parse_atom(&mut self) -> ParserResult {
        let first = self.current()?;

        if matches!(first.value(), Token::LeftParen) {
            self.advance();

            let expr = self.parse_expr()?;
            let last = self.current()?;

            return match last.value() {
                Token::RightParen => {
                    self.advance();
                    Ok(expr)
                }
                _ => syntax_error(last, "expected right parenthesis"),
            };
        }

        let atom = match first.value() {
            Token::True => wrap(AstNode::Bool(true), first.span()),
            Token::False => wrap(AstNode::Bool(false), first.span()),
            Token::IntLit(value) => wrap(AstNode::Int(value.clone()), first.span()),
            Token::FloatLit(value) => wrap(AstNode::Float(value.clone()), first.span()),
            Token::StringLit(value) => wrap(AstNode::String(value.clone()), first.span()),
            Token::Identifier(value) => wrap(AstNode::Var(value), first.span()),
            _ => {
                return syntax_error(
                    first,
                    "expected atom (literal, identifier, or expression within parenthesis)",
                );
            }
        };

        self.advance();
        atom
    }

    #[inline]
    fn current(&mut self) -> Result<Spanned<Token>, Spanned<ParserError>> {
        self.stream.current().map_err(|err| {
            let new_err = ParserError::LexerError(err.value().clone());
            Spanned::new(new_err, err.span())
        })
    }

    #[inline]
    fn advance(&mut self) {
        self.stream.advance();
    }
}

#[inline]
fn wrap(node: AstNode, span: Span) -> ParserResult {
    Ok(Spanned::new(node, span))
}

fn syntax_error(token: Spanned<Token>, message: impl ToString) -> ParserResult {
    let err = ParserError::SyntaxError(token.value(), message.to_string());
    Err(Spanned::new(err, token.span()))
}
