use crate::ast::ident::TypeName;
use crate::ast::stmt::Statement;
use crate::span::Spanned;

#[derive(Clone, Debug, PartialEq)]
pub struct Function {
    pub name: Spanned<String>,
    pub parameters: Spanned<Box<[Spanned<ParameterDecl>]>>,
    pub return_type: Option<Spanned<TypeName>>,
    pub body: Spanned<Statement>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ParameterDecl {
    pub name: Spanned<String>,
    pub type_name: Spanned<TypeName>,
}
