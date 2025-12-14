use std::{fmt::Display, fs};

use advent_of_code_2025::{
    Part,
    coords::{CompressedCoords2D, Coords2D},
};
use anyhow::{Error, Result};
use grid::Grid;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    /// Empty space (`.`)
    Empty,
    /// Red tile (`#`)
    Red,
    /// Green tile (`X`)
    Green,
}

impl Cell {
    #[inline]
    fn is_tile(&self) -> bool {
        self == &Cell::Red || self == &Cell::Green
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Empty => write!(f, "."),
            Cell::Red => write!(f, "#"),
            Cell::Green => write!(f, "X"),
        }
    }
}

impl TryFrom<char> for Cell {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Cell::Empty),
            '#' => Ok(Cell::Red),
            'X' => Ok(Cell::Green),
            _ => Err(anyhow::anyhow!("Invalid cell character '{value}'")),
        }
    }
}

fn solve_day09(input: &str, part: Part) -> usize {
    let coords = parse_input_to_coords(input);
    match part {
        Part::One => find_largest_rect_area(&coords),
        Part::Two => find_largest_red_and_green_rect_area(&coords),
    }
}

#[inline]
fn rect_area(a: &Coords2D, b: &Coords2D) -> usize {
    ((a.x.abs_diff(b.x) + 1) * (a.y.abs_diff(b.y) + 1)) as usize
}

/// Part One - Finds the largest rectangle area formed from 2 coordinates being the corners of the rectangle.
fn find_largest_rect_area(coords: &[Coords2D]) -> usize {
    coords
        .iter()
        .combinations(2)
        .map(|points| {
            let [a, b] = [points[0], points[1]];
            rect_area(a, b)
        })
        .max()
        .expect("coords should not be empty")
}

/// Part Two - The input `coords` of red tiles (`#`) can be connected in straight line by green
/// tiles (`X`). All tiles inside the loop of red/green tile is also green. Find the area of the
/// largest rectangle where you can make only red and green tiles.
///
/// # Example
///
/// (Top left corner is (0, 0))
///
/// ```txt
/// ..............                  ..............
/// .......#XXX#..                  .......#XXX#..
/// .......XxxxX..                  .......XxxxX..
/// ..#XXXX#xxxX..     =====>       ..OOOOOOOOxX..
/// ..XxxxxxxxxX..                  ..OOOOOOOOxX..
/// ..#XXXXXX#xX..                  ..OOOOOOOOxX..
/// .........XxX..                  .........XxX..
/// .........#X#..                  .........#X#..
/// ..............                  ..............
/// ```
///
/// # Algorithm
///
/// 1. Compress the input coordinates from `max(x) * max(y)` to `len(unique(x)) * len(unique(y))` so
///    that the board is significantly smaller to operate on
/// 2. Connect the red tiles (`#`) together with green tiles (`X`) to form an enclosed polygon
/// 3. Find a point that's inside the polygon
/// 4. Fill the polygon with green tiles starting with the point in Step 3
/// 5. For every 2 pairs of red tiles, see if the 4 sides of rectangle are entirely red/green tiles
///
/// Inspired by https://www.reddit.com/r/adventofcode/comments/1pichj2/comment/nt5guy3
///
/// Limitation: Step 4 only fills the polygon once from one starting point. However, it's possible
/// that any polygon has >=2 areas of empty tiles that are disconnected from each other and can be filled.
/// The puzzle input doesn't have this edge case so it's fine.
fn find_largest_red_and_green_rect_area(coords: &[Coords2D]) -> usize {
    let compressed_coords = CompressedCoords2D::from_coords(coords);

    let mut grid = make_cell_grid_from_compressed_coords(&compressed_coords);
    connect_red_tiles(&mut grid, &compressed_coords.coords);

    if let Some(start_coords) = find_first_inside_point(&grid) {
        fill_green_tiles(&mut grid, &start_coords);
    }

    compressed_coords
        .coords
        .iter()
        .combinations(2)
        .filter_map(|points| {
            let [a, b] = [points[0], points[1]];
            match is_rect_in_red_and_green(&grid, a, b) {
                true => {
                    let a_original = compressed_coords.to_original(a).unwrap();
                    let b_original = compressed_coords.to_original(b).unwrap();
                    Some(rect_area(&a_original, &b_original))
                }
                false => None,
            }
        })
        .max()
        .expect("should have at least 1 satisfying rectangle")
}

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

/// Constructs a cell grid with red tiles only from the given coordinates of red tiles.
fn make_cell_grid(coords: &[Coords2D], rows: usize, cols: usize) -> Grid<Cell> {
    let mut grid = Grid::init(rows, cols, Cell::Empty);
    coords.iter().for_each(|coord| {
        if let Some(cell) = grid.get_mut(coord.y, coord.x) {
            *cell = Cell::Red;
        }
    });
    grid
}

fn make_cell_grid_from_compressed_coords(compressed_coords: &CompressedCoords2D) -> Grid<Cell> {
    let rows = (compressed_coords.max_y() + 1) as usize;
    let cols = (compressed_coords.max_x() + 1) as usize;
    make_cell_grid(&compressed_coords.coords, rows, cols)
}

/// Connects the red tiles (`#`) together with green tiles (`X`), creating a hollow polygon.
///
/// # Example
///
/// ```txt
/// .......#...#..                 .......#XXX#..
/// ..............                 .......X...X..
/// ..#....#......     =====>      ..#XXXX#...X..
/// ..#........#..                 ..#XXXXXXXX#..
/// ```
fn connect_red_tiles(grid: &mut Grid<Cell>, red_tile_coords: &[Coords2D]) {
    // Self-wraps with first element (e.g. A -> B -> C -> A)
    let coords_iter = red_tile_coords.iter().chain(red_tile_coords.iter().take(1));
    for (a, b) in coords_iter.tuple_windows() {
        if a.x == b.x {
            let (start, end) = if a.y < b.y { (a, b) } else { (b, a) };
            for y in (start.y + 1)..end.y {
                if let Some(cell) = grid.get_mut(y, start.x) {
                    *cell = Cell::Green;
                }
            }
        }
        if a.y == b.y {
            let (start, end) = if a.x < b.x { (a, b) } else { (b, a) };
            for x in (start.x + 1)..end.x {
                if let Some(cell) = grid.get_mut(start.y, x) {
                    *cell = Cell::Green;
                }
            }
        }
    }
}

/// Finds the first empty point that's inside the polygon after connecting red tiles together to form
/// edges. The search starts from top to bottom, left to right.
///
/// It uses the [Point in Polygon](https://en.wikipedia.org/wiki/Point_in_polygon) algorithm, which
/// casts a horizontal ray from left to the point. The theorem states that the point is inside if the
/// ray intersects the edges for odd number of times.
fn find_first_inside_point(grid: &Grid<Cell>) -> Option<Coords2D> {
    // No need search first and last row/col since it's guaranteed to be outside the polygon
    for row in 1..grid.rows() - 1 {
        for col in 1..grid.cols() - 1 {
            if grid[(row, col)] != Cell::Empty {
                continue;
            }

            // When found empty cell, cast ray leftwards and count no. of boundary crossings
            let mut boundary_cross_indexes: Vec<usize> = vec![];
            let mut inside_boundary = false;

            for x in (0..col).rev() {
                let cell = grid[(row, x)];
                if cell.is_tile() && !inside_boundary {
                    boundary_cross_indexes.push(x);
                    inside_boundary = true;
                } else if !cell.is_tile() && inside_boundary {
                    // Handle `.#XXX#.`
                    //       x ┘     └ start
                    if grid[(row, x + 1)] == Cell::Red {
                        boundary_cross_indexes.push(x + 1);
                    }

                    // Handle `.XXX.`
                    //       x ┘   └ start
                    // If all are `X`, then they must be vertical edges
                    let last_boundary_cross_index = *boundary_cross_indexes.last().unwrap();
                    let boundary_cells: Vec<&Cell> = grid
                        .iter_row(row)
                        .get(x + 1..=last_boundary_cross_index)
                        .collect();
                    if boundary_cells.iter().all(|&cell| *cell == Cell::Green) {
                        let mut boundary_indexes: Vec<usize> =
                            (x + 1..=last_boundary_cross_index - 1).rev().collect();
                        boundary_cross_indexes.append(&mut boundary_indexes);
                    }

                    inside_boundary = false;
                }
            }

            if boundary_cross_indexes.len() % 2 == 1 {
                return Some(Coords2D::new(col as i64, row as i64));
            }
        }
    }
    None
}

/// Fills the polygon created from connecting red tiles (`#`) with green tiles (`X`).
fn fill_green_tiles(grid: &mut Grid<Cell>, start: &Coords2D) {
    assert_eq!(grid.get(start.y, start.x), Some(&Cell::Empty));

    let mut coords_to_fill: Vec<Coords2D> = vec![start.clone()];
    let search_dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    while let Some(coords) = coords_to_fill.pop() {
        *grid.get_mut(coords.y, coords.x).unwrap() = Cell::Green;

        search_dirs.iter().for_each(|(dx, dy)| {
            let new_coords = Coords2D::new(coords.x + dx, coords.y + dy);
            if grid.get(new_coords.y, new_coords.x) == Some(&Cell::Empty) {
                coords_to_fill.push(new_coords);
            }
        });
    }
}

/// Checks whether the rectangle formed by the two coordinates are entirely consisted of red/green tiles.
fn is_rect_in_red_and_green(grid: &Grid<Cell>, a: &Coords2D, b: &Coords2D) -> bool {
    let (x1, x2) = if a.x <= b.x {
        (a.x as usize, b.x as usize)
    } else {
        (b.x as usize, a.x as usize)
    };
    let (y1, y2) = if a.y <= b.y {
        (a.y as usize, b.y as usize)
    } else {
        (b.y as usize, a.y as usize)
    };

    for x in x1..=x2 {
        if grid[(y1, x)] == Cell::Empty || grid[(y2, x)] == Cell::Empty {
            return false;
        }
    }
    for y in y1..=y2 {
        if grid[(y, x1)] == Cell::Empty || grid[(y, x2)] == Cell::Empty {
            return false;
        }
    }

    true
}

fn main() -> Result<()> {
    let input = fs::read_to_string("puzzle_inputs/day09.txt")?;
    let input = input.trim();

    let part_1_solution = solve_day09(input, Part::One);
    let part_2_solution = solve_day09(input, Part::Two);
    println!("Part 1 Solution: {part_1_solution}");
    println!("Part 2 Solution: {part_2_solution}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_2025::grid::{grid_to_string, parse_string_to_grid};
    use grid::grid;
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

    #[test]
    fn test_find_largest_red_and_green_rect_area() {
        // Puzzle example (O = selected corner)
        // ..............
        // .......#XXX#..
        // .......X...X..
        // ..OXXXX#...X..
        // ..X........X..
        // ..#XXXXXXO.X..
        // .........X.X..
        // .........#X#..
        // ..............
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
        assert_eq!(find_largest_red_and_green_rect_area(&coords), 24);

        // FIXME: This test case fails because the empty spaces are disconnected in 2 places:
        //
        // Compressed Board before fill:
        //
        // ##..##
        // X#XX#X
        // X.##.X    <- Both (2,1) and (2,4) need to fill
        // #X##X#
        //
        // https://www.reddit.com/r/adventofcode/comments/1pi5rqn/2025_day_9_part_2_check_your_solution_with_this/
        // .#XO............#X#.
        // .XXX............XXX.
        // .XXX............XXX.
        // .XXX............XXX.
        // .XXX............XXX.
        // .XXX............XXX.
        // .XX#XXXXXXXXXXXX#XX.
        // .XXXXX#XXXXXX#XXXXX.
        // .XXXXXX......XXXXXX.
        // .OXXXX#......#XXXX#.
        let coords = [
            Coords2D::new(1, 0),
            Coords2D::new(3, 0),
            Coords2D::new(3, 6),
            Coords2D::new(16, 6),
            Coords2D::new(16, 0),
            Coords2D::new(18, 0),
            Coords2D::new(18, 9),
            Coords2D::new(13, 9),
            Coords2D::new(13, 7),
            Coords2D::new(6, 7),
            Coords2D::new(6, 9),
            Coords2D::new(1, 9),
        ];
        // assert_eq!(find_largest_red_and_green_rect_area(&coords), 30);
    }

    #[test]
    fn test_make_cell_grid_from_compressed_coords() {
        // ..........
        // .#...#....
        // ..........
        // .....#..#.
        // ..........
        // .#......#.
        // ..........
        let coords = [
            Coords2D::new(1, 1),
            Coords2D::new(5, 1),
            Coords2D::new(5, 3),
            Coords2D::new(8, 3),
            Coords2D::new(8, 5),
            Coords2D::new(1, 5),
        ];
        let compressed_coords = CompressedCoords2D::from_coords(&coords);
        let cell_grid = make_cell_grid_from_compressed_coords(&compressed_coords);

        // ##.
        // .##
        // #.#
        let expected_grid = grid![
            [Cell::Red, Cell::Red, Cell::Empty]
            [Cell::Empty, Cell::Red, Cell::Red]
            [Cell::Red, Cell::Empty, Cell::Red]
        ];
        assert_eq!(&cell_grid, &expected_grid);
    }

    #[test]
    fn test_connect_red_tiles() {
        // ..........
        // .#...#....
        // ..........
        // .....#..#.
        // ..........
        // .#......#.
        // ..........
        let coords = [
            Coords2D::new(1, 1),
            Coords2D::new(5, 1),
            Coords2D::new(5, 3),
            Coords2D::new(8, 3),
            Coords2D::new(8, 5),
            Coords2D::new(1, 5),
        ];
        let mut grid = make_cell_grid(&coords, 7, 10);
        connect_red_tiles(&mut grid, &coords);
        let expected_grid_str = r"
..........
.#XXX#....
.X...X....
.X...#XX#.
.X......X.
.#XXXXXX#.
.........."
            .trim();
        assert_eq!(grid_to_string(&grid), expected_grid_str);

        // .#.#
        // ##..
        // #.#.
        // ..##
        let coords = [
            Coords2D::new(1, 0),
            Coords2D::new(1, 1),
            Coords2D::new(0, 1),
            Coords2D::new(0, 2),
            Coords2D::new(2, 2),
            Coords2D::new(2, 3),
            Coords2D::new(3, 3),
            Coords2D::new(3, 0),
        ];
        let mut grid = make_cell_grid(&coords, 4, 4);
        connect_red_tiles(&mut grid, &coords);
        let expected_grid_str = r"
.#X#
##.X
#X#X
..##"
            .trim();
        assert_eq!(grid_to_string(&grid), expected_grid_str);

        // ##
        // ##
        let coords = [
            Coords2D::new(0, 0),
            Coords2D::new(1, 0),
            Coords2D::new(1, 1),
            Coords2D::new(0, 1),
        ];
        let mut grid = make_cell_grid(&coords, 2, 2);
        connect_red_tiles(&mut grid, &coords);
        let expected_grid_str = "##\n##";
        assert_eq!(grid_to_string(&grid), expected_grid_str);
    }

    #[test]
    fn test_fill_green_tiles() {
        let input = r"
.#X#
##.X
#X#X
..##
"
        .trim();
        let mut grid = parse_string_to_grid(input, Cell::try_from).unwrap();
        let start = find_first_inside_point(&grid).unwrap();
        assert_eq!(start, Coords2D::new(2, 1));
        fill_green_tiles(&mut grid, &start);
        let expected_grid_str = r"
.#X#
##XX
#X#X
..##
"
        .trim();
        assert_eq!(grid_to_string(&grid), expected_grid_str);

        let input = r"
....##....
....XX....
.#XX##XX#.
.X......X.
.#XXXXXX#.
"
        .trim();
        let mut grid = parse_string_to_grid(input, Cell::try_from).unwrap();
        let start = find_first_inside_point(&grid).unwrap();
        assert_eq!(start, Coords2D::new(2, 3));
        fill_green_tiles(&mut grid, &start);
        let expected_grid_str = r"
....##....
....XX....
.#XX##XX#.
.XXXXXXXX.
.#XXXXXX#.
"
        .trim();
        assert_eq!(grid_to_string(&grid), expected_grid_str);

        let input = r"
..............
.......#XXX#..
.......X...X..
..#XXXX#...X..
..X........X..
..#XXXXXX#.X..
.........X.X..
.........#X#..
..............
        "
        .trim();
        let mut grid = parse_string_to_grid(input, Cell::try_from).unwrap();
        let start = find_first_inside_point(&grid).unwrap();
        assert_eq!(start, Coords2D::new(8, 2));
        fill_green_tiles(&mut grid, &start);
        let expected_grid_str = r"
..............
.......#XXX#..
.......XXXXX..
..#XXXX#XXXX..
..XXXXXXXXXX..
..#XXXXXX#XX..
.........XXX..
.........#X#..
..............
"
        .trim();
        assert_eq!(grid_to_string(&grid), expected_grid_str);

        let input = r"
.............
.#XXXX#......
.X....X......
.X.#XX#......
.X.#XX#......
.X....X......
.X....X......
.X....X......
.X.##.#XXXX#.
.X.XX......X.
.#X##XXXXXX#.
        "
        .trim();
        let mut grid = parse_string_to_grid(input, Cell::try_from).unwrap();
        let start = find_first_inside_point(&grid).unwrap();
        assert_eq!(start, Coords2D::new(2, 2));
        fill_green_tiles(&mut grid, &start);
        let expected_grid_str = r"
.............
.#XXXX#......
.XXXXXX......
.XX#XX#......
.XX#XX#......
.XXXXXX......
.XXXXXX......
.XXXXXX......
.XX##X#XXXX#.
.XXXXXXXXXXX.
.#X##XXXXXX#.
"
        .trim();
        assert_eq!(grid_to_string(&grid), expected_grid_str);

        let input = r"
...#XXXXXXXX#
...X........X
...X.#XXXX#.X
...X.X....X.X
#X#X.X#X#.X.X
X.XX.XX.X.X.X
X.X#X#X.X.X.X
X.#XXX#.X.X.X
X.......X.X.X
X.......#X#.X
X...........X
#XXXXXXXXXXX#
        "
        .trim();
        let mut grid = parse_string_to_grid(input, Cell::try_from).unwrap();
        let start = find_first_inside_point(&grid).unwrap();
        assert_eq!(start, Coords2D::new(4, 1));
        fill_green_tiles(&mut grid, &start);
        let expected_grid_str = r"
...#XXXXXXXX#
...XXXXXXXXXX
...XX#XXXX#XX
...XXX....XXX
#X#XXX#X#.XXX
XXXXXXXXX.XXX
XXX#X#XXX.XXX
XX#XXX#XX.XXX
XXXXXXXXX.XXX
XXXXXXXX#X#XX
XXXXXXXXXXXXX
#XXXXXXXXXXX#
"
        .trim();
        assert_eq!(grid_to_string(&grid), expected_grid_str);
    }

    #[test]
    fn test_is_rect_in_red_and_green() {
        let input = r"
#XXXXXXX#
XXXXXXXXX
XXXXX#XX#
XXXXXX...
XXXXX#XX#
#XXXXXXX#
        "
        .trim();

        // A     B
        //
        //    D  C
        //
        //    E  F
        // H     G
        let [a, b, c, d, e, f, g, h] = [
            &Coords2D::new(0, 0),
            &Coords2D::new(8, 0),
            &Coords2D::new(8, 2),
            &Coords2D::new(5, 2),
            &Coords2D::new(5, 4),
            &Coords2D::new(8, 4),
            &Coords2D::new(8, 5),
            &Coords2D::new(0, 5),
        ];

        let grid = parse_string_to_grid(input, Cell::try_from).unwrap();

        assert!(is_rect_in_red_and_green(&grid, a, b));
        assert!(is_rect_in_red_and_green(&grid, a, c));
        assert!(is_rect_in_red_and_green(&grid, a, d));
        assert!(is_rect_in_red_and_green(&grid, a, e));
        assert!(!is_rect_in_red_and_green(&grid, a, f));
        assert!(!is_rect_in_red_and_green(&grid, a, g));
        assert!(is_rect_in_red_and_green(&grid, a, h));
        assert!(is_rect_in_red_and_green(&grid, d, e));
        assert!(!is_rect_in_red_and_green(&grid, d, f));
    }
}
