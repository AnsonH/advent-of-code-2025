use bimap::BiMap;
use itertools::Itertools;

use crate::coords::Coords2D;

/// A data structure that compresses 2D coordinates by mapping coordinate values to integers from 0
/// in ascending order.
///
/// This is useful when you have coordinates with large values (e.g., 10000, 50000, 100000)
/// but only care about their relative ordering. The compression maps these to consecutive
/// integers (0, 1, 2), making operations like grid-based algorithms more memory-efficient.
///
/// # Example
///
/// ```
/// # use advent_of_code_2025::coords::{Coords2D, CompressedCoords2D};
/// let coords = vec![
///     Coords2D::new(20000, 30000),
///     Coords2D::new(-15000, 0),
///     Coords2D::new(25000, -15000),
/// ];
///
/// let compressed = CompressedCoords2D::from_coords(&coords);
/// // Original sparse coordinates are now mapped to dense range [0, 1, 2]
/// assert_eq!(compressed.coords[0], Coords2D::new(1, 2));
/// assert_eq!(compressed.coords[1], Coords2D::new(0, 1));
/// assert_eq!(compressed.coords[2], Coords2D::new(2, 0));
/// ```
#[derive(Debug, Clone, Default)]
pub struct CompressedCoords2D {
    /// Compressed 2D coordinates.
    pub coords: Vec<Coords2D>,
    /// A bijective map that maps the old x coordinate to the new compressed coordinate.
    x_old_to_new_map: BiMap<i64, i64>,
    /// A bijective map that maps the old y coordinate to the new compressed coordinate.
    y_old_to_new_map: BiMap<i64, i64>,
}

impl CompressedCoords2D {
    /// Compresses the input list of `coords`.
    #[must_use]
    pub fn from_coords(coords: &[Coords2D]) -> Self {
        let compress = |extract: fn(&Coords2D) -> i64| -> BiMap<i64, i64> {
            coords
                .iter()
                .map(extract)
                .sorted()
                .dedup()
                .enumerate()
                .map(|(new, old)| (old, new as i64))
                .collect()
        };
        let x_old_to_new_map = compress(|c| c.x);
        let y_old_to_new_map = compress(|c| c.y);

        let compressed_coords: Vec<Coords2D> = coords
            .iter()
            .map(|coord| {
                Coords2D::new(
                    *x_old_to_new_map.get_by_left(&coord.x).unwrap(),
                    *y_old_to_new_map.get_by_left(&coord.y).unwrap(),
                )
            })
            .collect();

        Self {
            coords: compressed_coords,
            x_old_to_new_map,
            y_old_to_new_map,
        }
    }

    /// Gets the largest compressed x coordinate's value.
    #[inline]
    pub fn max_x(&self) -> i64 {
        self.x_old_to_new_map.len().saturating_sub(1) as i64
    }

    /// Gets the largest compressed y coordinate's value.
    #[inline]
    pub fn max_y(&self) -> i64 {
        self.y_old_to_new_map.len().saturating_sub(1) as i64
    }

    /// Decompresses a coordinate back to the original value.
    pub fn to_original(&self, coords: &Coords2D) -> Option<Coords2D> {
        let x_option = self.x_old_to_new_map.get_by_right(&coords.x).cloned();
        let y_option = self.y_old_to_new_map.get_by_right(&coords.y).cloned();
        match (x_option, y_option) {
            (Some(x), Some(y)) => Some(Coords2D::new(x, y)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_from_coords() {
        let input = [
            Coords2D::new(20000, 30000),
            Coords2D::new(-15000, 0),
            Coords2D::new(25000, -15000),
        ];

        let compressed_coords = CompressedCoords2D::from_coords(&input);
        assert_eq!(
            &compressed_coords.coords,
            &vec![
                Coords2D::new(1, 2),
                Coords2D::new(0, 1),
                Coords2D::new(2, 0)
            ]
        );

        let input = [
            Coords2D::new(100, 100),
            Coords2D::new(100, 500),
            Coords2D::new(500, 500),
            Coords2D::new(500, 300),
            Coords2D::new(800, 300),
            Coords2D::new(800, 100),
        ];
        let compressed_coords = CompressedCoords2D::from_coords(&input);
        assert_eq!(
            &compressed_coords.coords,
            &vec![
                Coords2D::new(0, 0),
                Coords2D::new(0, 2),
                Coords2D::new(1, 2),
                Coords2D::new(1, 1),
                Coords2D::new(2, 1),
                Coords2D::new(2, 0)
            ]
        );
    }

    #[test]
    fn test_max_x_and_y() {
        let input = [
            Coords2D::new(100, 100),
            Coords2D::new(100, 500),
            Coords2D::new(500, 500),
            Coords2D::new(500, 300),
            Coords2D::new(800, 300),
            Coords2D::new(800, 100),
        ];
        let compressed_coords = CompressedCoords2D::from_coords(&input);
        assert_eq!(compressed_coords.max_x(), 2);
        assert_eq!(compressed_coords.max_y(), 2);
    }

    #[test]
    fn test_to_original() {
        let input = [
            Coords2D::new(100, 100), // (0, 0)
            Coords2D::new(100, 500), // (0, 2)
            Coords2D::new(500, 500), // (1, 2)
            Coords2D::new(500, 300), // (1, 1)
            Coords2D::new(800, 300), // (2, 1)
            Coords2D::new(800, 100), // (2, 0)
        ];
        let compressed_coords = CompressedCoords2D::from_coords(&input);

        assert_eq!(
            compressed_coords.to_original(&Coords2D::new(1, 2)),
            Some(Coords2D::new(500, 500))
        );
        assert_eq!(
            compressed_coords.to_original(&Coords2D::new(2, 1)),
            Some(Coords2D::new(800, 300))
        );
        assert_eq!(compressed_coords.to_original(&Coords2D::new(5, 10)), None);
    }
}
