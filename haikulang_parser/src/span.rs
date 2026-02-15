use std::fmt;
use std::ops::Range;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Span {
    start: usize,
    end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        debug_assert!(
            start <= end,
            "span start ({}) was greater than span end ({})",
            start,
            end
        );
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

    pub fn range(&self) -> Range<usize> {
        self.start..self.end
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.start, self.end)
    }
}

#[derive(Clone, Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn span_constructs_correctly() {
        // Given
        let span = Span::new(19, 27);

        // Then
        assert_eq!(span.start, 19);
        assert_eq!(span.start(), span.start);
        assert_eq!(span.end, 27);
        assert_eq!(span.end(), span.end);
    }

    #[test]
    fn span_expands_correctly() {
        // Given
        let span1 = Span::new(19, 27);
        let span2 = Span::new(35, 42);

        // When
        let span3 = span1.to(span2);

        // Then
        assert_eq!(span3.start(), 19);
        assert_eq!(span3.end(), 42);
    }

    #[test]
    fn span_produces_correct_range() {
        // Given
        let span = Span::new(19, 27);

        // When
        let range = span.range();

        // Then
        assert_eq!(range.start, 19);
        assert_eq!(range.end, 27);
        assert_eq!(range, 19..27);
    }

    #[test]
    fn span_formats_correctly() {
        // Given
        let span = Span::new(19, 27);

        // Then
        assert_eq!(format!("{}", span), "19:27");
    }

    #[test]
    fn spanned_constructs_correctly() {
        // Given
        #[derive(Clone, Copy, Debug, PartialEq)]
        struct Something {
            a: i16,
            b: i32,
        }

        let object = Something { a: 9, b: 18 };
        let span = Span::new(8, 24);

        // When
        let spanned = Spanned::new(object, span);

        // Then
        assert_eq!(spanned.value, object);
        assert_eq!(spanned.value(), spanned.value);
        assert_eq!(spanned.span, span);
        assert_eq!(spanned.span(), spanned.span);
    }
}
