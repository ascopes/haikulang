use crate::lexer::{FloatLit, IntLit, StringLit};

#[derive(Clone, Debug)]
pub enum AstNode {
    BinOp(InnerAstNode, BinaryOp, InnerAstNode),
    UnaryOp(UnaryOp, InnerAstNode),
    Float(FloatLit),
    Int(IntLit),
    Bool(bool),
    String(StringLit),
    Var(String),
}

pub type InnerAstNode = Box<AstNode>;

#[derive(Clone, Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
}

#[derive(Clone, Debug)]
pub enum UnaryOp {
    Plus,
    Minus,
    Not,
    Invert,
}
