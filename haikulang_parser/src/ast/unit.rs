use crate::ast::func::FunctionDecl;
use crate::ast::ident::IdentifierPath;
use crate::ast::structs::StructDecl;
use crate::span::Spanned;
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq)]
pub struct CompilationUnit {
    pub path: PathBuf,
    pub name: String,
    pub members: Box<[Spanned<CompilationUnitMember>]>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CompilationUnitMember {
    Use(Box<UseDecl>),
    Function(Box<FunctionDecl>),
    Struct(Box<StructDecl>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct UseDecl {
    pub path: Spanned<IdentifierPath>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compilation_unit_member_enum_size_is_not_too_large() {
        let desired_max_size = 24;
        let size = size_of::<CompilationUnitMember>();

        assert!(
            size <= desired_max_size,
            "CompilationUnitMember enum size is too large (wanted <= {} bytes, was {} bytes), consider boxing elements to reduce the size.",
            desired_max_size,
            size
        )
    }
}
