use crate::ast::ident::{Identifier, IdentifierPath};
use crate::ast::stmt::Statement;
use crate::span::Spanned;

#[derive(Clone, Debug, PartialEq)]
pub struct ExternFunctionDecl {
    pub name: Spanned<Identifier>,
    pub parameters: Spanned<Box<[Spanned<ParameterDecl>]>>,
    pub return_type: Option<Spanned<IdentifierPath>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionDecl {
    pub name: Spanned<Identifier>,
    pub parameters: Spanned<Box<[Spanned<ParameterDecl>]>>,
    pub return_type: Option<Spanned<IdentifierPath>>,
    pub body: Spanned<Statement>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ParameterDecl {
    pub name: Spanned<Identifier>,
    pub type_name: Spanned<IdentifierPath>,
}
