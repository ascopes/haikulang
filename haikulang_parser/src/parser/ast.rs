use crate::lexer::{FloatValue, IntValue, StrValue};
use crate::location::Location;

#[derive(Debug, PartialEq)]
pub struct AstNode {
    pub kind: AstNodeKind,
    pub location: Location,
}

#[derive(Debug, PartialEq)]
pub enum AstNodeKind {
    BinaryOp(Box<AstNode>, Operator, Box<AstNode>),
    UnaryOp(Operator, Box<AstNode>),
    Identifier(String),
    Literal(Literal),
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    Float(FloatValue),
    Int(IntValue),
    Str(StrValue),
    Bool(bool),
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    IntDiv,
    Mod,
    Pow,
}
