use haikulang_lexer::lexer::{Lexer, LexerResult};

pub struct Parser<'a> {
    lexer: &'a mut Lexer<'a>,
    current_token: LexerResult,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer<'a>) -> Self {
        let current_token = lexer.next_token();
        Self {
            lexer,
            current_token,
        }
    }
}
