use std::fs;

use advent_of_code_2025::Part;
use anyhow::Result;

#[inline]
fn get_digit(row: &str, index: usize) -> &str {
    &row[index..index + 1]
}

/// Given a `row` of string consisting numbers of 1~9, select `target_num` of digits from left to
/// right (no need to be consecutive) so that it forms the largest number without re-arranging the numbers.
///
/// # Example
///
/// ```
/// assert_eq!(largest_joltage("12321", 2), 32);
/// assert_eq!(largest_joltage("125241", 2), 54);
/// assert_eq!(largest_joltage("123251", 3), 351);
/// ```
fn largest_joltage(row: &str, target_num: usize) -> u64 {
    let row_len = row.len();
    let mut digit_indexes: Vec<usize> = (0..target_num).collect();
    let mut pointer = 1;

    'pointer_loop: while pointer < row_len {
        let digits: Vec<&str> = digit_indexes
            .iter()
            .map(|&index| get_digit(row, index))
            .collect();
        let pointer_digit = get_digit(row, pointer);

        for (digit_index, digit) in digits.iter().enumerate() {
            // If pointer digit > any currently selected digit, move the rest of the selected digits
            // starting from the right of the pointer
            // e.g.   v  pointer             v pointer              v pointer
            //      3 4 5 8 1      -->   3 4 5 8 1      -->   3 4 5 8 1
            //      ^ ^ ^ selected         ^ ^ ^ selected         ^ ^ ^ selected
            if pointer > digit_indexes[digit_index]
                && pointer_digit > digit
                // can select remaining digits without overflow
                && pointer + (target_num - digit_index) - 1 < row_len
            {
                (digit_index..target_num).for_each(|i| {
                    digit_indexes[i] = pointer + i - digit_index;
                });
                pointer = digit_indexes[digit_index] + 1;
                continue 'pointer_loop;
            }
        }
        pointer += 1;
    }

    let result: String = digit_indexes
        .iter()
        .map(|&index| get_digit(row, index))
        .collect::<Vec<&str>>()
        .join("");

    result
        .parse()
        .unwrap_or_else(|_| panic!("failed to convert {result} to number"))
}

/// Day 3: Lobby
///
/// - Part One: Picks 2 numbers from list of numbers.
/// - Part Two: Picks 12 numbers from list of numbers.
fn solve_day03(input: &str, part: Part) -> u64 {
    let rows: Vec<&str> = input.lines().collect();
    let target_num = match part {
        Part::One => 2,
        Part::Two => 12,
    };
    rows.iter()
        .map(|row| largest_joltage(row, target_num))
        .sum()
}

fn main() -> Result<()> {
    let input = fs::read_to_string("puzzle_inputs/day03.txt")?;
    let input = input.trim();

    let part_1_solution = solve_day03(input, Part::One);
    let part_2_solution = solve_day03(input, Part::Two);
    println!("Part 1 Solution: {part_1_solution}");
    println!("Part 2 Solution: {part_2_solution}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_largest_joltage() {
        // Part One (2 batteries)
        assert_eq!(largest_joltage("11", 2), 11);
        assert_eq!(largest_joltage("12345", 2), 45);
        assert_eq!(largest_joltage("123454321", 2), 54);
        assert_eq!(largest_joltage("12345439", 2), 59);
        assert_eq!(largest_joltage("123454391", 2), 91);
        assert_eq!(largest_joltage("111119111111", 2), 91);
        assert_eq!(largest_joltage("111119111112", 2), 92);

        // >2 batteries
        assert_eq!(largest_joltage("1234", 3), 234);
        assert_eq!(largest_joltage("3645", 3), 645);
        assert_eq!(largest_joltage("6138125", 3), 825);
        assert_eq!(largest_joltage("7465255975185", 4), 9785);

        // Puzzle examples - Part One
        assert_eq!(largest_joltage("987654321111111", 2), 98);
        assert_eq!(largest_joltage("811111111111119", 2), 89);
        assert_eq!(largest_joltage("234234234234278", 2), 78);
        assert_eq!(largest_joltage("818181911112111", 2), 92);

        // Puzzle examples - Part Two
        assert_eq!(largest_joltage("987654321111111", 12), 987654321111);
        assert_eq!(largest_joltage("811111111111119", 12), 811111111119);
        assert_eq!(largest_joltage("234234234234278", 12), 434234234278);
        assert_eq!(largest_joltage("818181911112111", 12), 888911112111);
    }

    #[test]
    fn test_solve_day03() {
        let input = r"987654321111111
811111111111119
234234234234278
818181911112111";

        assert_eq!(solve_day03(input, Part::One), 357);
        assert_eq!(solve_day03(input, Part::Two), 3121910778619);
    }
}
