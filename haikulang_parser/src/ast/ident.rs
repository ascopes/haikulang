use crate::span::Spanned;

#[derive(Clone, Debug, PartialEq)]
pub struct TypeName {
    pub qualifier: Box<[Spanned<Identifier>]>,
    pub local_name: Spanned<Identifier>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Identifier(Box<str>);

impl Identifier {
    pub fn from_str(value: &str) -> Self {
        Self(Box::from(value))
    }

    pub fn from_string(value: String) -> Self {
        Self(value.into_boxed_str())
    }

    pub fn from_boxed_str(value: Box<str>) -> Self {
        Self(value)
    }
}
