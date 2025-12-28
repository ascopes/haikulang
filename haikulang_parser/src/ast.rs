use haikulang_lexer::token::Token;

#[derive(Debug, PartialEq)]
pub enum AstNode {
    BinaryOp(Box<Self>, Token, Box<Self>),
    UnaryOp(Token, Box<Self>),
    Literal(Token),
}
