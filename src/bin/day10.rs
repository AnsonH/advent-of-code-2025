use std::{
    collections::{HashSet, VecDeque},
    fs,
};

use advent_of_code_2025::Part;
use anyhow::Result;
use itertools::Itertools;

fn solve_day10(input: &str, part: Part) -> usize {
    let machines: Vec<Machine> = input.lines().map(Machine::from_input).collect();
    match part {
        Part::One => machines.iter().map(min_presses_to_target_state).sum(),
        Part::Two => todo!(),
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Machine {
    bulb_count: usize,
    /// Target states of each bulb represented in binary. Leftmost bulb is least significant bit.
    ///
    /// e.g. `[##..]` = `0011` in binary (NOT `1100`!), or `6` in decimal
    target_state: u16,
    /// Each button is represented in binary. Pressing the button toggles the bulbs described by
    /// its positions.
    ///
    /// e.g. `(0, 2)` = `0101` in binary, or `9` in decimal
    buttons: Vec<u16>,
    joltages: Vec<u16>,
}

impl Machine {
    #[must_use]
    fn new(bulb_count: usize, target_state: u16, buttons: Vec<u16>, joltages: Vec<u16>) -> Self {
        Self {
            bulb_count,
            target_state,
            buttons,
            joltages,
        }
    }

    /// Parses a single line of input (e.g. `[.##.] (1) (2) (0,3) {3,5,4,7}`)
    fn from_input(input: &str) -> Self {
        let segments: Vec<&str> = input.split_ascii_whitespace().collect();

        let target_state_str = &segments[0][1..segments[0].len() - 1]
            .chars()
            .rev()
            .map(|c| if c == '#' { '1' } else { '0' })
            .join("");
        let bulb_count = target_state_str.len();
        let target_state = u16::from_str_radix(target_state_str, 2).unwrap();

        let button_strings = segments.iter().get(1..segments.len() - 1);
        let buttons: Vec<u16> = button_strings
            .map(|input_str| {
                input_str[1..input_str.len() - 1]
                    .split(",")
                    .fold(0, |acc, pos| acc | (1 << pos.parse::<u16>().unwrap()))
            })
            .collect();

        let joltages_str = segments.last().unwrap();
        let joltages: Vec<u16> = joltages_str[1..joltages_str.len() - 1]
            .split(",")
            .map(|s| s.parse().unwrap())
            .collect();

        Self::new(bulb_count, target_state, buttons, joltages)
    }
}

/// Part One - Find minimum number of button presses to reach the machine target state.
///
/// It performs [Breadth First Search](https://en.wikipedia.org/wiki/Breadth-first_search) on a graph
/// where each node is all possible states of the light bulbs, and each edge is a possible state transition
/// after pressing any button. It starts at state `0` (all bulbs are off).
///
/// Pressing a button is presented with XOR, since a button toggles the bulb.
fn min_presses_to_target_state(machine: &Machine) -> usize {
    let mut explored_states: HashSet<u16> = HashSet::new();
    let mut queue: VecDeque<(usize, u16)> = VecDeque::from([(0, 0)]); // (round_num, state_to_explore)

    while let Some((round_num, state)) = &queue.pop_front() {
        if *state == machine.target_state {
            return *round_num;
        }
        explored_states.insert(*state);
        machine.buttons.iter().for_each(|&button| {
            let next_state = *state ^ button;
            if !explored_states.contains(&next_state) {
                queue.push_back((round_num + 1, next_state));
            }
        });
    }

    panic!(
        "Machine target state [{}] is unreachable",
        debug_machine_state(machine.target_state, machine.bulb_count)
    )
}

/// e.g. `debug_machine_state(6, 4)` = `"##.."` (6 = `0011` binary)
fn debug_machine_state(current_state: u16, bulb_count: usize) -> String {
    // `width$` = named parameter
    // `:05` = left pad a number with `0` till reach total length `5`
    // `:0width$` = left pad a number with `0` till reach total length of `width`
    // `b` = format as binary
    let s = format!("{current_state:0width$b}", width = bulb_count);
    let s: String = s.chars().rev().collect();
    s.replace("0", ".").replace("1", "#").to_string()
}

fn main() -> Result<()> {
    let input = fs::read_to_string("puzzle_inputs/day10.txt")?;
    let input = input.trim();

    let part_1_solution = solve_day10(input, Part::One);
    // let part_2_solution = solve_day10(input, Part::Two);
    println!("Part 1 Solution: {part_1_solution}");
    // println!("Part 2 Solution: {part_2_solution}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_machine_from_input() {
        assert_eq!(
            Machine::from_input(r"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}"),
            Machine::new(4, 6, vec![8, 10, 4, 12, 5, 3], vec![3, 5, 4, 7])
        );
        assert_eq!(
            Machine::from_input(
                r"[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"
            ),
            Machine::new(6, 46, vec![31, 25, 55, 6], vec![10, 11, 11, 5, 10, 5])
        );
    }

    #[test]
    fn test_min_presses_to_target_state() {
        // Puzzle examples
        let machine = Machine::from_input(r"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}");
        assert_eq!(min_presses_to_target_state(&machine), 2); // (0, 1) -> (0, 2)

        let machine =
            Machine::from_input(r"[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}");
        assert_eq!(min_presses_to_target_state(&machine), 3); // last 3 buttons

        let machine = Machine::from_input(
            r"[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
        );
        assert_eq!(min_presses_to_target_state(&machine), 2); // (0,3,4) -> (0,1,2,4,5)
    }

    #[test]
    fn test_debug_machine_state() {
        assert_eq!(debug_machine_state(7, 4), String::from("###."));
        assert_eq!(debug_machine_state(14, 7), String::from(".###..."));
        assert_eq!(debug_machine_state(22, 6), String::from(".##.#."));
    }

    #[test]
    fn test_solve_day10() {
        let input = r"
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
"
        .trim();

        assert_eq!(solve_day10(input, Part::One), 7);
    }
}
