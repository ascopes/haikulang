#[derive(Clone, Debug, Default, PartialEq)]
pub enum LexerError {
    InvalidStringLit(String),
    InvalidIntLit(String),
    InvalidFloatLit(String),

    #[default]
    Unspecified,
}
