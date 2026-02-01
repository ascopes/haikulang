use crate::lexer::token::{FloatLit, IntLit, StrLit};
use crate::span::{Span, Spanned};

// TODO(ascopes): use a string interner to reduce memory overhead later.
#[derive(Clone, Debug)]
pub enum AstNode {
    BinaryExpr(Box<BinaryExpr>),
    UnaryExpr(Box<UnaryExpr>),
    AssignmentExpr(Box<AssignmentExpr>),
    Float(FloatLit),
    Int(IntLit),
    Bool(bool),
    String(StrLit),
    Var(StrLit),
}

#[derive(Clone, Debug)]
pub struct BinaryExpr {
    pub left: Spanned<AstNode>,
    pub op: BinaryOp,
    pub right: Spanned<AstNode>,
}

impl BinaryExpr {
    pub fn new(left: Spanned<AstNode>, op: BinaryOp, right: Spanned<AstNode>) -> Spanned<AstNode> {
        let span = left.span().to(right.span());
        let node = Box::new(Self { left, op, right });
        Spanned::new(AstNode::BinaryExpr(node), span)
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

#[derive(Clone, Debug)]
pub struct UnaryExpr {
    pub op: UnaryOp,
    pub value: Spanned<AstNode>,
}

impl UnaryExpr {
    pub fn new(start: Span, op: UnaryOp, value: Spanned<AstNode>) -> Spanned<AstNode> {
        let span = start.to(value.span());
        let node = Box::new(Self { op, value });
        Spanned::new(AstNode::UnaryExpr(node), span)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum UnaryOp {
    Plus,
    Minus,
    Not,
    Invert,
}

#[derive(Clone, Debug)]
pub struct AssignmentExpr {
    pub lvalue: Spanned<AstNode>,
    pub op: Option<BinaryOp>,
    pub rvalue: Spanned<AstNode>,
}

impl AssignmentExpr {
    pub fn new(
        lvalue: Spanned<AstNode>,
        op: Option<BinaryOp>,
        rvalue: Spanned<AstNode>,
    ) -> Spanned<AstNode> {
        let span = lvalue.span().to(rvalue.span());
        let node = Box::new(Self { lvalue, op, rvalue });
        Spanned::new(AstNode::AssignmentExpr(node), span)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast::AstNode;
    use crate::span::{Span, Spanned};

    #[test]
    fn ast_node_size_is_not_too_large() {
        let desired_max_size = 24;
        let size = size_of::<AstNode>();

        assert!(
            size <= desired_max_size,
            "AstNode size is too large (wanted <= {} bytes, was {} bytes), consider boxing elements to reduce the size.",
            desired_max_size,
            size
        )
    }

    #[test]
    fn binary_expr_constructs_correctly() {
        // Given
        let left = Spanned::new(AstNode::Int(123), Span::new(5, 8));
        let op = BinaryOp::Mul;
        let right = Spanned::new(AstNode::Int(456), Span::new(12, 15));

        // When
        let expr = BinaryExpr::new(left, op, right);

        // Then
        assert_eq!(expr.span(), Span::new(5, 15));
        match expr.value() {
            AstNode::BinaryExpr(boxed_expr) => {
                assert!(
                    matches!(boxed_expr.left.value(), AstNode::Int(123)),
                    "expected Int(123), got {:?}",
                    boxed_expr.left.value()
                );
                assert_eq!(boxed_expr.left.span(), Span::new(5, 8));

                assert_eq!(boxed_expr.op, BinaryOp::Mul);

                assert!(
                    matches!(boxed_expr.right.value(), AstNode::Int(456)),
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
        let value = Spanned::new(AstNode::Int(456), Span::new(12, 15));

        // When
        let expr = UnaryExpr::new(op_span, op, value);

        // Then
        assert_eq!(expr.span(), Span::new(11, 15));
        match expr.value() {
            AstNode::UnaryExpr(boxed_expr) => {
                assert_eq!(boxed_expr.op, UnaryOp::Invert);

                assert!(
                    matches!(boxed_expr.value.value(), AstNode::Int(456)),
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
        let left = Spanned::new(AstNode::Int(123), Span::new(5, 8));
        let op = BinaryOp::Mul;
        let right = Spanned::new(AstNode::Int(456), Span::new(12, 15));

        // When
        let expr = AssignmentExpr::new(left, Some(op), right);

        // Then
        assert_eq!(expr.span(), Span::new(5, 15));
        match expr.value() {
            AstNode::AssignmentExpr(boxed_expr) => {
                assert!(
                    matches!(boxed_expr.lvalue.value(), AstNode::Int(123)),
                    "expected Int(123), got {:?}",
                    boxed_expr.lvalue.value()
                );
                assert_eq!(boxed_expr.lvalue.span(), Span::new(5, 8));

                assert!(matches!(boxed_expr.op, Some(BinaryOp::Mul)));

                assert!(
                    matches!(boxed_expr.rvalue.value(), AstNode::Int(456)),
                    "expected Int(456), got {:?}",
                    boxed_expr.rvalue.value()
                );
                assert_eq!(boxed_expr.rvalue.span(), Span::new(12, 15));
            }
            other => panic!("Expected AssignmentExpr, got {:?}", other),
        }
    }
}
