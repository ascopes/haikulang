use crate::ast::ident::Identifier;
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
    Identifier(Box<Identifier>),
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

impl MemberAccessExpr {
    pub fn new(owner: Spanned<Expr>, member: Spanned<Identifier>) -> Spanned<Expr> {
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
}
