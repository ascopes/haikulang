use crate::ast::ident::{Identifier, TypeName};
use crate::span::Spanned;

#[derive(Clone, Debug, PartialEq)]
pub struct StructDecl {
    pub identifier: Spanned<Identifier>,
    pub members: Box<[Spanned<StructMemberDecl>]>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StructMemberDecl {
    pub identifier: Spanned<Identifier>,
    pub type_name: Spanned<TypeName>,
}
