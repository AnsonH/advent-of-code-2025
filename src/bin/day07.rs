use std::{fmt::Display, fs};

use advent_of_code_2025::{Part, grid::parse_string_to_grid};
use anyhow::{Error, Result};
use grid::*;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    /// Empty space (`.`)
    Empty,
    /// Starting position (`S`)
    Start,
    /// A beam splitter (`^`)
    Splitter,
    /// A beam (`|`). It holds a numeric "weight" that indicates how many path combinations can the
    /// the beam arrive here from the source.
    Beam(usize),
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Empty => write!(f, "."),
            Cell::Start => write!(f, "S"),
            Cell::Splitter => write!(f, "^"),
            Cell::Beam(_) => write!(f, "|"),
        }
    }
}

impl TryFrom<char> for Cell {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Cell::Empty),
            'S' => Ok(Cell::Start),
            '^' => Ok(Cell::Splitter),
            '|' => Ok(Cell::Beam(1)), // we don't know the actual weight of beam, so default to 1
            _ => Err(anyhow::anyhow!("Invalid cell character '{value}'")),
        }
    }
}

/// Move the beams forward by 1 row at row number `row_idx` (zero-based).
///
/// # High-Level Example
///
/// ```txt
/// ...S...                               ...S...
/// ...|...                               ...|...
/// ..|^|..  --- next_tick(&grid, 3) -->  ..|^|..
/// ..^....                               .|^||..    <- update row of index 3
/// .......                               .......
/// ```
///
/// # Beam Weights
///
/// Each beam's weight counts all possible ways a beam can travel to that cell from the start.
/// When the beam hits a splitter (`^`), its weight is duplicated. If beams overlap, their weight
/// is summed up.
///
/// Example:
///
/// ```txt
///   2 3 4    <- beam weight                                 2 3 4
/// . | | | .                  -- next_tick(&grid, 2) -->   . | | | .
/// . ^ . ^ .                                               | ^ | ^ |        
///                                                         2   9   4   <- new beam weight
///                                                             ╰─ 2 + 3 + 4
/// ```
fn next_tick(grid: &mut Grid<Cell>, row_idx: usize) -> (&Grid<Cell>, usize) {
    assert!(row_idx > 0, "row_idx should be greater than 0");

    let mut total_splits = 0;
    for col_idx in 0..grid.cols() {
        let cell = grid[(row_idx, col_idx)];
        let above_cell = grid[(row_idx.saturating_sub(1), col_idx)];

        match (above_cell, cell) {
            (Cell::Start, Cell::Empty) => {
                *grid.get_mut(row_idx, col_idx).unwrap() = Cell::Beam(1);
            }
            (Cell::Beam(weight), Cell::Empty) => {
                *grid.get_mut(row_idx, col_idx).unwrap() = Cell::Beam(weight);
            }
            (Cell::Beam(above_weight), Cell::Beam(current_weight)) => {
                *grid.get_mut(row_idx, col_idx).unwrap() =
                    Cell::Beam(above_weight + current_weight);
            }
            (Cell::Beam(above_weight), Cell::Splitter) => {
                total_splits += 1;

                let left_cell_coords = (row_idx, col_idx.saturating_sub(1));
                let right_cell_coords = (row_idx, col_idx + 1);
                for coords in [left_cell_coords, right_cell_coords] {
                    if let Some(adjacent_cell) = grid.get_mut(coords.0, coords.1) {
                        let new_weight = match *adjacent_cell {
                            Cell::Beam(existing_weight) => Some(above_weight + existing_weight),
                            Cell::Empty => Some(above_weight),
                            _ => None,
                        };
                        if let Some(w) = new_weight {
                            *adjacent_cell = Cell::Beam(w)
                        }
                    }
                }
            }
            _ => {}
        }
    }

    (grid, total_splits)
}

/// Shoots the beam from start position until the beam reaches the end, and returns the total number
/// of splits.
fn shoot_beam_and_count_splits(cell_grid: &mut Grid<Cell>) -> (&Grid<Cell>, usize) {
    let total_splits = (1..cell_grid.rows())
        .map(|row_idx| next_tick(cell_grid, row_idx).1)
        .sum();
    (cell_grid, total_splits)
}

/// Counts number of possible paths a beam can travel.
fn count_beam_possible_paths(cell_grid: &Grid<Cell>) -> usize {
    cell_grid
        .iter_rows()
        .next_back()
        .expect("grid has >=1 row")
        .map(|&cell| match cell {
            Cell::Beam(weight) => weight,
            _ => 0,
        })
        .sum()
}

fn solve_day07(input: &str, part: Part) -> usize {
    let mut cell_grid = parse_string_to_grid(input, Cell::try_from).expect("input should be valid");
    let (_, total_splits) = shoot_beam_and_count_splits(&mut cell_grid);
    match part {
        Part::One => total_splits,
        Part::Two => count_beam_possible_paths(&cell_grid),
    }
}

fn main() -> Result<()> {
    let input = fs::read_to_string("puzzle_inputs/day07.txt")?;

    let part_1_solution = solve_day07(&input, Part::One);
    let part_2_solution = solve_day07(&input, Part::Two);
    println!("Part 1 Solution: {part_1_solution}");
    println!("Part 2 Solution: {part_2_solution}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_string_to_grid() {
        let input = r"
..S..
.....
.^.^."
            .trim();
        let expected_grid = grid![
            [Cell::Empty, Cell::Empty, Cell::Start, Cell::Empty, Cell::Empty]
            [Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty]
            [Cell::Empty, Cell::Splitter, Cell::Empty, Cell::Splitter, Cell::Empty]
        ];

        let grid = parse_string_to_grid(input, Cell::try_from);
        assert!(grid.is_ok());
        assert_eq!(grid.unwrap(), expected_grid);
    }

    #[test]
    fn test_next_tick() {
        // ...
        // ...
        let mut input = grid![
            [Cell::Empty, Cell::Empty, Cell::Empty]
            [Cell::Empty, Cell::Empty, Cell::Empty]
        ];
        let expected_output = input.clone();
        assert_eq!(next_tick(&mut input, 1), (&expected_output, 0_usize));

        // .S.
        // ...
        let mut input = grid![
            [Cell::Empty, Cell::Start, Cell::Empty]
            [Cell::Empty, Cell::Empty, Cell::Empty]
        ];
        let expected_output = grid![
            [Cell::Empty, Cell::Start, Cell::Empty]
            [Cell::Empty, Cell::Beam(1), Cell::Empty]
        ];
        assert_eq!(next_tick(&mut input, 1), (&expected_output, 0_usize));

        // ..|..
        // .^.^.
        let mut input = grid![
            [Cell::Empty, Cell::Empty,    Cell::Beam(5), Cell::Empty,    Cell::Empty]
            [Cell::Empty, Cell::Splitter, Cell::Empty,   Cell::Splitter, Cell::Empty]
        ];
        let expected_output = grid![
            [Cell::Empty, Cell::Empty, Cell::Beam(5), Cell::Empty, Cell::Empty]
            [Cell::Empty, Cell::Splitter, Cell::Beam(5), Cell::Splitter, Cell::Empty]
        ];
        assert_eq!(next_tick(&mut input, 1), (&expected_output, 0_usize));

        // ..|..
        // ..^..
        let mut input = grid![
            [Cell::Empty, Cell::Empty, Cell::Beam(5), Cell::Empty, Cell::Empty]
            [Cell::Empty, Cell::Empty, Cell::Splitter, Cell::Empty, Cell::Empty]
        ];
        let expected_output = grid![
            [Cell::Empty, Cell::Empty, Cell::Beam(5), Cell::Empty, Cell::Empty]
            [Cell::Empty, Cell::Beam(5), Cell::Splitter, Cell::Beam(5), Cell::Empty]
        ];
        assert_eq!(next_tick(&mut input, 1), (&expected_output, 1_usize));

        // .|.|.
        // .^.^.
        let mut input = grid![
            [Cell::Empty, Cell::Beam(2), Cell::Empty, Cell::Beam(3), Cell::Empty]
            [Cell::Empty, Cell::Splitter, Cell::Empty, Cell::Splitter, Cell::Empty]
        ];
        let expected_output = grid![
            [Cell::Empty, Cell::Beam(2), Cell::Empty, Cell::Beam(3), Cell::Empty]
            [Cell::Beam(2), Cell::Splitter, Cell::Beam(2 + 3), Cell::Splitter, Cell::Beam(3)]
        ];
        assert_eq!(next_tick(&mut input, 1), (&expected_output, 2_usize));

        // .|||.
        // .^.^.
        // .....
        let mut input = grid![
            [Cell::Empty, Cell::Beam(2), Cell::Beam(3), Cell::Beam(5), Cell::Empty]
            [Cell::Empty, Cell::Splitter, Cell::Empty, Cell::Splitter, Cell::Empty]
            [Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty]
        ];
        let expected_output = grid![
            [Cell::Empty, Cell::Beam(2), Cell::Beam(3), Cell::Beam(5), Cell::Empty]
            [Cell::Beam(2), Cell::Splitter, Cell::Beam(2 + 3 + 5), Cell::Splitter, Cell::Beam(5)]
            [Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty]
        ];
        assert_eq!(next_tick(&mut input, 1), (&expected_output, 2_usize));
    }

    #[test]
    fn test_solve_day07() {
        // Puzzle example
        let input = r"
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."
            .trim();

        assert_eq!(solve_day07(input, Part::One), 21);
        assert_eq!(solve_day07(input, Part::Two), 40);
    }
}
