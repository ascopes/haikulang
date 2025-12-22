use super::location::Location;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Eof,

    // Literals
    Ident,
    String,
    Number,

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

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub data: String,
    pub location: Location,
}
