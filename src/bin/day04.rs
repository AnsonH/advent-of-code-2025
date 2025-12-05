use std::{fmt::Display, fs, vec};

use advent_of_code_2025::{Part, grid::parse_string_to_grid};
use anyhow::{Error, Result};
use grid::*;
use itertools::iproduct;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    Empty,
    Roll,
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Empty => write!(f, "."),
            Cell::Roll => write!(f, "@"),
        }
    }
}

impl TryFrom<char> for Cell {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Cell::Empty),
            '@' => Ok(Cell::Roll),
            _ => Err(anyhow::anyhow!("Invalid cell character '{value}'")),
        }
    }
}

/// A paper roll is "accessible" if the number of paper rolls adjacent to it is smaller than or equal
/// to this number.
const ACCESSIBLE_ROLL_MAX_ADJACENCY: usize = 3;

/// Converts a cell grid to a string. Used for debugging purposes.
#[allow(dead_code)]
fn grid_to_string(grid: &Grid<Cell>) -> String {
    grid.iter_rows()
        .map(|row| row.map(|cell| cell.to_string()).collect::<String>())
        .collect::<Vec<_>>()
        .join("\n")
}

/// Counts the number of paper rolls adjacent to a cell of coordinates `(row, col)`.
fn count_adjacent_rolls(grid: &Grid<Cell>, row: usize, col: usize) -> usize {
    iproduct!(-1..=1, -1..=1)
        .filter(|&(dy, dx)| (dy, dx) != (0, 0))
        .map(|(dy, dx)| {
            // Ignore out-of-bounds cell (i.e. index < 0)
            let Some(new_row) = row.checked_add_signed(dy) else {
                return false;
            };
            let Some(new_col) = col.checked_add_signed(dx) else {
                return false;
            };
            grid.get(new_row, new_col) == Some(&Cell::Roll)
        })
        .filter(|&has_roll| has_roll)
        .count()
}

/// Keeps removing "accessible" paper rolls from a grid until there are no further accessible paper
/// rolls can be removed or it hits the `max_rounds` limit.
///
/// Pass `None` to `max_rounds` to make it infinitely loop until all accessible paper rolls are removed.
///
/// Returns a list of number of paper rolls removed in each iteration.
fn remove_accessible_rolls(initial_grid: &Grid<Cell>, max_rounds: Option<usize>) -> Vec<usize> {
    let grid: &mut Grid<Cell> = &mut initial_grid.clone();
    let mut round = 0_usize;
    let mut removed_rolls_counts: Vec<usize> = vec![];

    while max_rounds.is_none_or(|max| round < max) {
        // println!("{}\n\n", grid_to_string(grid));

        let accessible_rolls_coords: Vec<(usize, usize)> = grid
            .indexed_iter()
            .filter_map(|((row, col), &cell)| {
                let is_accessible = cell == Cell::Roll
                    && count_adjacent_rolls(grid, row, col) <= ACCESSIBLE_ROLL_MAX_ADJACENCY;
                if is_accessible {
                    Some((row, col))
                } else {
                    None
                }
            })
            .collect();

        let removed_rolls_count = accessible_rolls_coords.len();
        removed_rolls_counts.push(removed_rolls_count);

        if removed_rolls_count == 0 {
            break;
        }

        // Remove the accessible rolls
        accessible_rolls_coords.iter().for_each(|&(row, col)| {
            if let Some(cell) = grid.get_mut(row, col) {
                *cell = Cell::Empty;
            }
        });

        round += 1;
    }

    removed_rolls_counts
}

/// Day 4: Printing Department
///
/// - Part One: Find the total number of "accessible" paper rolls from the grid
/// - Part Two: Keep removing "accessible" paper rolls until no rolls can be removed, and find the
///   total number of rolls removed
fn solve_day04(grid: &Grid<Cell>, part: Part) -> usize {
    let max_rounds = match part {
        Part::One => Some(1),
        Part::Two => None,
    };
    remove_accessible_rolls(grid, max_rounds).iter().sum()
}

fn main() -> Result<()> {
    let input = fs::read_to_string("puzzle_inputs/day04.txt")?;
    let input = input.trim();
    let grid = parse_string_to_grid(input, Cell::try_from)?;

    let part_1_solution = solve_day04(&grid, Part::One);
    let part_2_solution = solve_day04(&grid, Part::Two);
    println!("Part 1 Solution: {part_1_solution}");
    println!("Part 2 Solution: {part_2_solution}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_input_to_grid() {
        let input = "..@.\n@@.@";
        let grid = parse_string_to_grid(input, Cell::try_from);
        assert!(grid.is_ok());
        assert_eq!(
            grid.unwrap(),
            grid![
                [Cell::Empty, Cell::Empty, Cell::Roll, Cell::Empty]
                [Cell::Roll, Cell::Roll, Cell::Empty, Cell::Roll]
            ],
        );
    }

    #[test]
    fn test_count_adjacent_rolls() {
        let grid = grid![
            [Cell::Empty, Cell::Empty, Cell::Roll, Cell::Roll]
            [Cell::Roll, Cell::Roll, Cell::Roll, Cell::Roll]
            [Cell::Roll, Cell::Empty, Cell::Roll, Cell::Empty]
            [Cell::Roll, Cell::Roll, Cell::Roll, Cell::Roll]
        ];
        assert_eq!(count_adjacent_rolls(&grid, 0, 0), 2);
        assert_eq!(count_adjacent_rolls(&grid, 0, 1), 4);
        assert_eq!(count_adjacent_rolls(&grid, 0, 3), 3);
        assert_eq!(count_adjacent_rolls(&grid, 2, 1), 8);
        assert_eq!(count_adjacent_rolls(&grid, 3, 0), 2);
        assert_eq!(count_adjacent_rolls(&grid, 3, 3), 2);
    }

    #[test]
    fn test_remove_accessible_rolls() {
        let grid = grid![
            [Cell::Empty, Cell::Empty, Cell::Roll, Cell::Roll]
            [Cell::Roll, Cell::Roll, Cell::Roll, Cell::Roll]
            [Cell::Roll, Cell::Empty, Cell::Roll, Cell::Empty]
            [Cell::Roll, Cell::Roll, Cell::Roll, Cell::Roll]
        ];
        assert_eq!(remove_accessible_rolls(&grid, None), vec![5, 4, 3, 0]);
        assert_eq!(remove_accessible_rolls(&grid, Some(5)), vec![5, 4, 3, 0]);
        assert_eq!(remove_accessible_rolls(&grid, Some(1)), vec![5]);

        let all_empty_grid = grid![
            [Cell::Empty, Cell::Empty, Cell::Empty]
            [Cell::Empty, Cell::Empty, Cell::Empty]
            [Cell::Empty, Cell::Empty, Cell::Empty]
        ];
        assert_eq!(remove_accessible_rolls(&all_empty_grid, None), vec![0]);
        assert_eq!(remove_accessible_rolls(&all_empty_grid, Some(5)), vec![0]);
    }

    #[test]
    fn test_solve_day04() {
        let input = r"
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
"
        .trim();

        let grid = parse_string_to_grid(input, Cell::try_from).unwrap();

        remove_accessible_rolls(&grid, None);

        assert_eq!(solve_day04(&grid, Part::One), 13);
        assert_eq!(solve_day04(&grid, Part::Two), 43);
    }
}
