use crate::ast::expr::Expr;
use crate::ast::ident::{Identifier, TypeName};
use crate::span::{Span, Spanned};

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

impl ExprStatement {
    pub fn new(expr: Spanned<Expr>) -> Spanned<Statement> {
        let span = expr.span();
        let statement = Box::new(Self { expr });
        Spanned::new(Statement::Expr(statement), span)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct VarDeclStatement {
    pub identifier: Spanned<Identifier>,
    pub expr: Option<Spanned<Expr>>,
}

impl VarDeclStatement {
    pub fn new(
        start: Span,
        identifier: Spanned<Identifier>,
        expr: Option<Spanned<Expr>>,
    ) -> Spanned<Statement> {
        let span = if let Some(spanned_expr) = &expr {
            start.to(spanned_expr.span())
        } else {
            start.to(identifier.span())
        };
        let statement = Box::new(Self { identifier, expr });
        Spanned::new(Statement::VarDecl(statement), span)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct UseStatement {
    pub path: Spanned<TypeName>,
}

impl UseStatement {
    pub fn new(start: Span, path: Spanned<TypeName>) -> Spanned<Statement> {
        let span = start.to(path.span());
        let statement = Box::new(Self { path });
        Spanned::new(Statement::Use(statement), span)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct IfStatement {
    pub condition: Spanned<Expr>,
    pub body: Spanned<Statement>,
    pub otherwise: Option<Spanned<Statement>>,
}

impl IfStatement {
    pub fn new(
        start: Span,
        condition: Spanned<Expr>,
        body: Spanned<Statement>,
        otherwise: Option<Spanned<Statement>>,
    ) -> Spanned<Statement> {
        let span = if let Some(spanned_expr) = &otherwise {
            start.to(spanned_expr.span())
        } else {
            start.to(body.span())
        };
        let statement = Box::new(Self {
            condition,
            body,
            otherwise,
        });
        Spanned::new(Statement::If(statement), span)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct WhileStatement {
    pub condition: Spanned<Expr>,
    pub body: Spanned<Statement>,
}

impl WhileStatement {
    pub fn new(
        start: Span,
        condition: Spanned<Expr>,
        body: Spanned<Statement>,
    ) -> Spanned<Statement> {
        let span = start.to(body.span());
        let statement = Box::new(Self { condition, body });
        Spanned::new(Statement::While(statement), span)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct BlockStatement {
    pub statements: Box<[Spanned<Statement>]>,
}

impl BlockStatement {
    pub fn new(
        start: Span,
        statements: Box<[Spanned<Statement>]>,
        end: Span,
    ) -> Spanned<Statement> {
        let span = start.to(end);
        let statement = Box::new(Self { statements });
        Spanned::new(Statement::Block(statement), span)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ReturnStatement {
    pub expr: Option<Spanned<Expr>>,
}

impl ReturnStatement {
    pub fn new(start: Span, expr: Option<Spanned<Expr>>) -> Spanned<Statement> {
        let span = if let Some(spanned_expr) = &expr {
            start.to(spanned_expr.span())
        } else {
            start
        };
        let statement = Box::new(Self { expr });
        Spanned::new(Statement::Return(statement), span)
    }
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
