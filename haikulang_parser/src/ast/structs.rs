use crate::ast::ident::{Identifier, IdentifierPath};
use crate::span::Spanned;

#[derive(Clone, Debug, PartialEq)]
pub struct StructDecl {
    pub identifier: Spanned<Identifier>,
    pub members: Box<[Spanned<StructMemberDecl>]>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StructMemberDecl {
    pub identifier: Spanned<Identifier>,
    pub identifier_path: Spanned<IdentifierPath>,
}
