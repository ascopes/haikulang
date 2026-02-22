use crate::error_reporting::{report_io_error, report_source_error};
use clap::Args;
use haikulang_parser::lexer::token_stream::TokenStream;
use haikulang_parser::parser::core::Parser;
use std::fs::read_to_string;
use std::path::PathBuf;

#[derive(Args)]
pub struct ParserCommand {
    file: PathBuf,
}

pub fn invoke_parser(args: ParserCommand) {
    let path = args.file.as_path();
    let source = match read_to_string(path) {
        Ok(source) => source,
        Err(err) => report_io_error(path.to_str().unwrap(), err),
    };
    let token_stream = TokenStream::new(&source);
    let mut parser = Parser::new(token_stream, path);

    match parser.parse() {
        Ok(ast) => println!("{:?}", ast),
        Err(err) => report_source_error(path.to_str().unwrap(), &source, err),
    }
}
