use crate::span::Spanned;

#[derive(Clone, Debug, PartialEq)]
pub struct TypeName {
    pub qualifier: Box<[Spanned<String>]>,
    pub local_name: Spanned<String>,
}
