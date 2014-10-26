use std::fmt::Show;
use std::fmt;

/// A two-dimensional bounding volume for a `Quadtree` node.
pub struct Volume<T: Primitive> {
    pub min: [T, ..2],
    pub max: [T, ..2]
}

impl<T: Primitive> Volume<T> {
    /// Create a new bounding volume from two points, where both `min`
    /// and `max` are of format `[x, y]`.
    #[inline]
    pub fn new(min: [T, ..2], max: [T, ..2]) -> Volume<T> {
        Volume {
            min: min,
            max: max
        }
    }
    
    #[inline]
    pub fn min(&self) -> [T, ..2] {
        self.min
    }
    
    #[inline]
    pub fn max(&self) -> [T, ..2] {
        self.max
    }
    
    /// Returns `true` if `p` is inside the volume, `false` otherwise.
    #[inline]
    pub fn contains(&self, p: &[T, ..2]) -> bool {
        let min = self.min;
        let max = self.max;
        
        p[0] >= min[0] && p[0] <= max[0] &&
            p[1] >= min[1] && p[1] <= max[1]
    }
    
    /// Returns `true` if `other` intersects the volume, `false`
    /// otherwise.
    #[inline]
    pub fn intersects(&self, other: &Volume<T>) -> bool {
        let min = self.min;
        let max = self.max;
        min[0] < other.max[0] && max[0] > other.min[0] &&
            min[1] < other.max[1] && max[1] > other.min[1]
    }
}

impl<T: Primitive + Show> Show for Volume<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let min = self.min;
        let max = self.max;
        write!(f, "[[{} {}] [{} {}]]", min[0], min[1], max[0], max[1])
    }
}
