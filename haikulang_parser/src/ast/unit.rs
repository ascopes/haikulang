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
