mod error_reporting;
mod lexer_cmd;
mod parser_cmd;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct MainCommand {
    // The command to invoke.
    #[command(subcommand)]
    command: MainSubCommand,
}

#[derive(Subcommand)]
enum MainSubCommand {
    /// Invoke the lexer across a given file and show the token stream output.
    Lexer(lexer_cmd::LexerCommand),

    /// Invoke the parser across a given file and show the AST output.
    Parser(parser_cmd::ParserCommand),
}

fn main() {
    let cli = MainCommand::parse();

    match cli.command {
        MainSubCommand::Lexer(args) => lexer_cmd::invoke_lexer(args),
        MainSubCommand::Parser(args) => parser_cmd::invoke_parser(args),
    }
}
