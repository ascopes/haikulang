use crate::error_reporting::AriadneErrorReporter;
use clap::Args;
use haikulang_parser::error::ErrorReporter;
use haikulang_parser::lexer::token::Token;
use haikulang_parser::lexer::token_stream::TokenStream;
use std::fs::read_to_string;
use std::path::PathBuf;
use std::process::exit;

#[derive(Args)]
pub struct LexerCommand {
    file: PathBuf,
}

pub fn invoke_lexer(args: LexerCommand) {
    let path = args.file.as_path();
    let source = read_to_string(path).unwrap();
    let mut token_stream = TokenStream::new(&source);
    let mut error_reporter = AriadneErrorReporter::new();

    let mut index = 0;
    loop {
        match token_stream.current() {
            Ok(token) => {
                println!("{}: {}: {:?}", index, token.span(), token.value());
                index += 1;

                if token.value() == Token::Eof {
                    break;
                } else {
                    token_stream.advance();
                }
            }
            Err(err) => error_reporter.report(&err),
        }
    }

    if error_reporter.print(path.to_str().unwrap(), &source) {
        exit(2);
    }
}
