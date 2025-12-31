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
    True,
    False,

    // Operators
    Add,
    Sub,
    Mul,
    Div,
    IntDiv,
    Mod,
    Pow,

    // Flow control and structures
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Semi,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub raw: String,
    pub location: Location,
}
