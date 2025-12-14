use std::fmt::Debug;

/// Represents a 2D coordinate.
#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub struct Coords2D {
    pub x: i64,
    pub y: i64,
}

impl Coords2D {
    #[must_use]
    #[inline]
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

impl Debug for Coords2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Coords2D({}, {})", self.x, self.y)
    }
}
