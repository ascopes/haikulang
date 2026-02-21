use crate::ast::ident::{Identifier, TypeName};
use crate::ast::stmt::Statement;
use crate::span::Spanned;

#[derive(Clone, Debug, PartialEq)]
pub struct Function {
    pub name: Spanned<Identifier>,
    pub parameters: Spanned<Box<[Spanned<ParameterDecl>]>>,
    pub return_type: Option<Spanned<TypeName>>,
    pub body: Spanned<Statement>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ParameterDecl {
    pub name: Spanned<Identifier>,
    pub type_name: Spanned<TypeName>,
}
