use crate::lexer::token::{FloatLit, IntLit, StrLit};
use crate::span::{Span, Spanned};

// TODO(ascopes): use a string interner to reduce memory overhead later.
#[derive(Clone, Debug)]
pub enum AstNode {
    BinaryExpr(Box<BinaryExpr>),
    UnaryExpr(Box<UnaryExpr>),
    AssignmentExpr(Box<AssignmentExpr>),
    MemberAccessExpr(Box<MemberAccessExpr>),
    FunctionCallExpr(Box<FunctionCallExpr>),
    Float(FloatLit),
    Int(IntLit),
    Bool(bool),
    String(StrLit),
    Identifier(StrLit),
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

#[derive(Clone, Debug)]
pub struct MemberAccessExpr {
    pub owner: Spanned<AstNode>,
    pub member: Spanned<String>,
}

impl MemberAccessExpr {
    pub fn new(owner: Spanned<AstNode>, member: Spanned<String>) -> Spanned<AstNode> {
        let span = owner.span().to(member.span());
        let node = Box::new(Self { owner, member });
        Spanned::new(AstNode::MemberAccessExpr(node), span)
    }
}

#[derive(Clone, Debug)]
pub struct FunctionCallExpr {
    pub name: Spanned<AstNode>,
    pub arguments: Box<[Spanned<AstNode>]>,
}

impl FunctionCallExpr {
    pub fn new(
        owner: Spanned<AstNode>,
        arguments: Box<[Spanned<AstNode>]>,
        end: Span,
    ) -> Spanned<AstNode> {
        let span = owner.span().to(end);
        let node = Box::new(Self {
            name: owner,
            arguments,
        });
        Spanned::new(AstNode::FunctionCallExpr(node), span)
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

    #[test]
    fn member_access_expr_constructs_correctly() {
        // Given
        let owner = Spanned::new(AstNode::Identifier(Box::from("foo")), Span::new(3, 6));
        let member = Spanned::new("bar".to_string(), Span::new(7, 10));

        // When
        let expr = MemberAccessExpr::new(owner, member);

        // Then
        assert_eq!(expr.span(), Span::new(3, 10));
        match expr.value() {
            AstNode::MemberAccessExpr(boxed_expr) => {
                match boxed_expr.owner.value() {
                    AstNode::Identifier(str) => assert_eq!(str.as_ref(), "foo"),
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
        let name = Spanned::new(AstNode::Identifier(Box::from("foo")), Span::new(3, 6));
        let args: [Spanned<AstNode>; 3] = [
            Spanned::new(AstNode::Int(64), Span::new(5, 7)),
            Spanned::new(AstNode::Float(69.1), Span::new(10, 13)),
            BinaryExpr::new(
                Spanned::new(AstNode::Int(12), Span::new(15, 20)),
                BinaryOp::Div,
                Spanned::new(AstNode::Float(23.1), Span::new(20, 25)),
            ),
        ];
        let close_paren_span = Span::new(27, 28);

        // When
        let expr = FunctionCallExpr::new(name, Box::from(args), close_paren_span);

        // Then
        assert_eq!(expr.span(), Span::new(3, 28));
        match expr.value() {
            AstNode::FunctionCallExpr(boxed_expr) => {
                assert_eq!(boxed_expr.name.span(), Span::new(3, 6));
                match boxed_expr.name.value() {
                    AstNode::Identifier(name) => assert_eq!(name.as_ref(), "foo"),
                    other => panic!("Expected Identifier for name, got {:?}", other),
                }

                assert_eq!(boxed_expr.arguments.len(), 3);

                let first_arg = boxed_expr.arguments.get(0).unwrap();
                assert_eq!(first_arg.span(), Span::new(5, 7));
                assert!(matches!(first_arg.value(), AstNode::Int(64)));

                let second_arg = boxed_expr.arguments.get(1).unwrap();
                assert_eq!(second_arg.span(), Span::new(10, 13));
                assert!(matches!(second_arg.value(), AstNode::Float(69.1)));

                let third_arg = boxed_expr.arguments.get(2).unwrap();
                assert_eq!(third_arg.span(), Span::new(15, 25));
                assert!(matches!(third_arg.value(), AstNode::BinaryExpr(_)));
            }
            other => panic!("Expected FunctionCallExpr, got {:?}", other),
        }
    }
}
