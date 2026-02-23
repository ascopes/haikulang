use crate::error_reporting::AriadneErrorReporter;
use clap::Args;
use haikulang_parser::ast::pretty::PrettyPrinterVisitor;
use haikulang_parser::ast::visitor::Visitor;
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
        Ok(ast) => {
            let mut str = String::new();
            {
                let mut pretty_printer = PrettyPrinterVisitor::new(&mut str);
                pretty_printer
                    .visit_compilation_unit(&ast.value(), ast.span())
                    .unwrap();
            }
            println!("{}", str);
        }
        // Errors imply reporting took place, handle that below.
        Err(()) => {}
    };

    if error_reporter.print(path.to_str().unwrap(), &source) {
        exit(2);
    }
}
