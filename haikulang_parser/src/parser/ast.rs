use crate::lexer::{FloatValue, IntValue, StrValue};
use crate::location::Location;

pub struct AstNode {
    pub kind: AstNodeKind,
    pub location: Location,
}

#[derive(Debug, PartialEq)]
pub enum AstNodeKind {
    BinaryOp(Box<Self>, Operator, Box<Self>),
    UnaryOp(Operator, Box<Self>),
    Literal(Literal),
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    Float(FloatValue),
    Int(IntValue),
    Str(StrValue),
}

#[derive(Debug, PartialEq)]
pub enum Operator {}
