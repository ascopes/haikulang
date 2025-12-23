use crate::location::Location;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Eof,

    // Error types
    Unknown,
    MalformedLiteral(String, Location),

    // Literals
    Ident,
    Str(String),
    Int(u64),
    Float(f64),

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
    pub raw: String,
    pub location: Location,
}
