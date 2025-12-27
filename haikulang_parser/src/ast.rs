use haikulang_lexer::token::Token;

enum AstNode {
    BinaryOp {
        left_value: Box<Self>,
        op: Token,
        right_value: Box<Self>,
    },
    UnaryOp {
        op: Token,
        value: Box<Self>,
    },
}
