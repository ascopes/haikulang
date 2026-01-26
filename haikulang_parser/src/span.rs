use std::fmt;

#[derive(Clone, Copy, Debug)]
pub struct Span {
    start: usize,
    end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }

    pub fn to(&self, other: Self) -> Self {
        Self::new(self.start, other.end)
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.start, self.end)
    }
}

#[derive(Clone, Debug)]
pub struct Spanned<T: Clone> {
    value: T,
    span: Span,
}

impl<T: Clone> Spanned<T> {
    pub fn new(value: T, span: Span) -> Self {
        Self { value, span }
    }

    pub fn value(&self) -> T {
        self.value.clone()
    }

    pub fn span(&self) -> Span {
        self.span
    }
}
