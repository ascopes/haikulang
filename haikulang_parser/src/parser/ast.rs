use crate::lexer::{FloatValue, IntValue, StrValue};
use crate::location::Location;

#[derive(Debug, PartialEq)]
pub struct AstNode {
    pub kind: AstNodeKind,
    pub location: Location,
}

pub type NestedAstNode = Box<AstNode>;

#[derive(Debug, PartialEq)]
pub enum AstNodeKind {
    BinaryOperator(NestedAstNode, BinaryOperator, NestedAstNode),
    UnaryOperator(UnaryOperator, NestedAstNode),
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
pub enum UnaryOperator {
    Pos,
    Neg,
    Not,
}

#[derive(Debug, PartialEq)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    IntDiv,
    Mod,
    Pow,
    Eq,
    Ne,
    Lt,
    Lte,
    Gt,
    Gte,
    And,
    Or,
}
