use crate::lexer::{FloatLit, IntLit, StringLit};
use crate::span::Spanned;

#[derive(Clone, Debug)]
pub enum AstNode {
    BinaryOp(InnerAstNode, BinaryOp, InnerAstNode),
    UnaryOp(UnaryOp, InnerAstNode),
    Float(FloatLit),
    Int(IntLit),
    Bool(bool),
    String(StringLit),
    Var(String),
}

pub type InnerAstNode = Box<Spanned<AstNode>>;

#[derive(Clone, Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    BinaryAnd,
    BinaryOr,
    BinaryXor,
    BinaryNot,
    BinaryShl,
    BinaryShr,
    BoolAnd,
    BoolOr,
    BoolNot,
    Eq,
    NotEq,
    Less,
    LessEq,
    Greater,
    GreaterEq,
}

#[derive(Clone, Debug)]
pub enum UnaryOp {
    Plus,
    Minus,
    Not,
    Invert,
}
