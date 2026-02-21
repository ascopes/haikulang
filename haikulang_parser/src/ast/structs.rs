use crate::ast::ident::{Identifier, TypeName};
use crate::span::Spanned;

#[derive(Clone, Debug, PartialEq)]
pub struct Struct {
    pub identifier: Spanned<Identifier>,
    pub members: Box<[Spanned<StructMember>]>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StructMember {
    pub identifier: Spanned<Identifier>,
    pub type_name: Spanned<TypeName>,
}
