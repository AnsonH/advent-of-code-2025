//! Utilities for parsing strings.

use std::ops::RangeInclusive;

/// Parses a number range string like `5-10` into a [RangeInclusive] range.
///
/// # Panic
///
/// Panics if the format is invalid or the number parsing failed.
///
/// # Example
///
/// ```
/// use advent_of_code_2025::parse::parse_u64_number_range;
///
/// assert_eq!(parse_u64_number_range("5-10"), 5..=10);
/// ```
pub fn parse_u64_number_range(input: &str) -> RangeInclusive<u64> {
    let (start, end) = input
        .split_once('-')
        .expect("input range should be delimited by `-`");
    start.parse().unwrap()..=end.parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_u64_number_range() {
        assert_eq!(parse_u64_number_range("5-10"), 5..=10);
        assert_eq!(
            parse_u64_number_range("404919393645906-405195345919978"),
            404919393645906..=405195345919978
        );
    }
}
