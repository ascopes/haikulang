use crate::ast::expr::*;
use crate::debug_assert_matches;
use crate::error::{ParserError, ParserResult};
use crate::lexer::token::Token;
use crate::parser::core::Parser;
use crate::span::Spanned;

//noinspection DuplicatedCode
impl<'src, 'err> Parser<'src, 'err> {
    // FIXME(ascopes): consolidate expression parsing into a Pratt parser.
    //  This should be slightly less verbose, remove code duplication, and should
    //  help avoid stack overflows on heavily nested expressions.
    #[allow(dead_code)]
    pub(super) fn parse_expr(&mut self) -> ParserResult<Expr> {
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
    fn parse_assignment_expr(&mut self) -> ParserResult<Expr> {
        let lvalue = self.parse_bool_or_expr()?;

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
            _ => return Ok(lvalue),
        };
        self.advance();

        // Verify lvalue is assignable.
        // Purposely recursive here, to force right-associativity.
        // `x = y = z = a` is parsed as `(x = (y = (z = a)))`
        let rvalue = self.parse_assignment_expr()?;

        let span = lvalue.span().to(rvalue.span());

        Ok(Spanned::new(
            Expr::Assignment(Box::new(AssignmentExpr { lvalue, op, rvalue })),
            span,
        ))
    }

    // bool_or_expr ::= bool_and_expr , BINARY_OR , bool_or_expr
    //                | bool_and_expr
    //                ;
    fn parse_bool_or_expr(&mut self) -> ParserResult<Expr> {
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
    fn parse_bool_and_expr(&mut self) -> ParserResult<Expr> {
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
    fn parse_binary_or_expr(&mut self) -> ParserResult<Expr> {
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
    fn parse_binary_xor_expr(&mut self) -> ParserResult<Expr> {
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
    fn parse_binary_and_expr(&mut self) -> ParserResult<Expr> {
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
    fn parse_equality_expr(&mut self) -> ParserResult<Expr> {
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
    fn parse_compare_expr(&mut self) -> ParserResult<Expr> {
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
    fn parse_binary_shift_expr(&mut self) -> ParserResult<Expr> {
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
    fn parse_sum_expr(&mut self) -> ParserResult<Expr> {
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
    fn parse_factor_expr(&mut self) -> ParserResult<Expr> {
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
    fn parse_unary_expr(&mut self) -> ParserResult<Expr> {
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

        Ok(Spanned::new(
            Expr::Unary(Box::new(UnaryExpr { op, value })),
            span,
        ))
    }

    // pow_expr ::= primary_expr , POW , pow_expr
    //            | primary_expr
    //            ;
    fn parse_pow_expr(&mut self) -> ParserResult<Expr> {
        let left = self.parse_primary_expr()?;

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

        let span = left.span().to(right.span());

        Ok(Spanned::new(
            Expr::Binary(Box::new(BinaryExpr { left, op, right })),
            span,
        ))
    }

    // primary_expr  ::= atom , ( member_access_expr | index_expr | function_call_expr )* ;
    fn parse_primary_expr(&mut self) -> ParserResult<Expr> {
        let mut expr = self.parse_atom()?;

        // Consume chained calls and selectors
        loop {
            expr = match self.current()?.value() {
                Token::Period => self.parse_member_access_expr(expr)?,
                Token::LeftBracket => self.parse_index_expr(expr)?,
                Token::LeftParen => self.parse_function_call(expr)?,
                _ => break,
            }
        }

        Ok(expr)
    }

    // member_access_expr ::= PERIOD , identifier ;
    fn parse_member_access_expr(&mut self, owner: Spanned<Expr>) -> ParserResult<Expr> {
        debug_assert_matches!(self.current()?.value(), Token::Period);
        self.advance();
        let member = self.parse_identifier()?;
        let span = owner.span().to(member.span());
        Ok(Spanned::new(
            Expr::MemberAccess(Box::new(MemberAccessExpr { owner, member })),
            span,
        ))
    }

    // index_expr ::= LEFT_BRACKET , expr , RIGHT_BRACKET ;
    fn parse_index_expr(&mut self, owner: Spanned<Expr>) -> ParserResult<Expr> {
        debug_assert_matches!(self.current()?.value(), Token::LeftBracket);
        self.advance();
        let index = self.parse_expr()?;
        let right_sq = self.eat(Token::RightBracket, "right square bracket")?;
        let span = owner.span().to(right_sq.span());
        Ok(Spanned::new(
            Expr::Index(Box::new(IndexExpr { owner, index })),
            span,
        ))
    }

    // function_call ::= LEFT_PAREN , arg_list , RIGHT_PAREN ;
    // arg_list      ::= expr , ( COMMA , expr )* ;
    fn parse_function_call(&mut self, name: Spanned<Expr>) -> ParserResult<Expr> {
        debug_assert_matches!(
            name.value(),
            Expr::IdentifierPath(_) | Expr::MemberAccess(_)
        );

        let left_paren = self.eat(Token::LeftParen, "left parenthesis")?;
        let mut arguments = Vec::<Spanned<Expr>>::new();

        // Allow zero or more arguments, which are expressions.
        while !matches!(self.current()?.value(), Token::RightParen) {
            arguments.push(self.parse_expr()?);

            if matches!(self.current()?.value(), Token::Comma) {
                self.advance();
            } else {
                break;
            }
        }

        let right_paren = self.eat(Token::RightParen, "right parenthesis")?;
        let span = left_paren.span().to(right_paren.span());

        Ok(Spanned::new(
            Expr::FunctionCall(Box::new(FunctionCallExpr {
                identity: name,
                arguments: Spanned::new(
                    arguments.into_boxed_slice(),
                    left_paren.span().to(right_paren.span()),
                ),
            })),
            span,
        ))
    }

    // atom ::= identifier_path
    //        | TRUE
    //        | FALSE
    //        | INT_LIT
    //        | FLOAT_LIT
    //        | STRING_LIT
    //        | LEFT_PAREN , expr , RIGHT_PAREN
    //        ;
    fn parse_atom(&mut self) -> ParserResult<Expr> {
        let first = self.current()?;

        if first.value() == Token::LeftParen {
            self.advance();

            let expr = self.parse_expr()?;
            self.eat(Token::RightParen, "right parenthesis")?;

            return Ok(expr);
        }

        if matches!(first.value(), Token::Identifier(_)) {
            let identifier_path = Box::from(self.parse_identifier_path()?.value());
            return Ok(Spanned::new(
                Expr::IdentifierPath(identifier_path),
                first.span(),
            ));
        }

        let atom = match first.value() {
            Token::True => Spanned::new(Expr::Bool(true), first.span()),
            Token::False => Spanned::new(Expr::Bool(false), first.span()),
            Token::IntLit(value) => Spanned::new(Expr::Int(value.clone()), first.span()),
            Token::FloatLit(value) => Spanned::new(Expr::Float(value.clone()), first.span()),
            Token::StringLit(value) => Spanned::new(Expr::String(value), first.span()),
            _ => {
                let err = Spanned::new(
                    ParserError::SyntaxError(
                        "expected atom (literal, identifier, or expression within parenthesis)"
                            .to_string(),
                    ),
                    first.span(),
                );
                self.report_error(&err);
                return Err(err);
            }
        };

        self.advance();
        Ok(atom)
    }

    /*
     * Helpers
     */

    fn parse_binary_op_left_assoc<OpFn, ParserFn>(
        &mut self,
        op_fn: OpFn,
        parser_fn: ParserFn,
    ) -> ParserResult<Expr>
    where
        OpFn: Fn(Token) -> Option<BinaryOp>,
        ParserFn: Fn(&mut Self) -> ParserResult<Expr>,
    {
        let mut left = parser_fn(self)?;

        loop {
            if let Some(op) = op_fn(self.current()?.value()) {
                self.advance();
                let right = parser_fn(self)?;

                let span = left.span().to(right.span());

                left = Spanned::new(
                    Expr::Binary(Box::from(BinaryExpr { left, op, right })),
                    span,
                );
            } else {
                break;
            };
        }

        Ok(left)
    }
}
