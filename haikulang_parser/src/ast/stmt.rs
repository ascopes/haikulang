use crate::ast::expr::Expr;
use crate::span::{Span, Spanned};

#[derive(Clone, Debug)]
pub enum Statement {
    Empty,
    Expr(Box<ExprStatement>),
    VarDecl(Box<VarDeclStatement>),
    Block(Box<BlockStatement>),
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

// TODO(ascopes): should blocks be treated as expressions that yield their last
//  attribute as the expression result? If so, BlockStatement should be changed to a
//  BlockExpr.
#[derive(Clone, Debug)]
pub struct BlockStatement {
    pub statements: Box<[Spanned<Statement>]>,
}

#[derive(Clone, Debug)]
pub struct IfStatement {
    pub condition: Spanned<Expr>,
    pub if_true_block: Spanned<BlockStatement>,
    pub else_block: Option<Spanned<ElseClause>>,
}

#[derive(Clone, Debug)]
pub enum ElseClause {
    Block(Box<BlockStatement>),
    If(Box<IfStatement>),
}

#[derive(Clone, Debug)]
pub struct WhileStatement {
    pub condition: Spanned<Expr>,
    pub body: Spanned<Statement>,
}
