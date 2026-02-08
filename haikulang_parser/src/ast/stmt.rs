use crate::ast::expr::Expr;
use crate::span::Spanned;

#[derive(Clone, Debug)]
pub enum Stmt {
    Expr(Box<Expr>),
    VarDecl(Box<VarDeclStmt>),
    Block(Box<BlockStmt>),
    If(Box<IfStmt>),
    While(Box<WhileStmt>),
}

#[derive(Clone, Debug)]
pub struct ExprStmt {
    pub expr: Spanned<Stmt>,
}

#[derive(Clone, Debug)]
pub struct VarDeclStmt {
    pub name: Spanned<String>,
    pub value: Spanned<Expr>,
}

// TODO(ascopes): should blocks be treated as expressions that yield their last
//  attribute as the expression result? If so, BlockStmt should be changed to a
//  BlockExpr.
#[derive(Clone, Debug)]
pub struct BlockStmt {
    pub stmts: Box<[Spanned<Stmt>]>,
}

#[derive(Clone, Debug)]
pub struct IfStmt {
    pub condition: Spanned<Expr>,
    pub if_true_block: Spanned<BlockStmt>,
    pub else_block: Option<Spanned<ElseClause>>,
}

#[derive(Clone, Debug)]
pub enum ElseClause {
    Block(Box<BlockStmt>),
    If(Box<IfStmt>),
}

#[derive(Clone, Debug)]
pub struct WhileStmt {
    pub condition: Spanned<Expr>,
    pub body: Spanned<Stmt>,
}
