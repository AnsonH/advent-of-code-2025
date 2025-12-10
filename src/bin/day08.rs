use std::collections::{HashMap, HashSet};
use std::fs;

use advent_of_code_2025::{Part, coords::Coords3D, line::Line3D};
use anyhow::Result;
use itertools::{Itertools, iproduct};

fn parse_input_to_coords(input: &str) -> Vec<Coords3D> {
    input
        .lines()
        .map(|line| {
            let values: Vec<i64> = line
                .split(",")
                .map(|num_str| num_str.parse().expect("should be a valid integer"))
                .collect();
            Coords3D::new(values[0], values[1], values[2])
        })
        .collect()
}

/// Builds a hash map of all possible lines that can be formed between 2 [Coords3D] and its respective
/// length. The line is undirected, so `Line3D(A, B) == Line3D(B, A)`, and the hash map has no
/// duplicated lines.
fn build_edge_length_map(coords: &[Coords3D]) -> HashMap<Line3D, f64> {
    let mut line_to_length_map = HashMap::new();
    iproduct!(coords, coords)
        .filter(|(coord_a, coord_b)| coord_a != coord_b)
        .for_each(|(coord_a, coord_b)| {
            // Line3D treats `Line3D(A, B) == Line3D(B, A)`, so there's no duplication of (A, B) and (B, A)
            let line = Line3D(*coord_a, *coord_b);
            line_to_length_map.insert(line.clone(), line.len());
        });
    line_to_length_map
}

/// Connects 2 junction boxes together in the list of circuits.
///
/// Each circuit is a `HashSet<Coords3D>` containing list of junction box coordinates interconnected.
/// Connecting two boxes from different circuits will join the two circuits together. If two boxes
/// are from same circuit, nothing happens.
///
/// # Example
///
/// ```txt
/// [A, B, C]                             [A, B, C, D, E]                             [A, B, C, D, E]
/// [D, E]      --- connect A and D -->   [F]               --- connect A and E --->  [F]
/// [F]             (diff circuits)                             (same circuit)
/// ```
fn connect_junction_box<'a>(
    circuits: &'a mut Vec<HashSet<&Coords3D>>,
    box_a: &'a Coords3D,
    box_b: &'a Coords3D,
) {
    let box_a_circuit_idx = circuits
        .iter()
        .position(|circuit| circuit.contains(box_a))
        .expect("circuits should contain coord_a");
    let box_b_circuit_idx = circuits
        .iter()
        .position(|circuit| circuit.contains(box_b))
        .expect("circuits should contain coord_b");

    if box_a_circuit_idx == box_b_circuit_idx {
        return; // No-op if two boxes already in same circuit
    }

    // Remove the circuit with higher index to avoid shifting
    let (remove_idx, keep_idx) = if box_b_circuit_idx > box_a_circuit_idx {
        (box_b_circuit_idx, box_a_circuit_idx)
    } else {
        (box_a_circuit_idx, box_b_circuit_idx)
    };
    let removed_circuit = circuits.remove(remove_idx);
    circuits[keep_idx].extend(removed_circuit);
}

/// Connects 2 [Coords3D] in ascending order of their distance for `rounds` times. The iteration
/// always ends if the connection causes all junction boxes to form a single circuit.
///
/// # Returns
///
/// A tuple of two items:
/// 1. List of circuits, where each circuit is a set of coordinates forming the circuit.
/// 2. The first line connection which causes all of the junction boxes to form a single circuit.
///    This is also the final line connection.
fn connect_junction_boxes(
    coords: &[Coords3D],
    rounds: usize,
) -> (Vec<HashSet<&Coords3D>>, Option<Line3D>) {
    let edge_length_map = build_edge_length_map(coords);

    let mut circuits: Vec<HashSet<&Coords3D>> =
        coords.iter().map(|coord| HashSet::from([coord])).collect();

    let shortest_edges = edge_length_map
        .iter()
        .sorted_by(|a, b| a.1.partial_cmp(b.1).unwrap()) // sort in ascending line lengths
        .take(rounds);

    let mut final_line: Option<Line3D> = None;

    for (line, _) in shortest_edges {
        connect_junction_box(&mut circuits, &line.0, &line.1);

        if final_line.is_none() && circuits.len() == 1 {
            final_line = Some(line.clone());
            break;
        }
    }

    (circuits, final_line)
}

/// Connects 2 coordinates in ascending order of their distance for `rounds` times, then get the
/// 3 circuits with largest size, and multiply their sizes.
fn solve_day08_part_1(coords: &[Coords3D], rounds: usize) -> usize {
    let (circuits, _) = connect_junction_boxes(coords, rounds);
    circuits
        .iter()
        .map(|circuit| circuit.len())
        .sorted()
        .rev()
        .take(3)
        .product()
}

fn solve_day08_part_2(coords: &[Coords3D]) -> usize {
    let (_, final_line) = connect_junction_boxes(coords, usize::MAX);
    let final_line = final_line.expect("final line connection should be present");
    (final_line.0.x * final_line.1.x) as usize
}

fn solve_day08(input: &str, part: Part) -> usize {
    let coords = parse_input_to_coords(input);
    match part {
        Part::One => solve_day08_part_1(&coords, 1000),
        Part::Two => solve_day08_part_2(&coords),
    }
}

fn main() -> Result<()> {
    let input = fs::read_to_string("puzzle_inputs/day08.txt")?;

    let part_1_solution = solve_day08(&input, Part::One);
    let part_2_solution = solve_day08(&input, Part::Two);
    println!("Part 1 Solution: {part_1_solution}");
    println!("Part 2 Solution: {part_2_solution}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_input_to_coords() {
        let input = "162,817,812\n57,618,57";
        assert_eq!(
            parse_input_to_coords(input),
            vec![Coords3D::new(162, 817, 812), Coords3D::new(57, 618, 57)]
        )
    }

    #[test]
    fn test_build_edge_length_map() {
        let coord_a = Coords3D::new(2, 2, 0);
        let coord_b = Coords3D::new(2, 3, 0);
        let coord_c = Coords3D::new(4, 2, 0);
        let coords = vec![coord_a, coord_b, coord_c];

        let map = build_edge_length_map(&coords);

        assert_eq!(map.len(), 3);
        assert_eq!(map.get(&Line3D(coord_a, coord_b)), Some(&1.0));
        assert_eq!(map.get(&Line3D(coord_a, coord_c)), Some(&2.0));
        assert_eq!(map.get(&Line3D(coord_b, coord_c)), Some(&5_f64.sqrt()));

        assert!(!map.contains_key(&Line3D(coord_a, coord_a)));
    }

    #[test]
    fn test_connect_junction_box() {
        let [a, b, c, d, e, f] = [
            &Coords3D::new(2, 2, 0),
            &Coords3D::new(4, 2, 0),
            &Coords3D::new(2, 5, 0),
            &Coords3D::new(6, 6, 0),
            &Coords3D::new(9, 5, 0),
            &Coords3D::new(10, 0, 0),
        ];

        let mut circuits = vec![[a].into(), [b].into(), [c].into()];
        connect_junction_box(&mut circuits, a, b);
        assert_eq!(&circuits, &vec![[a, b].into(), [c].into()]);

        let mut circuits = vec![[a].into(), [b].into(), [c].into()];
        connect_junction_box(&mut circuits, c, a);
        assert_eq!(&circuits, &vec![[a, c].into(), [b].into()]);

        let mut circuits = vec![[a, b, c].into(), [d, e].into(), [f].into()];
        connect_junction_box(&mut circuits, d, a);
        assert_eq!(&circuits, &vec![[a, b, c, d, e].into(), [f].into()]);

        let mut circuits = vec![[a, b].into(), [c, d].into(), [e, f].into()];
        connect_junction_box(&mut circuits, d, e);
        assert_eq!(&circuits, &vec![[a, b].into(), [c, d, e, f].into()]);

        let mut circuits = vec![[a, b, c].into(), [d, e].into(), [f].into()];
        connect_junction_box(&mut circuits, a, b);
        assert_eq!(
            &circuits,
            &vec![[a, b, c].into(), [d, e].into(), [f].into()]
        );
    }

    #[test]
    fn test_connect_junction_boxes() {
        let coords = [
            Coords3D::new(2, 2, 0),
            Coords3D::new(4, 2, 0),
            Coords3D::new(2, 5, 0),
            Coords3D::new(6, 6, 0),
            Coords3D::new(9, 5, 0),
            Coords3D::new(10, 0, 0),
        ];
        let [a, b, c, d, e, f] = coords;

        // AB (len=2)
        let circuits = connect_junction_boxes(&coords, 1);
        assert_eq!(
            circuits.0,
            vec![
                [&a, &b].into(),
                [&c].into(),
                [&d].into(),
                [&e].into(),
                [&f].into()
            ]
        );
        assert!(circuits.1.is_none());

        // AB -> AC (len=3)
        let circuits = connect_junction_boxes(&coords, 2);
        assert_eq!(
            circuits.0,
            vec![[&a, &b, &c].into(), [&d].into(), [&e].into(), [&f].into()]
        );
        assert!(circuits.1.is_none());

        // AB -> AC -> DE (len=3.16)
        let circuits = connect_junction_boxes(&coords, 3);
        assert_eq!(
            circuits.0,
            vec![[&a, &b, &c].into(), [&d, &e].into(), [&f].into()]
        );
        assert!(circuits.1.is_none());

        // AB -> AC -> DE -> AC (len=3.6)
        let circuits = connect_junction_boxes(&coords, 4);
        assert_eq!(
            circuits.0,
            vec![[&a, &b, &c].into(), [&d, &e].into(), [&f].into()]
        );
        assert!(circuits.1.is_none());

        // AB -> AC -> DE -> AC -> BD (len=4.5)
        let circuits = connect_junction_boxes(&coords, 5);
        assert_eq!(circuits.0, vec![[&a, &b, &c, &d, &e].into(), [&f].into()]);
        assert!(circuits.1.is_none());

        // AB -> AC -> DE -> AC -> BD -> ... -> EF (len=5.09)
        let circuits = connect_junction_boxes(&coords, 1000);
        assert_eq!(circuits.0, vec![[&a, &b, &c, &d, &e, &f].into()]);
        assert_eq!(circuits.1, Some(Line3D(e, f)));
    }

    #[test]
    fn test_solve_day08_part_1() {
        // Puzzle example
        let input = r"
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"
            .trim();
        let coords = parse_input_to_coords(input);
        assert_eq!(solve_day08_part_1(&coords, 10), 5 * 4 * 2);
    }

    #[test]
    fn test_solve_day08_part_2() {
        // Puzzle example
        let input = r"
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"
            .trim();
        let coords = parse_input_to_coords(input);
        assert_eq!(solve_day08_part_2(&coords), 216 * 117);
    }
}
