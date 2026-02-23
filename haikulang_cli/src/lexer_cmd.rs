use clap::Args;
use haikulang_parser::lexer::token::Token;
use haikulang_parser::lexer::token_stream::TokenStream;
use std::fs::read_to_string;
use std::path::PathBuf;

#[derive(Args)]
pub struct LexerCommand {
    file: PathBuf,
}

pub fn invoke_lexer(args: LexerCommand) {
    let path = args.file.as_path();
    let source = read_to_string(path).unwrap();
    let mut token_stream = TokenStream::new(&source);

    loop {
        match token_stream.current() {
            Ok(token) => {
                println!("{}: {:?}", token.span(), token.value());

                if token.value() == Token::Eof {
                    return;
                } else {
                    token_stream.advance();
                }
            }
            Err(err) => panic!("{:?}", err),
        }
    }
}
