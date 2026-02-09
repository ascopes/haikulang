use crate::lexer::token::{FloatLit, IntLit, StrLit};
use crate::span::{Span, Spanned};

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Binary(Box<BinaryExpr>),
    Unary(Box<UnaryExpr>),
    Assignment(Box<AssignmentExpr>),
    MemberAccess(Box<MemberAccessExpr>),
    FunctionCall(Box<FunctionCallExpr>),
    Float(FloatLit),
    Int(IntLit),
    Bool(bool),
    String(StrLit),
    Identifier(StrLit),
}

#[derive(Clone, Debug, PartialEq)]
pub struct BinaryExpr {
    pub left: Spanned<Expr>,
    pub op: BinaryOp,
    pub right: Spanned<Expr>,
}

impl BinaryExpr {
    pub fn new(left: Spanned<Expr>, op: BinaryOp, right: Spanned<Expr>) -> Spanned<Expr> {
        let span = left.span().to(right.span());
        let node = Box::new(Self { left, op, right });
        Spanned::new(Expr::Binary(node), span)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    BinaryAnd,
    BinaryOr,
    BinaryXor,
    BinaryNot,
    BinaryShl,
    BinaryShr,
    BoolAnd,
    BoolOr,
    BoolNot,
    Eq,
    NotEq,
    Less,
    LessEq,
    Greater,
    GreaterEq,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UnaryExpr {
    pub op: UnaryOp,
    pub value: Spanned<Expr>,
}

impl UnaryExpr {
    pub fn new(start: Span, op: UnaryOp, value: Spanned<Expr>) -> Spanned<Expr> {
        let span = start.to(value.span());
        let node = Box::new(Self { op, value });
        Spanned::new(Expr::Unary(node), span)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum UnaryOp {
    Plus,
    Minus,
    Not,
    Invert,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AssignmentExpr {
    pub lvalue: Spanned<Expr>,
    pub op: Option<BinaryOp>,
    pub rvalue: Spanned<Expr>,
}

impl AssignmentExpr {
    pub fn new(
        lvalue: Spanned<Expr>,
        op: Option<BinaryOp>,
        rvalue: Spanned<Expr>,
    ) -> Spanned<Expr> {
        let span = lvalue.span().to(rvalue.span());
        let node = Box::new(Self { lvalue, op, rvalue });
        Spanned::new(Expr::Assignment(node), span)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MemberAccessExpr {
    pub owner: Spanned<Expr>,
    pub member: Spanned<String>,
}

impl MemberAccessExpr {
    pub fn new(owner: Spanned<Expr>, member: Spanned<String>) -> Spanned<Expr> {
        let span = owner.span().to(member.span());
        let node = Box::new(Self { owner, member });
        Spanned::new(Expr::MemberAccess(node), span)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionCallExpr {
    pub name: Spanned<Expr>,
    pub arguments: Box<[Spanned<Expr>]>,
}

impl FunctionCallExpr {
    pub fn new(owner: Spanned<Expr>, arguments: Box<[Spanned<Expr>]>, end: Span) -> Spanned<Expr> {
        let span = owner.span().to(end);
        let node = Box::new(Self {
            name: owner,
            arguments,
        });
        Spanned::new(Expr::FunctionCall(node), span)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::span::{Span, Spanned};

    #[test]
    fn expr_enum_size_is_not_too_large() {
        let desired_max_size = 24;
        let size = size_of::<Expr>();

        assert!(
            size <= desired_max_size,
            "Expr enum size is too large (wanted <= {} bytes, was {} bytes), consider boxing elements to reduce the size.",
            desired_max_size,
            size
        )
    }

    #[test]
    fn binary_expr_constructs_correctly() {
        // Given
        let left = Spanned::new(Expr::Int(123), Span::new(5, 8));
        let op = BinaryOp::Mul;
        let right = Spanned::new(Expr::Int(456), Span::new(12, 15));

        // When
        let expr = BinaryExpr::new(left, op, right);

        // Then
        assert_eq!(expr.span(), Span::new(5, 15));
        match expr.value() {
            Expr::Binary(boxed_expr) => {
                assert!(
                    matches!(boxed_expr.left.value(), Expr::Int(123)),
                    "expected Int(123), got {:?}",
                    boxed_expr.left.value()
                );
                assert_eq!(boxed_expr.left.span(), Span::new(5, 8));

                assert_eq!(boxed_expr.op, BinaryOp::Mul);

                assert!(
                    matches!(boxed_expr.right.value(), Expr::Int(456)),
                    "expected Int(456), got {:?}",
                    boxed_expr.right.value()
                );
                assert_eq!(boxed_expr.right.span(), Span::new(12, 15));
            }
            other => panic!("Expected BinaryExpr, got {:?}", other),
        }
    }

    #[test]
    fn unary_expr_constructs_correctly() {
        // Given
        let op_span = Span::new(11, 12);
        let op = UnaryOp::Invert;
        let value = Spanned::new(Expr::Int(456), Span::new(12, 15));

        // When
        let expr = UnaryExpr::new(op_span, op, value);

        // Then
        assert_eq!(expr.span(), Span::new(11, 15));
        match expr.value() {
            Expr::Unary(boxed_expr) => {
                assert_eq!(boxed_expr.op, UnaryOp::Invert);

                assert!(
                    matches!(boxed_expr.value.value(), Expr::Int(456)),
                    "expected Int(456), got {:?}",
                    boxed_expr.value.value()
                );
                assert_eq!(boxed_expr.value.span(), Span::new(12, 15));
            }
            other => panic!("Expected UnaryExpr, got {:?}", other),
        }
    }

    #[test]
    fn assignment_expr_constructs_correctly() {
        // Given
        let left = Spanned::new(Expr::Int(123), Span::new(5, 8));
        let op = BinaryOp::Mul;
        let right = Spanned::new(Expr::Int(456), Span::new(12, 15));

        // When
        let expr = AssignmentExpr::new(left, Some(op), right);

        // Then
        assert_eq!(expr.span(), Span::new(5, 15));
        match expr.value() {
            Expr::Assignment(boxed_expr) => {
                assert!(
                    matches!(boxed_expr.lvalue.value(), Expr::Int(123)),
                    "expected Int(123), got {:?}",
                    boxed_expr.lvalue.value()
                );
                assert_eq!(boxed_expr.lvalue.span(), Span::new(5, 8));

                assert!(matches!(boxed_expr.op, Some(BinaryOp::Mul)));

                assert!(
                    matches!(boxed_expr.rvalue.value(), Expr::Int(456)),
                    "expected Int(456), got {:?}",
                    boxed_expr.rvalue.value()
                );
                assert_eq!(boxed_expr.rvalue.span(), Span::new(12, 15));
            }
            other => panic!("Expected AssignmentExpr, got {:?}", other),
        }
    }

    #[test]
    fn member_access_expr_constructs_correctly() {
        // Given
        let owner = Spanned::new(Expr::Identifier(Box::from("foo")), Span::new(3, 6));
        let member = Spanned::new("bar".to_string(), Span::new(7, 10));

        // When
        let expr = MemberAccessExpr::new(owner, member);

        // Then
        assert_eq!(expr.span(), Span::new(3, 10));
        match expr.value() {
            Expr::MemberAccess(boxed_expr) => {
                match boxed_expr.owner.value() {
                    Expr::Identifier(str) => assert_eq!(str.as_ref(), "foo"),
                    other => panic!("Expected Identifier for member, got {:?}", other),
                }
                assert_eq!(boxed_expr.owner.span(), Span::new(3, 6));

                assert_eq!(boxed_expr.member.value(), "bar".to_string());
                assert_eq!(boxed_expr.member.span(), Span::new(7, 10));
            }
            other => panic!("Expected MemberAccessExpr, got {:?}", other),
        }
    }

    #[test]
    fn function_call_expr_constructs_correctly() {
        // Given
        let name = Spanned::new(Expr::Identifier(Box::from("foo")), Span::new(3, 6));
        let args: [Spanned<Expr>; 3] = [
            Spanned::new(Expr::Int(64), Span::new(5, 7)),
            Spanned::new(Expr::Float(69.1), Span::new(10, 13)),
            BinaryExpr::new(
                Spanned::new(Expr::Int(12), Span::new(15, 20)),
                BinaryOp::Div,
                Spanned::new(Expr::Float(23.1), Span::new(20, 25)),
            ),
        ];
        let close_paren_span = Span::new(27, 28);

        // When
        let expr = FunctionCallExpr::new(name, Box::from(args), close_paren_span);

        // Then
        assert_eq!(expr.span(), Span::new(3, 28));
        match expr.value() {
            Expr::FunctionCall(boxed_expr) => {
                assert_eq!(boxed_expr.name.span(), Span::new(3, 6));
                match boxed_expr.name.value() {
                    Expr::Identifier(name) => assert_eq!(name.as_ref(), "foo"),
                    other => panic!("Expected Identifier for name, got {:?}", other),
                }

                assert_eq!(boxed_expr.arguments.len(), 3);

                let first_arg = boxed_expr.arguments.get(0).unwrap();
                assert_eq!(first_arg.span(), Span::new(5, 7));
                assert!(matches!(first_arg.value(), Expr::Int(64)));

                let second_arg = boxed_expr.arguments.get(1).unwrap();
                assert_eq!(second_arg.span(), Span::new(10, 13));
                assert!(matches!(second_arg.value(), Expr::Float(69.1)));

                let third_arg = boxed_expr.arguments.get(2).unwrap();
                assert_eq!(third_arg.span(), Span::new(15, 25));
                assert!(matches!(third_arg.value(), Expr::Binary(_)));
            }
            other => panic!("Expected FunctionCallExpr, got {:?}", other),
        }
    }
}
