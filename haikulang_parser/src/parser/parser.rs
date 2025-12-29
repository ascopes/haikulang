use super::ast::AstNode;
use crate::lexer::Lexer;
use crate::parser::ParserError;
use crate::parser::basic_parser::BasicParser;

pub type ParserResult = Result<AstNode, ParserError>;

pub trait Parser {
    fn parse(&mut self) -> ParserResult;
}

pub fn new_default_parser(lexer: &impl Lexer) -> impl Parser {
    BasicParser::new(lexer)
}
