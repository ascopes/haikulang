use crate::error_reporting::AriadneErrorReporter;
use clap::Args;
use haikulang_parser::lexer::token_stream::TokenStream;
use haikulang_parser::parser::core::Parser;
use std::fs::read_to_string;
use std::path::PathBuf;
use std::process::exit;

#[derive(Args)]
pub struct ParserCommand {
    file: PathBuf,
}

pub fn invoke_parser(args: ParserCommand) {
    let path = args.file.as_path();
    let source = read_to_string(path).unwrap();
    let token_stream = TokenStream::new(&source);
    let mut error_reporter = AriadneErrorReporter::new();
    let mut parser = Parser::new(token_stream, path, &mut error_reporter);

    match parser.parse() {
        Ok(ast) => println!("{:#?}", ast),
        // Errors imply reporting took place, handle that below.
        Err(err) => eprintln!("Encountered an error {}", err.value()),
    };

    if error_reporter.print(path.to_str().unwrap(), &source) {
        exit(2);
    }
}
