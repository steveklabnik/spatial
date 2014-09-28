use std::fmt::Show;
use std::fmt;

/// A two-dimensional bounding volume for a `Quadtree` node.
pub struct Volume<T: Primitive> {
    pub min: (T, T),
    pub max: (T, T)
}

impl<T: Primitive> Volume<T> {
    /// Create a new bounding volume from two points, where both `min`
    /// and `max` are of format `(x, y)`.
    #[inline]
    pub fn new(min: (T, T), max: (T, T)) -> Volume<T> {
        Volume {
            min: min,
            max: max
        }
    }

    #[inline]
    pub fn min(&self) -> (T, T) {
        self.min
    }

    #[inline]
    pub fn max(&self) -> (T, T) {
        self.max
    }

    /// Returns `true` if `p` is inside the volume, `false` otherwise.
    #[inline]
    pub fn contains(&self, p: (T, T)) -> bool {
        let (min_x, min_y) = self.min;
        let (max_x, max_y) = self.max;
        let (p_x, p_y) = p;

        p_x >= min_x && p_x <= max_x &&
            p_y >= min_y && p_y <= max_y
    }

    /// Returns `true` if `other` intersects the volume, `false`
    /// otherwise.
    #[inline]
    pub fn intersects(&self, other: Volume<T>) -> bool {
        let (min_x, min_y) = self.min;
        let (max_x, max_y) = self.max;
        let (other_min_x, other_min_y) = other.min;
        let (other_max_x, other_max_y) = other.max;

        min_x < other_max_x && max_x > other_min_x &&
            min_y < other_max_y && max_y > other_min_y
    }
}

impl<T: Primitive + Show> Show for Volume<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (min_x, min_y) = self.min;
        let (max_x, max_y) = self.max;
        write!(f, "[[{} {}] [{} {}]]", min_x, min_y, max_x, max_y)
    }
}
