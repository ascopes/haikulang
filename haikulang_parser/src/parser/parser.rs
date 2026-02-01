use crate::lexer::token::Token;
use crate::lexer::token_stream::TokenStream;
use crate::parser::ast::*;
use crate::parser::error::ParserError;
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
        self.parse_assignment_expr()
    }

    // assignment_expr ::= bool_or_expr , ASSIGN , assignment_expr
    //                   | bool_or_expr , ADD_ASSIGN , assignment_expr
    //                   | bool_or_expr , SUB_ASSIGN , assignment_expr
    //                   | bool_or_expr , MUL_ASSIGN , assignment_expr
    //                   | bool_or_expr , DIV_ASSIGN , assignment_expr
    //                   | bool_or_expr , MOD_ASSIGN , assignment_expr
    //                   | bool_or_expr , POW_ASSIGN , assignment_expr
    //                   | bool_or_expr , BINARY_AND_ASSIGN , assignment_expr
    //                   | bool_or_expr , BINARY_OR_ASSIGN , assignment_expr
    //                   | bool_or_expr , BINARY_XOR_ASSIGN , assignment_expr
    //                   | bool_or_expr , BINARY_SHL_ASSIGN , assignment_expr
    //                   | bool_or_expr , BINARY_SHR_ASSIGN , assignment_expr
    //                   | bool_or_expr
    //                   ;
    fn parse_assignment_expr(&mut self) -> ParserResult {
        let left = self.parse_bool_or_expr()?;

        let op = match self.current()?.value() {
            Token::Assign => None,
            Token::AddAssign => Some(BinaryOp::Add),
            Token::SubAssign => Some(BinaryOp::Sub),
            Token::MulAssign => Some(BinaryOp::Mul),
            Token::DivAssign => Some(BinaryOp::Div),
            Token::ModAssign => Some(BinaryOp::Mod),
            Token::PowAssign => Some(BinaryOp::Pow),
            Token::BinaryAndAssign => Some(BinaryOp::BinaryAnd),
            Token::BinaryOrAssign => Some(BinaryOp::BinaryOr),
            Token::BinaryXorAssign => Some(BinaryOp::BinaryXor),
            Token::BinaryShlAssign => Some(BinaryOp::BinaryShl),
            Token::BinaryShrAssign => Some(BinaryOp::BinaryShr),
            _ => return Ok(left),
        };
        self.advance();

        // Verify lvalue is assignable
        if !matches!(left.value(), AstNode::Var(_)) {
            return syntax_error(
                left.span(),
                "only identifiers can be used as lvalues in assignments",
            );
        }

        // Purposely recursive here, to force right-associativity.
        // `x = y = z = a` is parsed as `(x = (y = (z = a)))`
        let right = self.parse_assignment_expr()?;
        Ok(AssignmentExpr::new(left, op, right))
    }

    // bool_or_expr ::= bool_and_expr , BINARY_OR , bool_or_expr
    //                | bool_and_expr
    //                ;
    fn parse_bool_or_expr(&mut self) -> ParserResult {
        self.parse_binary_op_left_assoc(
            |token| match token {
                Token::BoolOr => Some(BinaryOp::BoolOr),
                _ => None,
            },
            Self::parse_bool_and_expr,
        )
    }

    // bool_and_expr ::= binary_or_expr , AND , bool_and_expr
    //                 | binary_or_expr
    //                 ;
    fn parse_bool_and_expr(&mut self) -> ParserResult {
        self.parse_binary_op_left_assoc(
            |token| match token {
                Token::BoolAnd => Some(BinaryOp::BoolAnd),
                _ => None,
            },
            Self::parse_binary_or_expr,
        )
    }

    // binary_or_expr ::= binary_xor_expr , BINARY_OR , binary_xor_expr
    //                  | binary_xor_expr
    //                  ;
    fn parse_binary_or_expr(&mut self) -> ParserResult {
        self.parse_binary_op_left_assoc(
            |token| match token {
                Token::BinaryOr => Some(BinaryOp::BinaryOr),
                _ => None,
            },
            Self::parse_binary_xor_expr,
        )
    }

    // binary_xor_expr ::= binary_and_expr , BINARY_XOR , binary_xor_expr
    //                   | binary_and_expr
    //                   ;
    fn parse_binary_xor_expr(&mut self) -> ParserResult {
        self.parse_binary_op_left_assoc(
            |token| match token {
                Token::BinaryXor => Some(BinaryOp::BinaryXor),
                _ => None,
            },
            Self::parse_binary_and_expr,
        )
    }

    // binary_and_expr ::= equality_expr , AND , binary_and_expr
    //                   | equality_expr
    //                   ;
    fn parse_binary_and_expr(&mut self) -> ParserResult {
        self.parse_binary_op_left_assoc(
            |token| match token {
                Token::BinaryAnd => Some(BinaryOp::BinaryAnd),
                _ => None,
            },
            Self::parse_equality_expr,
        )
    }

    // equality_expr ::= compare_expr , EQ , equality_expr
    //                 | compare_expr , NOT_EQ , equality_expr
    //                 ;
    fn parse_equality_expr(&mut self) -> ParserResult {
        self.parse_binary_op_left_assoc(
            |token| match token {
                Token::Eq => Some(BinaryOp::Eq),
                Token::NotEq => Some(BinaryOp::NotEq),
                _ => None,
            },
            Self::parse_compare_expr,
        )
    }

    // compare_expr ::= binary_shift_expr , LESS , compare_expr
    //                | binary_shift_expr , LESS_EQ , compare_expr
    //                | binary_shift_expr , GREATER , compare_expr
    //                | binary_shift_expr , GREATER_EQ , compare_expr
    //                | binary_shift_expr
    //                ;
    fn parse_compare_expr(&mut self) -> ParserResult {
        self.parse_binary_op_left_assoc(
            |token| match token {
                Token::Less => Some(BinaryOp::Less),
                Token::LessEq => Some(BinaryOp::LessEq),
                Token::Greater => Some(BinaryOp::Greater),
                Token::GreaterEq => Some(BinaryOp::GreaterEq),
                _ => None,
            },
            Self::parse_binary_shift_expr,
        )
    }

    // binary_shift_expr ::= sum_expr , BINARY_SHL , binary_shift_expr
    //                     | sum_expr , BINARY_SHR , binary_shift_expr
    //                     | sum_expr
    //                     ;
    fn parse_binary_shift_expr(&mut self) -> ParserResult {
        self.parse_binary_op_left_assoc(
            |token| match token {
                Token::BinaryShl => Some(BinaryOp::BinaryShl),
                Token::BinaryShr => Some(BinaryOp::BinaryShr),
                _ => None,
            },
            Self::parse_sum_expr,
        )
    }

    // sum_expr ::= factor_expr , ADD , sum_expr
    //            | factor_expr , SUB , sum_expr
    //            | factor_expr
    //            ;
    fn parse_sum_expr(&mut self) -> ParserResult {
        self.parse_binary_op_left_assoc(
            |token| match token {
                Token::Add => Some(BinaryOp::Add),
                Token::Sub => Some(BinaryOp::Sub),
                _ => None,
            },
            Self::parse_factor_expr,
        )
    }

    // factor_expr ::= pow_expr , MUL , factor_expr
    //               | pow_expr , DIV , factor_expr
    //               | pow_expr , MOD , factor_expr
    //               | pow_expr
    //               ;
    fn parse_factor_expr(&mut self) -> ParserResult {
        self.parse_binary_op_left_assoc(
            |token| match token {
                Token::Mul => Some(BinaryOp::Mul),
                Token::Div => Some(BinaryOp::Div),
                Token::Mod => Some(BinaryOp::Mod),
                _ => None,
            },
            Self::parse_unary_expr,
        )
    }

    // unary_expr ::= ADD , unary_expr
    //              | SUB , unary_expr
    //              | BINARY_NOT , unary_expr
    //              | BOOL_NOT , unary_expr
    //              | pow_expr
    //              ;
    fn parse_unary_expr(&mut self) -> ParserResult {
        let first = self.current()?;
        let op = match first.value() {
            Token::Add => UnaryOp::Plus,
            Token::Sub => UnaryOp::Minus,
            Token::BinaryNot => UnaryOp::Invert,
            Token::BoolNot => UnaryOp::Not,
            _ => return self.parse_pow_expr(),
        };
        self.advance();

        let value = self.parse_unary_expr()?;
        let span = first.span().to(value.span());
        Ok(UnaryExpr::new(span, op, value))
    }

    // pow_expr ::= atom , POW , pow_expr
    //            | atom
    //            ;
    fn parse_pow_expr(&mut self) -> ParserResult {
        let left = self.parse_atom()?;

        let op = match self.current()?.value() {
            Token::Pow => BinaryOp::Pow,
            _ => return Ok(left),
        };
        self.advance();

        // Purposely recursive here, to force right-associativity.
        // In maths, we always say `x ** y ** z` is `(x ** (y ** z))`. This differs to
        // most of the expr grammar here, so we treat it as an edge case and do not
        // wrap it in a utility handler helper.
        let right = self.parse_pow_expr()?;
        Ok(BinaryExpr::new(left, op, right))
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
                _ => syntax_error(first.span(), "expected right parenthesis"),
            };
        }

        let atom = match first.value() {
            Token::True => Spanned::new(AstNode::Bool(true), first.span()),
            Token::False => Spanned::new(AstNode::Bool(false), first.span()),
            Token::IntLit(value) => Spanned::new(AstNode::Int(value.clone()), first.span()),
            Token::FloatLit(value) => Spanned::new(AstNode::Float(value.clone()), first.span()),
            Token::StringLit(value) => Spanned::new(AstNode::String(value), first.span()),
            Token::Identifier(value) => Spanned::new(AstNode::Var(value), first.span()),
            _ => {
                return syntax_error(
                    first.span(),
                    "expected atom (literal, identifier, or expression within parenthesis)",
                );
            }
        };

        self.advance();
        Ok(atom)
    }

    fn parse_binary_op_left_assoc<OpFn, ParserFn>(
        &mut self,
        op_fn: OpFn,
        parser_fn: ParserFn,
    ) -> ParserResult
    where
        OpFn: Fn(Token) -> Option<BinaryOp>,
        ParserFn: Fn(&mut Self) -> ParserResult,
    {
        let mut left = parser_fn(self)?;

        loop {
            if let Some(op) = op_fn(self.current()?.value()) {
                self.advance();
                let right = parser_fn(self)?;

                left = BinaryExpr::new(left, op, right);
            } else {
                break;
            };
        }

        Ok(left)
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

fn syntax_error(span: Span, message: impl ToString) -> ParserResult {
    let err = ParserError::SyntaxError(message.to_string());
    Err(Spanned::new(err, span))
}
