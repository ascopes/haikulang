use crate::location::Location;

pub type IntValue = u64;
pub type FloatValue = f64;

#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    Eof,

    // Literals
    Ident,
    StrLit(String),
    IntLit(IntValue),
    FloatLit(FloatValue),

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
    Semi,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub raw: String,
    pub location: Location,
}
