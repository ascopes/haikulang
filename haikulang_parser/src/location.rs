use std::fmt::{Display, Formatter};

pub type Position = usize;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Location {
    pub offset: Position,
    pub line: Position,
    pub column: Position,
}

impl Default for Location {
    fn default() -> Self {
        Self {
            offset: 0,
            line: 1,
            column: 1,
        }
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn default_has_expected_value() {
        // Given
        let expected_location = Location {
            offset: 0,
            line: 1,
            column: 1,
        };

        // When
        let location = Location::default();

        // Then
        assert_eq!(expected_location, location);
    }

    #[test_case( 0,   1,  2,    "1:2" ; "start of file")]
    #[test_case(67, 420, 69, "420:69" ; "middle of file")]
    fn display_is_formatted_as_expected(
        input_offset: Position,
        input_line: Position,
        input_column: Position,
        expected: &str,
    ) {
        // Given
        let location = Location {
            offset: input_offset,
            line: input_line,
            column: input_column,
        };

        // Then
        assert_eq!(expected, location.to_string());
    }
}
