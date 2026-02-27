use ariadne::{Color, Config, Label, Report, ReportKind, Source};
use haikulang_parser::error::{ErrorReporter, ParserError};
use haikulang_parser::span::Spanned;

pub struct AriadneErrorReporter {
    errors: Vec<Spanned<ParserError>>,
}

impl AriadneErrorReporter {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    pub fn print(&self, file: &str, content: &str) -> bool {
        if self.errors.is_empty() {
            return false;
        }

        for error in &self.errors {
            let mut reporter = Report::build(ReportKind::Error, (file, error.span().range()))
                .with_message(format!("{}", error.value()))
                .with_config(
                    Config::new()
                        .with_compact(false)
                        .with_tab_width(4)
                        .with_multiline_arrows(true)
                        .with_underlines(true),
                );

            let label = Label::new((file, error.span().range()))
                .with_message("error occurred here!")
                .with_color(Color::BrightRed);
            reporter = reporter.with_label(label);

            reporter
                .finish()
                .print((file, Source::from(content)))
                .unwrap();
        }

        true
    }
}

impl ErrorReporter for AriadneErrorReporter {
    fn report(&mut self, error: &Spanned<ParserError>) {
        self.errors.push(error.clone());
    }
}
