use ariadne::{Color, Config, Label, Report, ReportKind, Source};
use haikulang_parser::span::Spanned;
use std::fmt::Display;
use std::io;
use std::process::exit;

pub fn report_io_error<'a>(file: &str, error: io::Error) -> ! {
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

pub fn report_source_error<E: Clone + Display>(file: &str, content: &str, error: Spanned<E>) -> ! {
    Report::build(ReportKind::Error, (file, error.span().range()))
        .with_message(format!("File {} contains errors", file))
        .with_label(
            Label::new((file, error.span().range()))
                .with_message(format!("{}", error.value()))
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
