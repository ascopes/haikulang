use ariadne::{Color, Config, Label, Report, ReportKind, Source};
use haikulang_parser::parser::error::{ErrorReporter, ParserError};
use haikulang_parser::span::Span;

pub struct AriadneErrorReporter {
    errors: Vec<(ParserError, Span)>,
}

impl AriadneErrorReporter {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    pub fn print(&self, file: &str, content: &str) -> bool {
        if self.errors.is_empty() {
            return false;
        }

        let mut reporter = Report::build(ReportKind::Error, (file, (0..content.len())))
            .with_message(format!("File {} contains errors", file))
            .with_config(
                Config::new()
                    .with_compact(false)
                    .with_tab_width(4)
                    .with_multiline_arrows(true)
                    .with_underlines(true),
            );

        for (error, span) in &self.errors {
            let label = Label::new((file, span.range()))
                .with_message(format!("{}", error))
                .with_color(Color::BrightRed);
            reporter = reporter.with_label(label);
        }

        reporter
            .finish()
            .print((file, Source::from(content)))
            .unwrap();

        return true;
    }
}

impl ErrorReporter for AriadneErrorReporter {
    fn report(&mut self, error: ParserError, span: Span) {
        self.errors.push((error, span));
    }
}
