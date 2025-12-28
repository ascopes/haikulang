use haikulang_lexer::token::Token;

enum AstNode {
    BinaryOp(Box<Self>, Token, Box<Self>),
    UnaryOp(Token, Box<Self>),
    Literal(Token),
}
