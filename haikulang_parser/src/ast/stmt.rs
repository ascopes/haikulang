use crate::ast::expr::Expr;
use crate::span::{Span, Spanned};

#[derive(Clone, Debug, PartialEq)]
pub enum Statement {
    Empty,
    Expr(Box<ExprStatement>),
    VarDecl(Box<VarDeclStatement>),
    If(Box<IfStatement>),
    While(Box<WhileStatement>),
}

#[derive(Clone, Debug, PartialEq)]
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

#[derive(Clone, Debug, PartialEq)]
pub struct VarDeclStatement {
    pub identifier: Spanned<String>,
    pub expr: Option<Spanned<Expr>>,
}

impl VarDeclStatement {
    pub fn new(
        start: Span,
        identifier: Spanned<String>,
        expr: Option<Spanned<Expr>>,
        end: Span,
    ) -> Spanned<Statement> {
        let span = start.to(end);
        let statement = Box::new(VarDeclStatement { identifier, expr });
        Spanned::new(Statement::VarDecl(statement), span)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct IfStatement {
    pub condition: Spanned<Expr>,
    pub if_true: Spanned<Statement>,
    pub otherwise: Option<Spanned<Statement>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WhileStatement {
    pub condition: Spanned<Expr>,
    pub body: Spanned<Statement>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::expr::*;
    use crate::span::{Span, Spanned};

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

    #[test]
    fn expr_constructs_correctly() {
        // Given
        let expr = BinaryExpr::new(
            Spanned::new(Expr::Int(123), Span::new(5, 8)),
            BinaryOp::Mul,
            Spanned::new(Expr::Int(456), Span::new(12, 15)),
        );
        let end_span = Span::new(15, 16);

        // When
        let stmt = ExprStatement::new(expr.clone(), end_span);

        // Then
        assert_eq!(stmt.span(), Span::new(5, 16));
        match stmt.value() {
            Statement::Expr(value) => assert_eq!(value.expr.value(), expr.value()),
            _ => panic!("expected Expr, got {:?}", stmt.value()),
        }
    }

    #[test]
    fn var_decl_constructs_correctly() {
        // Given
        let start_span = Span::new(1, 4);
        let identifier = Spanned::new("foo".to_string(), Span::new(5, 8));
        let expr = BinaryExpr::new(
            Spanned::new(Expr::Int(123), Span::new(12, 15)),
            BinaryOp::Mul,
            Spanned::new(Expr::Int(456), Span::new(18, 21)),
        );
        let end_span = Span::new(21, 22);

        // When
        let stmt = VarDeclStatement::new(start_span, identifier, Some(expr.clone()), end_span);

        // Then
        assert_eq!(stmt.span(), Span::new(1, 22));
        match stmt.value() {
            Statement::VarDecl(value) => {
                assert_eq!(value.identifier.value(), "foo");
                assert_eq!(value.expr.expect("no expr").value(), expr.value());
            }
            _ => panic!("expected VarDecl, got {:?}", stmt.value()),
        }
    }
}
