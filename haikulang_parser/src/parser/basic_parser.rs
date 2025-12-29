use crate::lexer::Lexer;
use crate::parser::{Parser, ParserResult};

pub(super) struct BasicParser<'a> {
    lexer: &'a dyn Lexer,
}

impl<'a> Parser for BasicParser<'a> {
    fn parse(&mut self) -> ParserResult {
        todo!()
    }
}

impl<'a> BasicParser<'a> {
    pub(super) fn new(lexer: &'a impl Lexer) -> Self {
        BasicParser { lexer }
    }
}
