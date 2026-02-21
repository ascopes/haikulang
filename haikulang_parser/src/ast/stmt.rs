use crate::ast::expr::Expr;
use crate::ast::ident::{Identifier, TypeName};
use crate::span::Spanned;

#[derive(Clone, Debug, PartialEq)]
pub enum Statement {
    Empty,
    Expr(Box<ExprStatement>),
    VarDecl(Box<VarDeclStatement>),
    Use(Box<UseStatement>),
    If(Box<IfStatement>),
    While(Box<WhileStatement>),
    Block(Box<BlockStatement>),
    Break,
    Continue,
    Return(Box<ReturnStatement>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprStatement {
    pub expr: Spanned<Expr>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct VarDeclStatement {
    pub identifier: Spanned<Identifier>,
    pub expr: Option<Spanned<Expr>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UseStatement {
    pub path: Spanned<TypeName>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct IfStatement {
    pub condition: Spanned<Expr>,
    pub body: Spanned<Statement>,
    pub otherwise: Option<Spanned<Statement>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WhileStatement {
    pub condition: Spanned<Expr>,
    pub body: Spanned<Statement>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BlockStatement {
    pub statements: Box<[Spanned<Statement>]>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ReturnStatement {
    pub expr: Option<Spanned<Expr>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn statement_enum_size_is_not_too_large() {
        let desired_max_size = 24;
        let size = size_of::<Statement>();

        assert!(
            size <= desired_max_size,
            "Statement enum size is too large (wanted <= {} bytes, was {} bytes), consider boxing elements to reduce the size.",
            desired_max_size,
            size
        )
    }
}
