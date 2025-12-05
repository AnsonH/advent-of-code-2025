use std::{fs, ops::RangeInclusive};

use advent_of_code_2025::parse::parse_u64_number_range;
use anyhow::Result;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Part {
    /// ID is "invalid" if some digit sequence repeats twice (e.g. `6464` - `64`x2).
    One,
    /// ID is "invalid" if some digit sequence repeats at least twice (e.g. `123123123` = `123`x3)
    Two,
}

/// Parses a comma-delimited input ranges to a vector of ranges.
///
/// e.g. `parse_input("1-5,1000-1002")` -> `vec![1..=5, 1000..=1002])`
fn parse_input(input: &str) -> Vec<RangeInclusive<u64>> {
    input.split(',').map(parse_u64_number_range).collect()
}

/// Part One - Invalid if upper half of number equals to lower half (e.g. `6464`, `123123`).
///
/// Numbers of odd number of digits is always valid because it's impossible to split an
/// odd-digit number equally in half.
fn is_invalid_part_one(number: u64) -> bool {
    let num_digits = number.ilog10() + 1;
    if num_digits % 2 == 1 {
        false
    } else {
        let upper_half = number / 10_u64.pow(num_digits / 2);
        let lower_half = number % 10_u64.pow(num_digits / 2);
        upper_half == lower_half
    }
}

/// Part Two - Invalid if some digit sequence repeats at least twice (e.g. `123123123`: `123`x3).
///
/// Algorithm: Start from left-most digit, gradually increase the length of the string to search.
///
/// Example: `123123`:
///
/// ```txt
/// Search '1' from '123123123'
///   Search '1' in '2', match = false
/// Search '12' from '123123123'
///   Skip search '12' in '3123123' since length of remaining substr not divisible by 2
/// Search '123' from '123123123'
///   Search '123' in '123', match = true
///   Search '123' in '123', match = true
/// ```
fn is_invalid_part_two(number: u64) -> bool {
    if number / 10 == 0 {
        return false; // Single digit always valid
    }

    let number_str = number.to_string();
    let num_digits = number_str.len();

    (1..=num_digits.div_ceil(2)).any(|pattern_len| {
        let rest_len = num_digits - pattern_len;
        if !rest_len.is_multiple_of(pattern_len) {
            return false;
        }

        let pattern = &number_str[..pattern_len];
        (0..rest_len / pattern_len).all(|round| {
            let start_index = pattern_len + pattern_len * round;
            let sub_str = &number_str[start_index..start_index + pattern_len];
            pattern == sub_str
        })
    })
}

fn find_invalid_ids(range: RangeInclusive<u64>, part: Part) -> Vec<u64> {
    let is_invalid = match part {
        Part::One => is_invalid_part_one,
        Part::Two => is_invalid_part_two,
    };
    range.filter(|&number| is_invalid(number)).collect()
}

/// Day 2: Gift Shop
///
/// - Part One: ID is "invalid" if some digit sequence repeats twice (e.g. `6464` - `64`x2).
/// - Part Two: ID is "invalid" if some digit sequence repeats at least twice (e.g. `123123123` = `123`x3)
fn solve_day02(ranges: &[RangeInclusive<u64>], part: Part) -> u64 {
    ranges.iter().fold(0, |sum, range| {
        sum + find_invalid_ids(range.clone(), part).iter().sum::<u64>()
    })
}

fn main() -> Result<()> {
    let input = fs::read_to_string("puzzle_inputs/day02.txt")?;
    let input = input.trim();
    let ranges = parse_input(input);

    let part_1_solution = solve_day02(&ranges, Part::One);
    let part_2_solution = solve_day02(&ranges, Part::Two);
    println!("Part 1 Solution: {part_1_solution}");
    println!("Part 2 Solution: {part_2_solution}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_input() {
        let input = "10327-17387,9696863768-9697013088,1-10000";
        assert_eq!(
            parse_input(input),
            vec![10327..=17387, 9696863768..=9697013088, 1..=10000]
        )
    }

    #[test]
    fn test_is_invalid_part_one() {
        assert!(is_invalid_part_one(11));
        assert!(is_invalid_part_one(22));
        assert!(is_invalid_part_one(1010));
        assert!(is_invalid_part_one(1188511885));

        assert!(!is_invalid_part_one(1));
        assert!(!is_invalid_part_one(9));
        assert!(!is_invalid_part_one(10));
        assert!(!is_invalid_part_one(8998));
        assert!(!is_invalid_part_one(16789524));
        assert!(!is_invalid_part_one(2222222225));
    }

    #[test]
    fn test_is_invalid_part_two() {
        assert!(is_invalid_part_two(11));
        assert!(is_invalid_part_two(555));
        assert!(is_invalid_part_two(123123));
        assert!(is_invalid_part_two(121212121212));
        assert!(is_invalid_part_two(479502479502));
        assert!(is_invalid_part_two(935935935935));

        assert!(!is_invalid_part_two(1));
        assert!(!is_invalid_part_two(10));
        assert!(!is_invalid_part_two(1001));
        assert!(!is_invalid_part_two(1212121214));
        assert!(!is_invalid_part_two(123123132123));
        assert!(!is_invalid_part_two(12341234123));
    }

    #[test]
    fn test_find_invalid_ids_part_one() {
        // Puzzle example
        assert_eq!(find_invalid_ids(11..=22, Part::One), vec![11, 22]);
        assert_eq!(find_invalid_ids(95..=115, Part::One), vec![99]);
        assert_eq!(find_invalid_ids(998..=1012, Part::One), vec![1010]);
        assert_eq!(
            find_invalid_ids(1188511880..=1188511890, Part::One),
            vec![1188511885]
        );
        assert_eq!(find_invalid_ids(222220..=222224, Part::One), vec![222222]);
        assert_eq!(find_invalid_ids(1698522..=1698528, Part::One), vec![]);
        assert_eq!(find_invalid_ids(446443..=446449, Part::One), vec![446446]);
        assert_eq!(
            find_invalid_ids(38593856..=38593862, Part::One),
            vec![38593859]
        );
        assert_eq!(find_invalid_ids(565653..=565659, Part::One), vec![]);
        assert_eq!(find_invalid_ids(824824821..=824824827, Part::One), vec![]);
        assert_eq!(find_invalid_ids(2121212118..=2121212124, Part::One), vec![]);
    }

    #[test]
    fn test_find_invalid_ids_part_two() {
        // Puzzle example
        assert_eq!(find_invalid_ids(11..=22, Part::Two), vec![11, 22]);
        assert_eq!(find_invalid_ids(95..=115, Part::Two), vec![99, 111]);
        assert_eq!(find_invalid_ids(998..=1012, Part::Two), vec![999, 1010]);
        assert_eq!(
            find_invalid_ids(1188511880..=1188511890, Part::Two),
            vec![1188511885]
        );
        assert_eq!(find_invalid_ids(222220..=222224, Part::Two), vec![222222]);
        assert_eq!(find_invalid_ids(1698522..=1698528, Part::Two), vec![]);
        assert_eq!(find_invalid_ids(446443..=446449, Part::Two), vec![446446]);
        assert_eq!(
            find_invalid_ids(38593856..=38593862, Part::Two),
            vec![38593859]
        );
        assert_eq!(find_invalid_ids(565653..=565659, Part::Two), vec![565656]);
        assert_eq!(
            find_invalid_ids(824824821..=824824827, Part::Two),
            vec![824824824]
        );
        assert_eq!(
            find_invalid_ids(2121212118..=2121212124, Part::Two),
            vec![2121212121]
        );
    }

    #[test]
    fn test_solve_day02() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,\
        446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

        let ranges = parse_input(input);
        assert_eq!(solve_day02(&ranges, Part::One), 1227775554);
        assert_eq!(solve_day02(&ranges, Part::Two), 4174379265);
    }
}
