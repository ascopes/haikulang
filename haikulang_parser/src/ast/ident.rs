use crate::span::Spanned;

#[derive(Clone, Debug, PartialEq)]
pub struct IdentifierPath {
    pub qualifier: Box<[Spanned<Identifier>]>,
    pub local_name: Spanned<Identifier>,
}

// Note: using String instead of Box<str> for now as it renders correctly in the RustRover
// debugger. Box<str> just shows up as a byte array and is a nightmare to debug sensibly.
#[derive(Clone, Debug, PartialEq)]
pub struct Identifier {
    pub value: String,
}
