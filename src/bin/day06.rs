use std::{fs, ops::Range, str::FromStr};

use advent_of_code_2025::Part;
use anyhow::Result;
use grid::Grid;
use itertools::Itertools;
use strum::EnumString;

#[derive(Debug, Clone, Copy, PartialEq, EnumString)]
enum Operator {
    #[strum(serialize = "+")]
    Add,
    #[strum(serialize = "*")]
    Multiply,
}

#[derive(Debug, Clone, PartialEq)]
struct Operation {
    /// Numbers to operate on.
    operands: Vec<u64>,
    /// The numeric operation.
    operator: Operator,
}

impl Operation {
    fn new(operands: Vec<u64>, operator: Operator) -> Self {
        Self { operands, operator }
    }
}

/// Read each column's number vertically to form full equation.
///
/// # Example
///
/// ```txt
/// 123 328
///  45 64  
///   6 98  
/// *   +   
/// ```
///
/// Above becomes `123 * 45 * 6` and `328 + 64 + 98`.
fn parse_input_for_part_1(input: &str) -> Vec<Operation> {
    let words: Vec<&str> = input
        .lines()
        .flat_map(|line| line.split_whitespace().collect::<Vec<&str>>())
        .collect();

    let width = words
        .len()
        .checked_div(input.lines().count())
        .unwrap_or_default();

    let mut grid = Grid::from_vec(words, width);
    grid.rotate_right(); // so that each row is an operation (e.g. ["*", "6", "45", "123"])

    grid.iter_rows()
        .map(|mut row_iter| {
            let operator = Operator::from_str(row_iter.next().expect("row should not be empty"))
                .expect("unrecognized operator");

            let operands: Vec<u64> = row_iter
                .map(|number_str| number_str.parse().expect("expected valid number"))
                .rev() // as `grid.rotate_right()` reversed the order of operands
                .collect();

            Operation::new(operands, operator)
        })
        .collect()
}

/// Start from every vertical column from right to left, and read digits from top to down
///
/// # Example
///
/// ```txt
/// 123 328
///  45 64  
///   6 98  
/// *   +   
/// ```
/// Above becomes `8 + 248 + 369` and `356 * 24 * 1`
fn parse_input_for_part_2(input: &str) -> Vec<Operation> {
    let mut lines_iter = input.lines();

    // Pattern: The operator symbol is always the leftmost position of a "number column", we can use
    // spacing between operators to deduce the index range of each "number column"
    //
    // Example:
    //
    // 123   8
    //  45  76    -->   operators_with_col_range = [(Operator::Multiply, 0..3), (Operator::Add, 4..7)]
    //   6 543
    // *   +
    let operators_line = lines_iter.next_back().expect("operators row is missing");
    let operators_with_col_range: Vec<(Operator, Range<usize>)> = operators_line
        .chars()
        .enumerate()
        .fold(vec![], |mut acc, (idx, ch)| {
            if let Ok(operator) = Operator::from_str(&ch.to_string()) {
                if let Some((old_op, old_range)) = acc.pop() {
                    // The `- 1` in `idx - 1` is to ignore a single whitespace between 2 adjacent number columns
                    acc.push((old_op, old_range.start..idx - 1));
                }
                acc.push((operator, idx..operators_line.len()));
            }
            acc
        });

    let number_lines: Vec<&str> = lines_iter.collect();
    operators_with_col_range
        .iter()
        .map(|(operator, col_range)| {
            let operands: Vec<u64> = col_range
                .clone()
                .rev() // since we read columns right-to-left
                .map(|col_idx| {
                    // Read every column from top to bottom to get each operand
                    number_lines
                        .iter()
                        .map(|line| line.get(col_idx..col_idx + 1).unwrap())
                        .join("")
                        .trim()
                        .parse::<u64>()
                        .expect("expected valid number")
                })
                .collect();
            Operation::new(operands, *operator)
        })
        .rev() // read entire "number columns" right-to-left
        .collect()
}

/// Old Solution - Using 2D grid transformations
#[allow(dead_code)]
#[deprecated]
fn parse_input_for_part_2_alternative(input: &str) -> Vec<Operation> {
    let mut lines_iter = input.lines();

    // Pattern: The operator symbol is always the leftmost position of a "number column", we can use
    // spacing between operators to deduce the index range of each "number column"
    //
    // Example:
    //
    // 123   8
    //  45  76    -->   operators_with_col_range = [(Operator::Multiply, 0..3), (Operator::Add, 4..7)]
    //   6 543
    // *   +
    let operators_line = lines_iter.next_back().expect("operators row is missing");
    let operators_with_col_range: Vec<(Operator, Range<usize>)> = operators_line
        .chars()
        .enumerate()
        .fold(vec![], |mut acc, (idx, ch)| {
            if let Ok(operator) = Operator::from_str(&ch.to_string()) {
                if let Some((old_op, old_range)) = acc.pop() {
                    // The `- 1` in `idx - 1` is to ignore a single whitespace between 2 adjacent number columns
                    acc.push((old_op, old_range.start..idx - 1));
                }
                acc.push((operator, idx..operators_line.len()));
            }
            acc
        });

    // Construct a 2D grid of input numbers, example:
    //
    // 123   8                                     [["123", "  8"]
    //  45  76    -->  input_number_strings_grid =  [" 45", " 76"]
    //   6 543                                      ["  6", "543"]]
    // *   +
    let number_strings: Vec<&str> = lines_iter
        .flat_map(|line| {
            operators_with_col_range
                .iter()
                .map(|(_, col_range)| line.get(col_range.clone()).unwrap())
        })
        .collect();
    let input_number_strings_grid = Grid::from_vec(number_strings, operators_with_col_range.len());

    input_number_strings_grid
        .iter_cols()
        .zip(operators_with_col_range.iter())
        .map(|(number_col, (operator, _))| {
            let col_number_strings: Vec<&str> = number_col.copied().collect(); // e.g. ["123", " 45", "  6"]
            let col_width = col_number_strings
                .first()
                .map_or(0, |col_num| col_num.len());

            // Example:
            //                                                                    [["1", "2", "3"]
            // col_number_strings = ["123", " 45", "  6"]  -->  col_number_grid =  [" ", "4", "5"]
            //                                                                     [" ", " ", "6"]]
            let col_number_grid: Grid<char> = Grid::from_vec(
                col_number_strings
                    .iter()
                    .flat_map(|s| s.chars().collect::<Vec<char>>())
                    .collect(),
                col_width,
            );
            let operands: Vec<u64> = col_number_grid
                .iter_cols()
                .rev() // Read columns right-to-left
                .map(|mut col_chars| {
                    col_chars
                        .join("")
                        .trim()
                        .parse()
                        .expect("expected valid number")
                })
                .collect();

            Operation::new(operands, *operator)
        })
        .rev()
        .collect()
}

fn compute_operation(operation: &Operation) -> u64 {
    let init = match operation.operator {
        Operator::Add => 0,
        Operator::Multiply => 1,
    };
    operation
        .operands
        .iter()
        .fold(init, |acc, operand| match operation.operator {
            Operator::Add => acc + operand,
            Operator::Multiply => acc * operand,
        })
}

fn solve_day06(input: &str, part: Part) -> u64 {
    let parser = match part {
        Part::One => parse_input_for_part_1,
        Part::Two => parse_input_for_part_2,
    };
    let operations = parser(input);
    operations.iter().map(compute_operation).sum()
}

fn main() -> Result<()> {
    // NOTE: Do NOT `trim_end()` because the whitespaces after the last line matters
    let input = fs::read_to_string("puzzle_inputs/day06.txt")?;

    let part_1_solution = solve_day06(&input, Part::One);
    let part_2_solution = solve_day06(&input, Part::Two);
    println!("Part 1 Solution: {part_1_solution}");
    println!("Part 2 Solution: {part_2_solution}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_input_for_part_1() {
        // Puzzle Example
        let input = r"
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   + 
"
        .trim();

        assert_eq!(
            parse_input_for_part_1(input),
            vec![
                Operation::new(vec![123, 45, 6], Operator::Multiply),
                Operation::new(vec![328, 64, 98], Operator::Add),
                Operation::new(vec![51, 387, 215], Operator::Multiply),
                Operation::new(vec![64, 23, 314], Operator::Add),
            ]
        );

        let input = "1\n2\n3\n*";
        assert_eq!(
            parse_input_for_part_1(input),
            vec![Operation::new(vec![1, 2, 3], Operator::Multiply)]
        );

        let input = "";
        assert_eq!(parse_input_for_part_1(input), vec![])
    }

    #[test]
    fn test_parse_input_for_part_2() {
        // Puzzle example
        let input = r"
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "
            .trim_start();
        assert_eq!(
            parse_input_for_part_2(input),
            vec![
                Operation::new(vec![4, 431, 623], Operator::Add),
                Operation::new(vec![175, 581, 32], Operator::Multiply),
                Operation::new(vec![8, 248, 369], Operator::Add),
                Operation::new(vec![356, 24, 1], Operator::Multiply),
            ]
        );

        // Different column shapes
        let input = r"
123456 123456 1           1
   123 123    123       123
     1 1      123456 123456
*      +      *      +     "
            .trim_start();
        assert_eq!(
            parse_input_for_part_2(input),
            vec![
                Operation::new(vec![136, 25, 14, 3, 2, 1], Operator::Add),
                Operation::new(vec![6, 5, 4, 33, 22, 111], Operator::Multiply),
                Operation::new(vec![6, 5, 4, 33, 22, 111], Operator::Add),
                Operation::new(vec![631, 52, 41, 3, 2, 1], Operator::Multiply),
            ]
        );
    }

    #[test]
    fn test_compute_operation() {
        let operation = Operation::new(vec![123, 45, 6], Operator::Multiply);
        assert_eq!(compute_operation(&operation), 123 * 45 * 6);

        let operation = Operation::new(vec![328, 64, 98], Operator::Add);
        assert_eq!(compute_operation(&operation), 328 + 64 + 98);
    }

    #[test]
    fn test_solve_day06() {
        let input = r"
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "
            .trim_start();

        assert_eq!(solve_day06(input, Part::One), 4277556);
        assert_eq!(solve_day06(input, Part::Two), 3263827);
    }
}
