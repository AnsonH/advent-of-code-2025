use std::fs;

use advent_of_code_2025::{Part, coords::Coords2D};
use anyhow::Result;
use itertools::Itertools;

fn parse_input_to_coords(input: &str) -> Vec<Coords2D> {
    input
        .lines()
        .map(|line| {
            let values: Vec<i64> = line
                .split(",")
                .map(|num_str| num_str.parse().expect("should be a valid integer"))
                .collect();
            Coords2D::new(values[0], values[1])
        })
        .collect()
}

/// Finds the largest rectangle area formed from 2 coordinates being the corners of the rectangle.
fn find_largest_rect_area(coords: &[Coords2D]) -> usize {
    coords
        .iter()
        .combinations(2)
        .map(|points| {
            let [a, b] = [points[0], points[1]];
            (a.x.abs_diff(b.x) + 1) * (a.y.abs_diff(b.y) + 1)
        })
        .max()
        .expect("coords should not be empty") as usize
}

fn solve_day09(input: &str, part: Part) -> usize {
    let coords = parse_input_to_coords(input);
    match part {
        Part::One => find_largest_rect_area(&coords),
        Part::Two => todo!(),
    }
}

fn main() -> Result<()> {
    let input = fs::read_to_string("puzzle_inputs/day09.txt")?;

    let part_1_solution = solve_day09(&input, Part::One);
    // let part_2_solution = solve_day09(&input, Part::Two);
    println!("Part 1 Solution: {part_1_solution}");
    // println!("Part 2 Solution: {part_2_solution}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_input_to_coords() {
        let input = "162,817\n57,618";
        assert_eq!(
            parse_input_to_coords(input),
            vec![Coords2D::new(162, 817), Coords2D::new(57, 618)]
        )
    }

    #[test]
    fn test_find_largest_rect_area() {
        let coords = [
            Coords2D::new(7, 1),
            Coords2D::new(11, 1),
            Coords2D::new(11, 7),
            Coords2D::new(9, 7),
            Coords2D::new(9, 5),
            Coords2D::new(2, 5),
            Coords2D::new(2, 3),
            Coords2D::new(7, 3),
        ];
        assert_eq!(find_largest_rect_area(&coords), 50);
    }
}
