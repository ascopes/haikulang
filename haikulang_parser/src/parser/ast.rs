use crate::lexer::{FloatLit, IntLit, StringLit};
use crate::span::Spanned;

#[derive(Clone, Debug)]
pub enum AstNode {
    // E.g. 1 + 2
    BinaryOp {
        left: InnerAstNode,
        op: BinaryOp,
        right: InnerAstNode,
    },
    // E.g. -4, not false, etc
    UnaryOp {
        op: UnaryOp,
        value: InnerAstNode,
    },
    // E.g. x = {{expr}} or x += {{expr}}
    Assignment {
        lvalue: InnerAstNode,
        op: Option<BinaryOp>,
        rvalue: InnerAstNode,
    },

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
