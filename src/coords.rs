use std::fmt::Debug;

/// Represents a 3D coordinate.
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Coords3D {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Coords3D {
    #[must_use]
    #[inline]
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }

    /// Computes the [Euclidean distance](https://en.wikipedia.org/wiki/Euclidean_distance)
    /// with another coordinate.
    pub fn distance(&self, other: &Self) -> f64 {
        let dist =
            (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2);
        (dist as f64).sqrt()
    }
}

impl Debug for Coords3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Coords3D({}, {}, {})", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_distance() {
        assert_eq!(
            Coords3D::new(0, 0, 0).distance(&Coords3D::new(0, 0, 0)),
            0_f64
        );
        assert_eq!(
            Coords3D::new(0, 0, 0).distance(&Coords3D::new(3, 0, 0)),
            3_f64
        );
        assert_eq!(
            Coords3D::new(0, 0, 0).distance(&Coords3D::new(0, 4, 0)),
            4_f64
        );
        assert_eq!(
            Coords3D::new(0, 0, 0).distance(&Coords3D::new(0, 0, 5)),
            5_f64
        );
        assert_eq!(
            Coords3D::new(0, 0, 0).distance(&Coords3D::new(2, 3, 6)),
            7_f64
        );
        assert_eq!(
            Coords3D::new(-1, -2, -3).distance(&Coords3D::new(2, 2, 1)),
            (9_f64 + 16_f64 + 16_f64).sqrt()
        );
        assert_eq!(
            Coords3D::new(1, 2, 3).distance(&Coords3D::new(4, 5, 6)),
            Coords3D::new(4, 5, 6).distance(&Coords3D::new(1, 2, 3))
        );
    }
}
