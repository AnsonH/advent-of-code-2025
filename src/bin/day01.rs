use std::fs;
use thiserror::Error;

use advent_of_code_2025::Part;
use anyhow::Result;

#[derive(Error, Debug, PartialEq)]
enum SolverError {
    #[error("the input '{0}' is invalid")]
    InvalidInput(String),
}

#[derive(Debug, Clone, PartialEq)]
enum Direction {
    Left,
    Right,
}

const INITIAL_DIAL_POSITION: isize = 50;
const DIAL_LENGTH: isize = 100;

/// Day 1: Secret Entrance
///
/// - Part One: Only counts number of times dial points to `0` at the end of each move.
/// - Part Two: Counts number of times the dial hits `0` during a rotation or end of one.
fn solve_day01(input: &str, part: Part) -> Result<isize, SolverError> {
    let rotations: Vec<&str> = input.lines().filter(|&line| !line.is_empty()).collect();

    let mut dial_position = INITIAL_DIAL_POSITION;
    let mut final_pos_zero_hit_count = 0;
    let mut total_zero_hit_count = 0;

    for rotation in rotations {
        let direction = match &rotation[..1] {
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => return Err(SolverError::InvalidInput(rotation.into())),
        };
        let distance = rotation[1..]
            .parse::<isize>()
            .map_err(|_| SolverError::InvalidInput(rotation.into()))?;

        let (new_dial_position, zero_hits) = turn_dial(dial_position, direction, distance);
        // println!(
        //     "The dial is rotated {rotation} to point at {dial_position}, hits zero for {zero_hits} times"
        // );

        if dial_position == 0 {
            final_pos_zero_hit_count += 1;
        }
        total_zero_hit_count += zero_hits;

        dial_position = new_dial_position;
    }

    match part {
        Part::One => Ok(final_pos_zero_hit_count),
        Part::Two => Ok(total_zero_hit_count),
    }
}

/// Turns the dial from starting position `start_pos` in `direction` for a number
/// of `distance`.
///
/// Returns a tuple of `(final_pos, zero_hits)`:
/// - `final_pos` - Final position of the pin
/// - `zero_hits` - Total number of times `0` is hit during rotation
///   - Note: `start_pos = 0` alone does not count as hitting zero
fn turn_dial(start_pos: isize, direction: Direction, distance: isize) -> (isize, isize) {
    let raw_final_pos: isize = match direction {
        Direction::Left => start_pos - distance,
        Direction::Right => start_pos + distance,
    };

    let final_pos = (DIAL_LENGTH + raw_final_pos % DIAL_LENGTH) % DIAL_LENGTH;

    let mut zero_hits = (raw_final_pos / DIAL_LENGTH).abs();
    if (start_pos > 0 && raw_final_pos < 0) || raw_final_pos == 0 {
        zero_hits += 1;
    }

    (final_pos, zero_hits)
}

fn main() -> Result<()> {
    let input = fs::read_to_string("puzzle_inputs/day01.txt")?;
    let part_1_solution = solve_day01(&input, Part::One)?;
    let part_2_solution = solve_day01(&input, Part::Two)?;
    println!("Part 1 Solution: {part_1_solution}");
    println!("Part 2 Solution: {part_2_solution}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_turn_dial() {
        // No overflow
        assert_eq!(turn_dial(11, Direction::Right, 8), (19, 0));
        assert_eq!(turn_dial(19, Direction::Left, 19), (0, 1));
        assert_eq!(turn_dial(5, Direction::Left, 5), (0, 1));
        assert_eq!(turn_dial(0, Direction::Right, 5), (5, 0));

        // Overflow x 1
        assert_eq!(turn_dial(5, Direction::Left, 10), (95, 1));
        assert_eq!(turn_dial(95, Direction::Right, 5), (0, 1));
        assert_eq!(turn_dial(0, Direction::Left, 5), (95, 0));
        assert_eq!(turn_dial(0, Direction::Left, 100), (0, 1));
        assert_eq!(turn_dial(0, Direction::Right, 100), (0, 1));

        // Multiple overflows
        assert_eq!(turn_dial(50, Direction::Right, 200), (50, 2));
        assert_eq!(turn_dial(50, Direction::Left, 201), (49, 2));
        assert_eq!(turn_dial(50, Direction::Right, 150), (0, 2));
        assert_eq!(turn_dial(50, Direction::Left, 150), (0, 2));
        assert_eq!(turn_dial(0, Direction::Right, 200), (0, 2));
        assert_eq!(turn_dial(0, Direction::Left, 200), (0, 2));
        assert_eq!(turn_dial(0, Direction::Left, 150), (50, 1));
        assert_eq!(turn_dial(50, Direction::Right, 1000), (50, 10));

        // Example input
        assert_eq!(turn_dial(50, Direction::Left, 68), (82, 1));
        assert_eq!(turn_dial(82, Direction::Left, 30), (52, 0));
        assert_eq!(turn_dial(52, Direction::Right, 48), (0, 1));
        assert_eq!(turn_dial(0, Direction::Left, 5), (95, 0));
        assert_eq!(turn_dial(95, Direction::Right, 60), (55, 1));
        assert_eq!(turn_dial(55, Direction::Left, 55), (0, 1));
        assert_eq!(turn_dial(0, Direction::Left, 1), (99, 0));
        assert_eq!(turn_dial(99, Direction::Left, 99), (0, 1));
        assert_eq!(turn_dial(0, Direction::Right, 14), (14, 0));
        assert_eq!(turn_dial(14, Direction::Left, 82), (32, 1));
    }

    #[test]
    fn test_example_input() {
        let input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";

        assert_eq!(solve_day01(input, Part::One), Ok(3));
        assert_eq!(solve_day01(input, Part::Two), Ok(6));
    }
}
