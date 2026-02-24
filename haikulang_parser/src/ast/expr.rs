use crate::ast::ident::{Identifier, IdentifierPath};
use crate::lexer::token::{FloatLit, IntLit, StrLit};
use crate::span::Spanned;

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Binary(Box<BinaryExpr>),
    Unary(Box<UnaryExpr>),
    Assignment(Box<AssignmentExpr>),
    MemberAccess(Box<MemberAccessExpr>),
    Index(Box<IndexExpr>),
    FunctionCall(Box<FunctionCallExpr>),
    Float(Box<FloatLitExpr>),
    Int(Box<IntLitExpr>),
    Bool(Box<BoolLitExpr>),
    String(Box<StrLitExpr>),
    IdentifierPath(Box<IdentifierPath>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct BinaryExpr {
    pub left: Spanned<Expr>,
    pub op: BinaryOp,
    pub right: Spanned<Expr>,
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

#[derive(Clone, Debug, PartialEq)]
pub struct MemberAccessExpr {
    pub owner: Spanned<Expr>,
    pub member: Spanned<Identifier>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct IndexExpr {
    pub owner: Spanned<Expr>,
    pub index: Spanned<Expr>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionCallExpr {
    pub identity: Spanned<Expr>,
    pub arguments: Spanned<Box<[Spanned<Expr>]>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FloatLitExpr {
    pub value: FloatLit,
}

#[derive(Clone, Debug, PartialEq)]
pub struct IntLitExpr {
    pub value: IntLit,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BoolLitExpr {
    pub value: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StrLitExpr {
    pub value: StrLit,
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn binary_op_enum_size_is_not_too_large() {
        let desired_max_size = 8;
        let size = size_of::<BinaryOp>();

        assert!(
            size <= desired_max_size,
            "BinaryOp enum size is too large (wanted <= {} bytes, was {} bytes), consider boxing elements to reduce the size.",
            desired_max_size,
            size
        )
    }

    #[test]
    fn unary_op_enum_size_is_not_too_large() {
        let desired_max_size = 8;
        let size = size_of::<UnaryOp>();

        assert!(
            size <= desired_max_size,
            "UnaryOp enum size is too large (wanted <= {} bytes, was {} bytes), consider boxing elements to reduce the size.",
            desired_max_size,
            size
        )
    }
}
