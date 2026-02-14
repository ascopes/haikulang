use ariadne::{Color, Config, Label, Report, ReportKind, Source};
use haikulang_parser::ast::stmt::Statement;
use haikulang_parser::lexer::token_stream::TokenStream;
use haikulang_parser::parser::error::ParserError;
use haikulang_parser::parser::parser::{Parser, ParserResult};
use haikulang_parser::span::Spanned;
use std::env::args;
use std::process::exit;
use std::{fs, io};

fn main() {
    let files: Vec<String> = args().skip(1).collect();

    if files.is_empty() {
        report_no_args();
    }

    for file in files {
        match fs::read_to_string(&file) {
            Ok(content) => match parse(content.as_str()) {
                Ok(ast) => println!("{:?}", ast),
                Err(err) => report_parser_error(file.as_str(), content.as_str(), err),
            },
            Err(err) => report_io_error(file.as_str(), err),
        }
    }
}

fn parse(file: &str) -> ParserResult<Statement> {
    let tokens = TokenStream::new(file);
    let mut parser = Parser::new(tokens);
    parser.parse_statement()
}

fn report_parser_error(file: &str, content: &str, parser_error: Spanned<ParserError>) {
    Report::build(ReportKind::Error, (file, parser_error.span().range()))
        .with_message(format!("File {} contains errors", file))
        .with_label(
            Label::new((file, parser_error.span().range()))
                .with_message(format!("{}", parser_error.value()))
                .with_color(Color::BrightRed),
        )
        .with_config(
            Config::new()
                .with_compact(false)
                .with_tab_width(4)
                .with_multiline_arrows(true)
                .with_underlines(true),
        )
        .finish()
        .print((file, Source::from(content)))
        .unwrap();
    exit(3);
}

fn report_io_error(file: &str, error: io::Error) {
    Report::build(ReportKind::Error, (file, 0..0))
        .with_message(format!("IO error reading {}: {}", file, error))
        .with_config(
            Config::new()
                .with_compact(false)
                .with_tab_width(4)
                .with_multiline_arrows(true)
                .with_underlines(true),
        )
        .finish()
        .print((file, Source::from("")))
        .unwrap();
    exit(2);
}

fn report_no_args() {
    eprintln!("No files were provided in arguments");
    exit(1);
}
