use crate::error_reporting::AriadneErrorReporter;
use clap::Args;
use haikulang_llvm::codegen::test_codegen;
use haikulang_parser::lexer::token_stream::TokenStream;
use haikulang_parser::parser::core::Parser;
use std::fs::read_to_string;
use std::path::PathBuf;

#[derive(Args)]
pub struct LlvmCommand {
    file: PathBuf,
}

pub fn invoke_llvm(args: LlvmCommand) {
    let path = args.file.as_path();
    let source = read_to_string(path).unwrap();
    let token_stream = TokenStream::new(&source);
    let mut error_reporter = AriadneErrorReporter::new();
    let mut parser = Parser::new(token_stream, path, &mut error_reporter);
    let expr = parser.parse_expr().unwrap();
    test_codegen(&expr);
}
