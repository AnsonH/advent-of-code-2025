use std::hash::Hash;

use crate::coords::Coords3D;

/// Represents an undirected line connecting two [Coords3D].
///
/// # Undirected
///
/// The line has no direction, so two [Line3D] with same pair of [Coords3D] but different order are
/// considered equal.
///
/// ```
/// use advent_of_code_2025::line::Line3D;
///
/// assert!(Line3D::new((1, 2, 3), (4, 5, 6)) == Line3D::new((4, 5, 6), (1, 2, 3)));
/// ```
#[derive(Debug, Clone, Default)]
pub struct Line3D(pub Coords3D, pub Coords3D);

impl Line3D {
    #[must_use]
    #[inline]
    pub fn new(first_coord: (i64, i64, i64), second_coord: (i64, i64, i64)) -> Self {
        Line3D(
            Coords3D::new(first_coord.0, first_coord.1, first_coord.2),
            Coords3D::new(second_coord.0, second_coord.1, second_coord.2),
        )
    }

    #[inline]
    pub fn len(&self) -> f64 {
        self.0.distance(&self.1)
    }
}

// Ensures `Line3D(A, B) == Line3D(B, A)``
impl PartialEq for Line3D {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 || self.0 == other.1 && self.1 == other.0
    }
}

impl Eq for Line3D {}

impl Hash for Line3D {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Always hash in a consistent order so that `hash(Line3D(A, B)) == hash(Line3D(B, A))`
        let (min, max) = if self.0.x < self.1.x
            || (self.0.x == self.1.x && self.0.y < self.1.y)
            || (self.0.x == self.1.x && self.0.y == self.1.y && self.0.z < self.1.z)
        {
            (self.0, self.1)
        } else {
            (self.1, self.0)
        };
        min.hash(state);
        max.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_length() {
        assert_eq!(
            Line3D::new((-1, -2, -3), (2, 2, 1)).len(),
            (9_f64 + 16_f64 + 16_f64).sqrt()
        );
        assert_eq!(
            Line3D::new((-1, -2, -3), (2, 2, 1)).len(),
            Line3D::new((2, 2, 1), (-1, -2, -3)).len(),
        );
    }

    #[test]
    fn test_equality() {
        assert!(Line3D::new((1, 2, 3), (4, 5, 6)) == Line3D::new((1, 2, 3), (4, 5, 6)));
        assert!(Line3D::new((1, 2, 3), (4, 5, 6)) == Line3D::new((4, 5, 6), (1, 2, 3)));
        assert!(Line3D::new((1, 2, 3), (4, 5, 6)) != Line3D::new((0, 2, 3), (4, 5, 6)));
    }

    #[test]
    fn test_hash() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::Hasher;

        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();

        // Line3D(A, B) should have same hash as Line3D(B, A)
        Line3D::new((1, 2, 3), (4, 5, 6)).hash(&mut hasher1);
        Line3D::new((4, 5, 6), (1, 2, 3)).hash(&mut hasher2);
        assert_eq!(
            hasher1.finish(),
            hasher2.finish(),
            "Lines with swapped endpoints should have the same hash"
        );

        // Different lines should (likely) have different hashes
        let mut hasher3 = DefaultHasher::new();
        Line3D::new((0, 0, 0), (1, 1, 1)).hash(&mut hasher3);
        assert_ne!(
            hasher1.finish(),
            hasher3.finish(),
            "Different lines should have different hashes"
        );

        // Test with HashSet to ensure practical usage works
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(Line3D::new((1, 2, 3), (4, 5, 6)));

        // Should recognize as duplicate due to equal hash and equality
        assert!(set.contains(&Line3D::new((4, 5, 6), (1, 2, 3))));
        assert_eq!(set.len(), 1);
    }
}
