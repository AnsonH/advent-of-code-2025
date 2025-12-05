use anyhow::Result;
use grid::*;

/// Creates a 2D [Grid] from a string input, where each row is separated by new line. Each character
/// is parsed by `parser` to convert it to type `T`.
///
/// # Example
///
/// ```
/// use advent_of_code_2025::grid::parse_string_to_grid;
/// use grid::*;
///
/// let input = "0123\n4567";
/// let grid = parse_string_to_grid(input, |ch| Ok(ch.to_digit(10).unwrap()));
/// assert_eq!(grid.unwrap(), grid![[0, 1, 2, 3][4, 5, 6, 7]]);
/// ```
pub fn parse_string_to_grid<T, F>(input: &str, parser: F) -> Result<Grid<T>>
where
    F: Fn(char) -> Result<T>,
{
    let lines: Vec<&str> = input.lines().collect();
    let width = lines.first().map(|line| line.len()).unwrap_or_default();

    if lines.iter().skip(1).any(|line| line.len() != width) {
        return Err(anyhow::anyhow!("Width of each line should be equal"));
    }

    let cells: Vec<T> = lines
        .iter()
        .flat_map(|line| line.chars().map(&parser).collect::<Vec<Result<T>>>())
        .collect::<Result<Vec<T>>>()?;

    Ok(Grid::from_vec(cells, width))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[derive(Debug, PartialEq)]
    enum Digit {
        Zero,
        One,
    }

    fn char_to_digit(ch: char) -> Result<Digit> {
        match ch {
            '0' => Ok(Digit::Zero),
            '1' => Ok(Digit::One),
            _ => Err(anyhow::anyhow!("Invalid character")),
        }
    }

    #[test]
    fn test_parse_string_to_grid() {
        let valid_input = "0011\n0101";
        let grid = parse_string_to_grid(valid_input, char_to_digit);
        assert!(grid.is_ok());
        assert_eq!(
            grid.unwrap(),
            grid![
                [Digit::Zero, Digit::Zero, Digit::One, Digit::One]
                [Digit::Zero, Digit::One, Digit::Zero, Digit::One]
            ]
        );

        let empty_input = "";
        let grid = parse_string_to_grid(empty_input, char_to_digit);
        assert!(grid.is_ok());
        assert_eq!(grid.unwrap(), Grid::default());

        let imbalanced_input = "0011\n1";
        let grid = parse_string_to_grid(imbalanced_input, char_to_digit);
        assert!(grid.is_err());
    }
}
