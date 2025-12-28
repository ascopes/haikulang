use haikulang_lexer::token::Token;

pub enum AstNode {
    BinaryOp(Box<Self>, Token, Box<Self>),
    UnaryOp(Token, Box<Self>),
    Literal(Token),
}
