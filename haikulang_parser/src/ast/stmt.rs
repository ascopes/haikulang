use crate::ast::expr::Expr;
use crate::span::{Span, Spanned};

#[derive(Clone, Debug)]
pub enum Statement {
    Empty,
    Expr(Box<ExprStatement>),
    VarDecl(Box<VarDeclStatement>),
    If(Box<IfStatement>),
    While(Box<WhileStatement>),
}

#[derive(Clone, Debug)]
pub struct ExprStatement {
    pub expr: Spanned<Expr>,
}

impl ExprStatement {
    pub fn new(expr: Spanned<Expr>, end: Span) -> Spanned<Statement> {
        let span = expr.span().to(end);
        let statement = Box::new(ExprStatement { expr });
        Spanned::new(Statement::Expr(statement), span)
    }
}

#[derive(Clone, Debug)]
pub struct VarDeclStatement {
    pub name: Spanned<String>,
    pub value: Spanned<Expr>,
}

#[derive(Clone, Debug)]
pub struct IfStatement {
    pub condition: Spanned<Expr>,
    pub if_true: Spanned<Statement>,
    pub otherwise: Option<Spanned<Statement>>,
}

#[derive(Clone, Debug)]
pub struct WhileStatement {
    pub condition: Spanned<Expr>,
    pub body: Spanned<Statement>,
}
