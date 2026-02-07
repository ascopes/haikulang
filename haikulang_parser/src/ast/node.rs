use crate::ast::expr::*;
use crate::lexer::token::{FloatLit, IntLit, StrLit};

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

#[cfg(test)]
mod tests {
    use super::*;

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
}
