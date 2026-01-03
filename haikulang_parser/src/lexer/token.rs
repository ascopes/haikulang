use crate::location::Location;

pub type IntValue = u64;
pub type FloatValue = f64;
pub type StrValue = String;

#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    Eof,

    // Literals
    Ident,
    StrLit(StrValue),
    IntLit(IntValue),
    FloatLit(FloatValue),

    // Keywords
    Fn,
    Return,
    If,
    Else,
    For,
    While,
    Break,
    Continue,
    True,
    False,

    // Arithmetic operators
    Add,
    Sub,
    Mul,
    Div,
    IntDiv,
    Mod,
    Pow,

    // Boolean operators
    Not,
    Eq,
    Ne,
    Lt,
    Lte,
    Gt,
    Gte,
    And,
    Or,

    // Assignment
    Assign,

    // Flow control and structures
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftSq,
    RightSq,
    Semi,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub raw: String,
    pub location: Location,
}
