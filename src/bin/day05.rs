use std::{fs, ops::RangeInclusive};

use advent_of_code_2025::{Part, parse::parse_u64_number_range};
use anyhow::Result;
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq)]
struct Database {
    /// A list of fresh ingredient ID ranges.
    fresh_id_ranges: Vec<RangeInclusive<u64>>,
    /// A list of available ingredient IDs.
    available_ids: Vec<u64>,
}

impl Database {
    fn new(fresh_id_ranges: Vec<RangeInclusive<u64>>, available_ids: Vec<u64>) -> Self {
        Self {
            fresh_id_ranges,
            available_ids,
        }
    }
}

fn parse_input_to_database(input: &str) -> Database {
    let parts: Vec<Vec<&str>> = input
        .lines()
        .collect::<Vec<&str>>()
        .split(|line| line.is_empty())
        .map(|part| part.to_vec())
        .collect();

    let (fresh_ids_strings, available_ids_strings) = (&parts[0], &parts[1]);

    let fresh_id_ranges: Vec<RangeInclusive<u64>> = fresh_ids_strings
        .iter()
        .map(|&range_str| parse_u64_number_range(range_str))
        .collect();

    let available_ids: Vec<u64> = available_ids_strings
        .iter()
        .map(|&id| id.parse().unwrap())
        .collect();

    Database::new(fresh_id_ranges, available_ids)
}

/// Merges overlapping ranges together, then sort the ranges by ascending order of the range's start.
///
/// # Example
///
/// Let's say we have `1-4, 7-9, 6-11, 10-13`. We first sort it in ascending order of each range's start.
/// Then, we lay them on the number line:
///
/// ``````txt
/// 1----4
///         6---------------11
///            7------9
///                      10--------13
/// ``````
/// A range overlaps with previous one if this range's start <= last range's end.
/// Therefore, the final merged range is `vec![1..=4, 6..=13]`.
///
/// Visualization: https://youtu.be/hG9QDwiE28w
fn sort_and_merge_ranges(input: &[RangeInclusive<u64>]) -> Vec<RangeInclusive<u64>> {
    input
        .iter()
        .sorted_by_key(|range| range.start())
        .fold(vec![], |mut output, range| {
            match output.last_mut() {
                Some(last_range) if range.start() <= last_range.end() => {
                    *last_range = *last_range.start()..=*range.end().max(last_range.end());
                }
                _ => output.push(range.clone()),
            }
            output
        })
}

fn optimize_database(database: Database) -> Database {
    let optimized_fresh_id_ranges = sort_and_merge_ranges(&database.fresh_id_ranges);
    Database::new(optimized_fresh_id_ranges, database.available_ids)
}

/// From the list of available IDs, count how may of them are within the fresh ID ranges.
fn count_fresh_ids_from_available(database: &Database) -> u64 {
    database
        .available_ids
        .iter()
        .map(|id| {
            database
                .fresh_id_ranges
                .iter()
                .any(|range| range.contains(id))
        })
        .filter(|is_fresh| *is_fresh)
        .count() as u64
}

/// Counts total number of IDs that are fresh.
///
/// NOTE: The database should be optimized so that the `fresh_id_ranges` are sorted and merged.
/// Otherwise double counting may happen.
fn count_all_fresh_ids(optimized_database: &Database) -> u64 {
    optimized_database
        .fresh_id_ranges
        .iter()
        .map(|range| range.end() - range.start() + 1)
        .sum()
}

fn solve_day05(input: &str, part: Part) -> u64 {
    let raw_database = parse_input_to_database(input);
    let optimized_database = optimize_database(raw_database);

    match part {
        Part::One => count_fresh_ids_from_available(&optimized_database),
        Part::Two => count_all_fresh_ids(&optimized_database),
    }
}

fn main() -> Result<()> {
    let input = fs::read_to_string("puzzle_inputs/day05.txt")?;
    let input = input.trim();

    let part_1_solution = solve_day05(input, Part::One);
    let part_2_solution = solve_day05(input, Part::Two);
    println!("Part 1 Solution: {part_1_solution}");
    println!("Part 2 Solution: {part_2_solution}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_input_to_database() {
        let input = r"
3-5
10-19
404919393645906-405195345919978

4
102
12345678901234
"
        .trim();
        assert_eq!(
            parse_input_to_database(input),
            Database::new(
                vec![3..=5, 10..=19, 404919393645906..=405195345919978],
                vec![4, 102, 12345678901234]
            )
        );
    }

    #[test]
    fn test_sort_and_merge_ranges() {
        assert_eq!(sort_and_merge_ranges(&[]), vec![]);
        assert_eq!(sort_and_merge_ranges(&[1..=5]), vec![1..=5]);
        assert_eq!(
            sort_and_merge_ranges(&[1..=5, 12..=16, 8..=10]),
            vec![1..=5, 8..=10, 12..=16]
        );
        assert_eq!(
            sort_and_merge_ranges(&[1..=5, 7..=12, 6..=8, 19..=26, 12..=13, 21..=25]),
            vec![1..=5, 6..=13, 19..=26]
        );
    }

    #[test]
    fn test_solve_day05() {
        let input = r"
3-5
10-14
16-20
12-18

1
5
8
11
17
32
        "
        .trim();

        assert_eq!(solve_day05(input, Part::One), 3);
        assert_eq!(solve_day05(input, Part::Two), 14);
    }
}
