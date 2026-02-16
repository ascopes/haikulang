use crate::ast::stmt::Statement;
use crate::span::Spanned;

#[derive(Clone, Debug, PartialEq)]
pub struct Function {
    pub identifier: Spanned<String>,
    pub parameters: Box<[Spanned<ParameterDecl>]>,
    pub return_type: Option<Spanned<TypeName>>,
    pub body: Spanned<Statement>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ParameterDecl {
    pub name: Spanned<String>,
    pub type_name: Spanned<TypeName>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeName {
    pub qualifier: Box<[Spanned<String>]>,
    pub local_name: Spanned<String>,
}
